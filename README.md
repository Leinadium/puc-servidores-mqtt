# puc-servidores-mqtt
Repositório para o trabalho 4 de INF1406 Programação Distribuída da PUC-Rio 

# Setup

## Requerimentos:

* [Rust](https://www.rust-lang.org/tools/install)

* [Mosquitto](https://mosquitto.org/download/)

# Execução

Todos os comandos abaixo são para windows.

## Exemplo

```bash
# buildando
cd exemplo 
cargo build

# suba o mosquitto (em outro terminal)
path\to\mosquitto\mosquitto.exe

# subindo um subscriber (em outro terminal)
cargo run --bin sub

# subindo um publisher (em outro terminal)
cargo run --bin pub
```