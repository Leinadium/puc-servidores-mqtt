extern crate paho_mqtt as mqtt;
extern crate serde_json as json;

mod api;

use std::{
    env,
    process,
    borrow::Borrow,
    time::Duration,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    // argumentos:
    // client.exe id insere chave valor
    // client.exe id consulta chave

    if args.len() != 4 && args.len() != 5 {
        println!("invalid arguments");
        println!("usage:\n  {} id insere [chave] [valor]\n  {} id consulta [chave]",
                 args.get(0).unwrap(), args.get(0).unwrap());
        process::exit(0);
    }


    let op: &String = args.get(2).expect("invalid operation argument");
    let id: i64 = args.get(1)
        .expect("invalid id argument")
        .parse::<i64>()
        .expect("id is not a number");
    let id_string = format!("inf1406-client-{}", id);
    let chave: &String = args.get(3).expect("invalid key argument");

    let conexao = api::conectar(&id_string, api::TOPICO_NONE);
    let topicoresp = format!("inf1406-client-r-{}", id);

    // INSERCAO
    if op.starts_with("insere") {
        let valor:&String = args.get(4).expect("invalid value argument");
        let msg = format!(r#"{{
            "tipomsg": "insercao", "chave": "{}", "novovalor": "{}",
            "topico-resp": "{}", "idpedido": {}
            }}"#, chave, valor, topicoresp, id);
        api::enviar(&conexao, &msg, api::TOPICO_REQS);

    } else if op.starts_with("consulta") {
        api::adicionar_topico(&conexao, &topicoresp);
        let msg = format!(r#"{{
            "tipomsg": "leitura", "chave": "{}",
            "topico-resp": "{}", "idpedido": {}
            }}"#, chave, topicoresp, id);

        api::enviar(&conexao, &msg, api::TOPICO_REQS);

        let cb = conexao.rx
            .recv_timeout(Duration::from_secs(10))
            .expect("nenhuma mensagem chegou em 10 segundos :(");

        if let Some(cb_msg) = cb {
            let cow = cb_msg.payload_str();
            let texto: &str = cow.borrow();
            println!("Recebi de volta: {}", texto);
        }
    }
    conexao.cli.disconnect(None).expect("erro fechando client");
}