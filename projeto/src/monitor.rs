extern crate paho_mqtt as mqtt;
extern crate serde_json as json;

use std::{
    env,
    process,
    thread,
    borrow::Borrow,
    collections::HashMap,
    time::{Duration}
};
use api::{
    Conexao
};
use std::thread::sleep;

#[allow(dead_code)]
mod api;


fn check_heartbeat(conexao: &Conexao, hashmap: &mut HashMap<i64, Duration>, now: Duration) {
    // verifica se há mensagens de heartbeat sem bloquear, para atualizar o hashmap
    let msg = conexao.rx.try_recv();

    let message = match msg {
        Ok(t) => t,
        Err(_error) => return
    };

    // converte a msg para json
    if let Some(message) = message {
        let cow = message.payload_str();
        let texto: &str = cow.borrow();

        // para controle
        println!("recebi: {}", texto);
        if texto.starts_with("assassinato") {
            conexao.cli.disconnect(None).unwrap();
            process::exit(0);
        }

        if let Ok(v) = json::from_str(texto) {
            // insere/atualiza o "último momento visto" do server no hashmap
            let id_serv = api::extrair_int(&v, "idServ");
            if id_serv != -1 {
                hashmap.insert(id_serv, now);
            }
        }
    }
}

fn verify_crashed_servers(hashmap: &mut HashMap<i64, Duration>, conexao: &Conexao, now: Duration) {
    // verifica falhas no heartbeat e envia mensagem para o tópico reqs caso haja
    for (id_serv, &vistoem) in hashmap.clone().iter() {
        // verifica se ultrapassou o tempo de timeout
        if now - vistoem > api::HEARTBEAT_TIMEOUT {
            // setup para enviar a msg
            let fail_hb_topico = api::TOPICO_REQS;
            // let fail_hb_conexao = api::conectar(&nome_id, fail_hb_topico);
            let fail_hb_text = format!(r#"{{"tipomsg": "falhaserv", "idserv": {}, "vistoem": "{}"}}"#, id_serv, vistoem.as_millis() as u64);

            api::enviar(&conexao, &fail_hb_text, fail_hb_topico);
            hashmap.remove(id_serv);

            // let msg = mqtt::Message::new(fail_hb_topico, fail_hb_text.clone(), api::QOS);
            // envio da msg
            // let tok = fail_hb_conexao.cli.publish(msg);
            // if let Err(e) = tok {
            //    println!("Monitor: Error sending server {} crash message: {:?}", id_serv, e);
            // } else {
            //     hashmap.remove(id_serv);
            // }
        }
    }
}

fn monitor_loop() {
    // SETUP
    let nome_id = format!("{}", api::MONITOR_NAME);
    let topico = api::TOPICO_MON;
    let conexao = api::conectar(&nome_id, topico);
    let mut hashmap: HashMap<i64, Duration> = HashMap::new();

    // LOOP
    // o monitor fica em loop esperando por heartbeats e verificando
    // se algum servidor que já enviou heartbeat a ele antes crashou
    loop {
        let now = api::get_now_as_duration();
        check_heartbeat(&conexao, &mut hashmap, now);
        verify_crashed_servers(&mut hashmap, &conexao, now);
        sleep(Duration::from_millis(1));
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        println!("Invalid arguments!");
        println!("Usage: {}", args.get(0).unwrap());
        process::exit(0x01);
    }

    println!("starting monitor");

    // iniciando o heartbeat
    let handle = thread::spawn(move || {
        monitor_loop();
    });

    handle.join().unwrap();
}