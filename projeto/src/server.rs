extern crate paho_mqtt as mqtt;
extern crate serde_json as json;

use std::{
    env,
    process,
    thread,
    borrow::Borrow,
    collections::HashMap,
    time::{Duration},
    fmt::Write,
};

use json::{Value};

use api::{
    ClientInsercao, ClientLeitura, MonitorMorte,
    ServidorAtualizacao, ServidorNascimento, ControleAssassinato,
    Operacao, Conexao,
};
#[allow(dead_code)]
mod api;


/// Envia um heartbeat e dorme.
/// Para ser usada em uma thread.
fn heartbeat_loop(n_server: i64) {

    let nome_id = format!("{}{}", api::SERVER_HEARTBEAT_NAME, n_server);
    let topico = api::TOPICO_MON;

    let conexao = api::conectar(&nome_id, topico);

    let text = format!(r#"{{"idServ": {}}}"#, n_server);
    loop {
        let msg = mqtt::Message::new(topico, text.clone(), api::QOS);
        let tok = conexao.cli.publish(msg);
        if let Err(e) = tok {
            println!("{}: Error sending hearbeat: {:?}", n_server, e);
        }
        thread::sleep(api::HEARTBEAT_SLEEP);
    }
}


/// Retorna o id do servidor que deve responder para essa chave
fn de_quem_eh(n_total: &i64, chave: &String) -> i64 {
    let mut soma = 0;
    for byte in chave.as_bytes() {
        let mut hexs = String::new();
        write!(&mut hexs, "{:x}", &byte).expect("unable to write");
        let byte_int = i64::from_str_radix(hexs.as_str(), 16).expect("invalid hexstring");
        soma += byte_int;
    }
    soma % n_total
}


/// Diz se eu sou o substituto do servidor
fn sou_sub(n_server: &i64, n_total: &i64, n_server_morto: &i64) -> bool {
    (n_server_morto + 1) % n_total == *n_server
}

/// Atualiza a lista, e retira as operações muito velhas
fn atualiza_log(lista: &mut Vec<Operacao>, new_op: Operacao)  {
    let mut i = -1;
    let atual = api::get_now_as_duration();

    for (ind, op) in lista.iter().enumerate() {
        let t = match op {
            Operacao::Leitura(x) => x.tempo,
            Operacao::Insercao(x) => x.tempo,
            Operacao::Morte(x) => x.tempo,
            Operacao::Nascimento(x) => x.tempo,
            Operacao::Atualizacao(x) => x.tempo,
            _ => atual
        };

        if atual - t < api::OPERACAO_TIMEOUT {
            i = ind as i32;
            break ;
        }
    }

    // retirando
    if i != -1 {
        while i > 0 {
            lista.remove(0);
            i -= 1;
        }
    }
    lista.push(new_op);
}


/// Trata uma mensagem recebida de um json
/// retorna o que deve ser feito.
fn parse(v: Value) -> Operacao {
    // fazendo o parsing:
    let tipomsg = match &v["tipomsg"] {
        Value::String(s) => s.clone(),        // tipomsg valida
        _ => {"".to_string()}                         // tipomsg invalida
    };

    match tipomsg.as_str() {
        "leitura" => Operacao::Leitura(ClientLeitura {
            chave: api::extrair_string(&v, "chave"),
            topicoresp: api::extrair_string(&v, "topico-resp"),
            idpedido: api::extrair_int(&v, "idpedido"),
            tempo: api::get_now_as_duration(),
        }),
        "insercao" => Operacao::Insercao(ClientInsercao {
            chave: api::extrair_string(&v, "chave"),
            novovalor: api::extrair_string(&v, "novovalor"),
            topicoresp: api::extrair_string(&v, "topico-resp"),
            idpedido: api::extrair_int(&v, "idpedido"),
            tempo: api::get_now_as_duration(),
        }),
        "falhaserv" => Operacao::Morte(MonitorMorte {
            idserv: api::extrair_int(&v, "idserv"),
            vistoem: api::extrair_tempo(&v, "vistoem"),
            tempo: api::get_now_as_duration(),
        }),
        "novoserv" => Operacao::Nascimento(ServidorNascimento {
            idserv: api::extrair_int(&v, "idserv"),
            topicoresp: api::extrair_string(&v, "topico-resp"),
            tempo: api::get_now_as_duration(),
        }),
        "atualizacao" => Operacao::Atualizacao(ServidorAtualizacao {
            hashmap: api::extrair_hashmap(&v, "conteudo"),
            tempo: api::get_now_as_duration(),
        }),
        "assassinato" => Operacao::Assassinato(ControleAssassinato {
            idserv: api::extrair_int(&v, "idserv"),
        }),
        _ => Operacao::Invalida
    }
}


/// Envia a mensagem de nascimento
fn bom_dia(conexao: &Conexao, meuid: &i64, topico_resp: &str) {
    let msg = format!(r#"{{"tipomsg": "novoserv", "idserv": {}, "topico-resp": "{}"}}"#,
                      meuid, topico_resp);
    api::enviar(conexao, &msg, api::TOPICO_REQS);
}


/// Atualiza o hashmap interno com a resposta que veio
fn se_atualiza(v: &HashMap<String, String>, old_hashmap: &mut HashMap<String, String>){
    for (key, value) in v.iter() {
        old_hashmap.insert(key.clone(), value.clone());
    }
}


/// Envia a atualização do hashmap interno para o servidor recem nascido
fn manda_atualizacao(conexao: &Conexao, h: &HashMap<String, String>, topico: &str) {
    let mut msg: String = r#"{"tipomsg": "atualizacao", "conteudo": {"#.to_owned();
    let mut first = true;

    for (key, value) in h.iter() {
        let s: String;
        if first {
            s = format!(r#""{}": "{}""#, key, value);
            first = false;
        } else {
            s = format!(r#", "{}": "{}""#, key, value);
        }
        msg.push_str(&s);
    }
    msg.push_str("}}");
    println!("enviando para o recem nascido: {}", msg);
    api::enviar(conexao, &msg, topico);
}


/// Envia as mensagens de leitura no lugar do morto
fn responde_pelo_morto(conexao: &Conexao, log: &Vec<Operacao>, hashmap: &HashMap<String, String>,
                       vistoem: &Duration, n_server_morto: &i64, n_total: &i64, n_server: &i64) {

    for op in log.iter() {
        match op {
            Operacao::Leitura(cl) => {
                if vistoem < &cl.tempo && de_quem_eh(n_total, &cl.chave) == *n_server_morto {
                    let content = match hashmap.get(&cl.chave) {
                        None => {"nao existe".to_string()}
                        Some(c) => {c.clone()}
                    };
                    let payload = format!(r#"{{ "idserv": {}, "resposta": "{}" }}"#,
                        n_server, content);
                    println!("  enviando msg pelo morto: {}", payload);
                    api::enviar(&conexao, &payload, &cl.topicoresp.as_str())
                }
            },
            _ => {}
        }
    }
}


/// Loop de execucao principal do programa.
/// Não precisa ser executado em uma thread.
fn main_loop(n_server: &i64, n_total: &i64, has_birth: bool) {
    // SETUP
    let nome_id = format!("{}{}c", api::SERVER_NAME, n_server);
    let topico = api::TOPICO_REQS;
    let topico_recuperacao = format!("{}{}", api::TOPICO_REC, &n_server);
    let conexao = api::conectar(&nome_id, topico);
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let mut lista_log: Vec<Operacao> = Vec::new();

    let mut bool_sub: bool = false;

    // pede atualizacao se for um servidor nascido dos mortos
    if has_birth {
        println!("Enviando um bom dia ao grupo");
        api::adicionar_topico(&conexao, &topico_recuperacao);
        bom_dia(&conexao, &n_server, &topico_recuperacao);
    }

    // LOOP
    // como o servidor só envia msgs de acordo com o que recebe
    // entao o loop principal é em função do que recebe
    for msg in conexao.rx.iter() {
        let op: Operacao;
        let mut devo_salvar = true;

        // convertendo para json
        if let Some(msg) = msg {
            let cow = msg.payload_str();
            let texto = cow.borrow();
            println!();
            println!("recebi alguma coisa: {}", texto);
            if let Ok(v) = json::from_str(texto) {
                op = parse(v);
                println!("  comando json valido")
            } else {
                op = Operacao::Invalida;
                println!("  comando invalido")
            }
        } else {
            op = Operacao::Invalida;
        }

        match &op {
            // TRATANDO UMA INSERCAO
            Operacao::Insercao(ci) => {
                println!("  tipo: insercao (chave: {})", ci.chave);
                hashmap.insert(ci.chave.clone(), ci.novovalor.clone());
            },
            // TRATANDO UMA LEITURA
            Operacao::Leitura(cl) => {
                println!("  tipo: leitura");
                let n = de_quem_eh(n_total, &cl.chave);
                println!("  responsavel pela resposta: {}", n);
                if n == *n_server || (bool_sub && sou_sub(&n_server, &n_total, &n)) {
                    println!("  eu sou o responsavel");
                    let content = match hashmap.get(&cl.chave) {
                        None => {"nao existe".to_string()}
                        Some(c) => {c.clone()}
                    };
                    let payload = format!(r#"{{ "idserv": {}, "resposta": "{}" }}"#,
                                          n_server, content);
                    println!("  enviando resposta: {}", payload);
                    api::enviar(&conexao, &payload, &cl.topicoresp.as_str());
                }
            },
            // TRATANDO UM SERVIDOR MORRENDO
            Operacao::Morte(mm) => {
                println!("  tipo: morte");
                if sou_sub(&n_server, &n_total, &mm.idserv) {
                    println!("  sou o responsavel pelo defunto");
                    responde_pelo_morto(&conexao, &lista_log, &hashmap, &mm.vistoem, &mm.idserv, &n_total, &n_server);
                    bool_sub = true;
                }
            },
            // TRATANDO UM SERVIDOR NASCENDO
            Operacao::Nascimento(sn) => {
                println!("  tipo: nascimento");
                if bool_sub && sou_sub(&n_server, &n_total, &sn.idserv) {
                    println!("  o morto ressuscitou, enviando o que ele perdeu");
                    manda_atualizacao(&conexao, &hashmap, &sn.topicoresp.as_str());
                    bool_sub = false;
                }
            },
            // TRATANDO A MINHA ATUALIZACAO
            Operacao::Atualizacao(sa) => {
                println!("  tipo: atualizacao");
                api::remover_topico(&conexao, &topico_recuperacao);
                println!("  pegando conteudo que perdi enquanto estava morto");
                se_atualiza(&sa.hashmap, &mut hashmap);
            },
            // (PARA CONTROLE) TRATANDO A MINHA MORTE
            Operacao::Assassinato(ca) => {
                println!("  tipo: assassinato");
                if ca.idserv == *n_server {
                    println!("  chegou a minha hora. Fechando");
                    conexao.cli.disconnect(None).expect("error disconnecting");
                    process::exit(0);
                }
            }

            _ => { devo_salvar = false; }     // operacao invalida
        }
        // colocando na lista
        if devo_salvar {
            atualiza_log(&mut lista_log, op);
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        println!("Invalid arguments!");
        println!("Usage: {} n_server n_total [-b]", args.get(0).unwrap());
        println!("n_server: my server id (n_server < n_total)");
        println!("n_total: amount of servers running");
        println!("-b: if the server has to announce its birth");
        process::exit(0x01);
    }

    let n_server: i64 = args.get(1)
        .expect("invalid n_server argument")
        .parse::<i64>()
        .expect("n_server is not a number");

    let n_total: i64 = args.get(2)
        .expect("invalid n_total argument")
        .parse::<i64>()
        .expect("n_total is not a number");

    let has_birth: bool = match args.get(3) {
        None => false,
        Some(c) => c.starts_with("-b")
    };


    println!("starting server #{} (server {}/{})", n_server, n_server + 1, n_total);

    // iniciando o heartbeat
    let n_server1 = n_server.clone();
   thread::spawn(move || {
        heartbeat_loop(n_server1);
    });

    main_loop(&n_server, &n_total, has_birth);
}