use std::{
    env,
    process,
    thread,
};

mod api;


extern crate paho_mqtt as mqtt;
extern crate serde_json as json;


/// Envia um heartbeat e dorme.
/// Para ser usada em uma thread.
fn loop_heartbeat(n_server: i32) {

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
        loop_heartbeat(n_server1);
    });

    handle.join().unwrap();
}