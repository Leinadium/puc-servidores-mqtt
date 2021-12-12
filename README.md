# puc-servidores-mqtt
Repositório para o trabalho 4 de INF1406 Programação Distribuída da PUC-Rio 

# Setup

## Requerimentos:

* [Rust](https://www.rust-lang.org/tools/install)

* [Mosquitto](https://mosquitto.org/download/)

# Execução

Todos os comandos abaixo são para windows.

Primeiro, certifique-se que o Rust está com o build atualizado:

```bash
cd projeto
cargo build
```

É possível que a biblioteca do `paho-mqtt` peça a instalação de algumas ferramentas a mais, como o `CMake`,
que pode ser instalado através do [chocolatey](https://chocolatey.org/install), fazendo ```choco install cmake```

Certifique-se também de estar com o `mosquitto` rodando na porta 1883.

## Servidor

```bash
cd projeto
cargo build --bin server
target\debug\server.exe [ID_SERVER] [N_TOTAL] 
```

em que ID_SERVER é o id do servidor, e N_TOTAL é a quantidade de servidores na rede.

## Client

```bash
cd projeto
cargo build --bin client
target\debug\client.exe [ID] [insere/consulta] [chave] ([valor])

# exemplos de execucao de client
client.exe 123 insere bom dia

client.exe 123 consulta bom
> Recebi de volta: { "idserv": 0, "resposta": "dia" }
 
```




## Exemplo do mqtt

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