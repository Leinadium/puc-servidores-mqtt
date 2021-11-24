# puc-servidores-mqtt
Repositório para o trabalho 4 de INF1406 Programação Distribuída da PUC-Rio 

# Setup

## Requerimentos:

* [Rust](https://www.rust-lang.org/tools/install)

* [Mosquitto](https://mosquitto.org/download/)

## Windows

Para o windows, a biblioteca *Pahu* pede que exista as seguintes ferramentas:

* [OpenSSL](https://www.openssl.org/) -> [Windows binary (use a versão completa, não a Light)](https://slproweb.com/products/Win32OpenSSL.html)

* [CMake](https://cmake.org/download/) -> Também pode ser instalado com as Build Tools do Visual Studio, ou [chocolatey](https://chocolatey.org/install)

# Execução

Todos os comandos abaixo são para windows.

## Exemplo

```bash
# buildando
cd exemplo
set OPENSSL_DIR=path\to\OpenSSL-Win64      
cargo build

# suba o mosquitto (em outro terminal)
path\to\mosquitto\mosquitto.exe

# subindo um subscriber (em outro terminal)
cargo run --bin sub

# subindo um publisher (em outro terminal)
cargo run --bin pub
```