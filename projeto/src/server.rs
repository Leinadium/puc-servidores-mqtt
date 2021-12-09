extern crate paho_mqtt as mqtt;
extern crate serde_json as json;

use std::{
    env,
    process,
    thread,
    borrow::Borrow,
};

use json::{Value};
use api::{Operacao};

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
        "leitura" => {},
        "insercao" => {},
        "morte" => {},
        "nascimento" => {},
        "atualizacao" => {},
        _ => {}
    }

    Operacao::Invalida
}

/// Loop de execucao principal do programa.
/// Não precisa ser executado em uma thread.
fn main_loop(n_server: &i32, _n_total: &i32) {
    let nome_id = format!("{}{}", api::SERVER_NAME, n_server);
    let topico = api::TOPICO_REQS;
    let conexao = api::conectar(&nome_id, topico);

    // como o servidor só envia msgs de acordo com o que recebe
    // entao o loop principal é em função do que recebe
    for msg in conexao.rx.iter() {
        if let Some(msg) = msg {
            // convertendo para json
            let texto = msg.payload_str().borrow();
            if let Some(v) = json::from_str(texto) {
                trata(v);
            }
        }
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