use std::{
    env,
    process,
    thread
};

mod api;


extern crate paho_mqtt as mqtt;


fn loop_hearbeat(n_server: i32) {
    // opções de conexão
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(api::TOPICO_MON)
        .client_id(format!("{}{}", MY_NAME, n_server))
        .finalize();

    // cria o client
    let mut cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating heartbeat client in thread: {:?}", err);
        process::exit(1);
    });
}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid arguments!");
        println!("Usage: {} n_server n_total", args.get(0).unwrap());
        process::exit(0x01);
    }

    let n_server = args.get(1)
        .parse::<i32>()
        .expect("n_server is not a number");

    let n_total = args.get(1)
        .parse::<i32>()
        .expect("n_total is not a number");



    println!("Hello, World!");
}