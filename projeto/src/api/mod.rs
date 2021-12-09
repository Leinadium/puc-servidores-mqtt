#[doc(hidden)]

use std::{
    process,
    time::Duration,
    sync::mpsc::Receiver
};
use mqtt::{Client, Message};

extern crate paho_mqtt as mqtt;

const END_BROKER:&str = "tcp://localhost:1883";
pub const TOPICO_REQS:&str = "inf1406-reqs";
pub const TOPICO_MON:&str = "inf1406-mon";
pub const TOPICO_NONE:&str = "none";

pub const SERVER_NAME:&str = "inf1406-server-";
pub const SERVER_HEARTBEAT_NAME: &str = "inf1406-server-h-";
pub const MONITOR_NAME:&str = "inf1406-monitor";
pub const MONITOR_HEARTBEAT_NAME:&str = "inf1406-monitor-h";

pub const QOS:i32 = 1;
pub const HEARTBEAT_SLEEP:Duration = Duration::from_secs(5);
pub const HEARTBEAT_TIMEOUT:Duration = Duration::from_secs(10);

/// Contem o canal de chegada, e o cliente para saida
/// é a estrutura retornada pela função ```conectar```
pub struct Conexao {
    pub rx: Receiver<Option<Message>>,
    pub cli: Client
}

/// Faz uma conexão mqtt no topico passado, usando o nome_id fornecido
///
/// # Arguments
/// * `nome_id` - String contendo o nome identificador dessa conexão
/// * `topico` - str contendo o nome do tópico a ser conectado.
///             Se topico for TOPICO_NONE, o client não estará recebendo
///             mensagem de nenhum tópico.
pub fn conectar(nome_id: &String, topico: &str) -> Conexao {
    // cria as opcoes
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(END_BROKER)
        .client_id(nome_id)
        .finalize();

    // cria o client de conexao
    let mut cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating {} client conection: {:?}", nome_id, err);
        process::exit(1);
    });

    // inicia o consumo
    let rx = cli.start_consuming();

    // define as opcoes das conexoes
    let last_will = mqtt::MessageBuilder::new()
        .topic(topico)
        .payload(format!("{} has lost connection", nome_id))
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

    // se inscrevendo no topico
    if topico != TOPICO_NONE {
        if let Err(e) = cli.subscribe(topico, 1) {  // QoS1 -> At least once
            println!("unable to subscribe: {:?}", e);
            process::exit(1);
        }
    }

    let ret = Conexao { rx, cli };
    ret
}
