#[doc(hidden)]

extern crate paho_mqtt as mqtt;
extern crate serde_json as json;

use std::{
    process,
    time::Duration,
    sync::mpsc::Receiver,
    time::SystemTime
};

use mqtt::{Client, Message};

use json::{Value};


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
pub const OPERACAO_TIMEOUT:Duration = Duration::from_secs(100);

/// json para um client querendo ler algum dado
pub struct ClientLeitura {
    pub chave: String,
    pub topicoresp: String,
    pub idpedido: i64,
    pub tempo: Systemtime,
}

/// json para um client querendo inserir algum dado
pub struct ClientInsercao {
    pub chave: String,
    pub novovalor: String,
    pub topicoresp: String,
    pub idpedido: i64,
    pub tempo: Systemtime,
}

/// json para um monitor avisando da morte de algum servidor
pub struct MonitorMorte {
    pub idserv: i64,
    pub vistoem: String,
    pub tempo: Systemtime,
}

/// json para um servidor avisando que nasceu
pub struct ServidorNascimento {
    pub topicoresp: String,
    pub tempo: Systemtime,
}

/// json para um servidor fornecendo atualizacao para outro
pub struct ServidorAtualizacao {
    // TODO...
    pub todo: String,
    pub tempo: Systemtime
}

pub enum Operacao {
    Leitura(ClientLeitura),
    Insercao(ClientInsercao),
    Morte(MonitorMorte),
    Nascimento(ServidorNascimento),
    Atualizacao(ServidorAtualizacao),
    Invalida,
}

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


pub fn enviar(conexao: &Conexao, texto: &String, topico: &str) {
    let msg = Message::new(topico, texto.clone(), QOS);
    let tok = conexao.cli.publish(msg);
    if let Err(e) = tok {
        println!("Erro sending msg in {}: {:?}", topico, e);
    }
}


/// Extrai a string da chave do dicionario
/// Retorna "" se não existir aquela chave, ou não for string
///
/// # Exemplo:
///     v -> { "bom": "dia" }
///     extrair_string(v, [&str] "bom") -> [String] "dia"
///     extrair_string(v, [&str] "xxx") -> [String] ""
pub fn extrair_string(v: &Value, key: &str) -> String {
    match &v[key] {
        Value::String(s) => s.clone(),
        _ => "".to_string()
    }
}

/// Extrai um i64 da chave do dicionario
/// Retorna -1 se não existir aquela chave, ou não for i64
///
/// # Exemplo:
///     v -> { "meuid": 123 }
///     extrair_string(v, [&str] "meuid") -> [i64] 123
///     extrair_string(v, [&str] "xxx") -> [i64] -1
pub fn extrair_int(v: &Value, key: &str) -> i64 {
    match &v[key] {
        Value::Number(n) => n.clone().as_i64().unwrap_or_else(-1),
        _ => -1
    }
}