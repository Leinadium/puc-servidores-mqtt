extern crate paho_mqtt as mqtt;
extern crate serde_json as json;

use std::{
    env,
    process,
    thread,
    borrow::Borrow,
    collections::HashMap,
    time::{Duration, SystemTime}
};
use std::alloc::System;

use json::{Value};
use mqtt::Message;

use api::{
    ClientInsercao, ClientLeitura, MonitorMorte, ServidorAtualizacao, ServidorNascimento,
    Operacao,
    Conexao
};
mod api;


/// Envia um heartbeat e dorme.
/// Para ser usada em uma thread.
fn heartbeat_loop(n_server: i32) {

    let nome_id = format!("{}{}", api::SERVER_HEARTBEAT_NAME, n_server);
    let topico = api::TOPICO_MON;

    let conexao = api::conectar(&nome_id, topico);

    let text = format!("{{idServ: {}}}", n_server);
    loop {
        let msg = mqtt::Message::new(topico, text.clone(), api::QOS);
        let tok = conexao.cli.publish(msg);
        if let Err(e) = tok {
            println!("{}: Error sending hearbeat: {:?}", n_server, e);
        }
        thread::sleep(api::HEARTBEAT_SLEEP);
    }
}


/// Diz se a operação com a chave deve ser feita nesse servidor
fn eh_comigo(n_server: &i32, n_total: &i32, chave: &String) -> bool {
    let mut soma = 0;
    for byte in chave.as_bytes() {
        let mut hexs = String::new();
        write!(&mut hexs, "{:x}", &byte).expect("unable to write");
        let byte_int = i32::from_str_radix(hexs.as_str(), 16).expect("invalid hexstring");
        soma += byte_int;
    }
    soma % n_total == n_server
}

/// Atualiza a lista, e retira as operações muito velhas
fn atualiza(lista: &mut Vec<Operacao>, new_op: Operacao)  {
    let mut i = -1;
    let atual = SystemTime::now();

    for (ind, op) in lista.iter().enumerate() {
        let t = match op {
            Operacao::Leitura(x)   |
            Operacao::Insercao(x) |
            Operacao::Morte(x)    |
            Operacao::Nascimento(x)  |
            Operacao::Atualizacao(x) => x.tempo,
            _ => atual
        };

        if atual.duration_since(t).unwrap_or_else(api::OPERACAO_TIMEOUT)
            < api::OPERACAO_TIMEOUT {
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
fn trata(v: Value) -> Operacao {
    // fazendo o parsing:
    let tipomsg = match &v["tipomsg"] {
        Value::String(s) => s.clone(),        // tipomsg valida
        _ => {"".to_string()}                         // tipomsg invalida
    };

    // TODO
    match tipomsg.as_str() {
        "leitura" => Operacao::Leitura(ClientLeitura {
            chave: api::extrair_string(&v, "chave"),
            topicoresp: api::extrair_string(&v, "topico-resp"),
            idpedido: api::extrair_int(&v, "idpedido"),
            tempo: SystemTime::now(),
        }),
        "insercao" => Operacao::Insercao(ClientInsercao {
            chave: api::extrair_string(&v, "chave"),
            novovalor: api::extrair_string(&v, "novovalor"),
            topicoresp: api::extrair_string(&v, "topico-resp"),
            idpedido: api::extrair_int(&v, "idpedido"),
            tempo: SystemTime::now(),
        }),
        "morte" => Operacao::Morte(MonitorMorte {
            idserv: api::extrair_int(&v, "idserv"),
            vistoem: api::extrair_string(&v, "vistoem"),
            tempo: SystemTime::now(),
        }),
        "nascimento" => Operacao::Nascimento(ServidorNascimento {
            topicoresp: api::extrair_string(&v, "topico-resp"),
            tempo: SystemTime::now(),
        }),
        "atualizacao" => Operacao::Atualizacao(ServidorAtualizacao {
            todo: "TODO".to_string(),
            tempo: SystemTime::now(),
        }),
        _ => Operacao::Invalida
    }
}

/// Loop de execucao principal do programa.
/// Não precisa ser executado em uma thread.
fn main_loop(n_server: &i32, _n_total: &i32) {
    // SETUP
    let nome_id = format!("{}{}", api::SERVER_NAME, n_server);
    let topico = api::TOPICO_REQS;
    let conexao = api::conectar(&nome_id, topico);
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let mut lista_log: Vec<Operacao> = Vec::new();

    // LOOP
    // como o servidor só envia msgs de acordo com o que recebe
    // entao o loop principal é em função do que recebe
    for msg in conexao.rx.iter() {
        let mut op: Operacao;

        // convertendo para json
        if let Some(msg) = msg {
            let texto = msg.payload_str().borrow();
            if let Some(v) = json::from_str(texto) {
                op = trata(v);
            } else {
                op = Operacao::Invalida;
            }
        } else {
            op = Operacao::Invalida;
        }

        // TODO: tratando todas operações
        match &op {
            // TRATANDO UMA INSERCAO
            Operacao::Insercao(ci) => {
                if eh_comigo(n_server, n_total, &ci.chave) {
                    hashmap.insert(ci.chave.clone(), ci.novovalor.clone());
                }
            },
            // TRATANDO UMA LEITURA
            Operacao::Leitura(cl) => {
                if eh_comigo(n_server, n_total, &ci.chave) {
                    let content = hashmap.get(&cl.chave)
                                                .unwrap_or_else("nao existe");
                    api::enviar(&conexao, content, &cl.topicoresp.to_string());
                }
            },
            // TRATANDO UM SERVIDOR MORRENDO
            Operacao::Morte(_mm) => {
                // TODO
            },
            Operacao::Nascimento(_sn) => {
                // TODO
            },
            // TRATANDO UM UPDATE
            Operacao::Atualizacao(_sa) => {
                // TODO
            }
            _ => {}     // operacao invalida
        }
        // colocando na lista
        atualiza(&mut lista_log, op);
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid arguments!");
        println!("Usage: {} n_server n_total", args.get(0).unwrap());
        process::exit(0x01);
    }

    let n_server: i32 = args.get(1)
        .unwrap()
        .parse::<i32>()
        .expect("n_server is not a number");

    let n_total: i32 = args.get(2)
        .unwrap()
        .parse::<i32>()
        .expect("n_total is not a number");

    println!("starting server #{}/{}", n_server, n_total);

    // iniciando o heartbeat
    let n_server1 = n_server.clone();
    let handle = thread::spawn(move || {
        heartbeat_loop(n_server1);
    });

    handle.join().unwrap();
}