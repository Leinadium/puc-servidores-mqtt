use std::{
    process,
    time::Duration
};

extern crate paho_mqtt as mqtt;

pub const END_BROKER:&str = "tcp://localhost:1883";
pub const TOPICO_REQS:&str = "inf1406-reqs";
pub const TOPICO_MON:&str = "inf1406-mon";

pub const SERVER_NAME:&str = "inf1406-server-";
pub const MONITOR_NAME:&str = "inf1406-monitor";

pub fn conectar(nome_id: &str, topico: &str) {
    // cria as opcoes
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(END_BROKER)
        .client_id(nome_id)
        .finalize();

    // cria o client de conexao
    let mut cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating {} client conection in reqs: {:?}", nome_id, err);
        process::exit(1);
    });

    // inicia o consumo
    cli.start_consuming();

    // define as opcoes das conexoes
    let last_will = mqtt::MessageBuilder::new()
        .topic(TOPICO_REQS)
        .payload("{} has lost connection on reqs")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .will_message(last_will)
        .finalize();

    // connect
    if let Err(e) = cli.connect(conn_opts) {
        println!("unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // se inscrevendo em req
    if let Err(e) = cli.subscribe(topico, 1) {  // QoS1 -> At least once
        println!("unable to subscribe: {:?}", e);
        process::exit(1);
    }
}
