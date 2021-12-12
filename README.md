# puc-servidores-mqtt
Repositório para o trabalho 4 de INF1406 Programação Distribuída da PUC-Rio 

# Implementação

## Server

O servidor ele possue uma thread, e um loop principal. Essa thread é iniciada logo ao
iniciar o programa, e é responsável por ficar enviando *heartbeats* para o monitor.

O loop principal é um loop nas mensagens mqtt que ele recebe. Ao receber uma mensagem,
ele faz um parsing dela, e determina se precisa fazer alguma ação ou não.

As ações possíveis: 

* Leitura -> Se o servidor é o responsável pela chave, ele vai pegar o valor, e responde o cliente.
* Inserção -> O servidor insere a chave na sua estrutura interna.
* Morte -> Se o servidor é o backup do servidor que morreu, ele vê as mensagens que este
deveria ter respondido, e fica responsável por responder no lugar deste por enquanto.
* Nascimento -> Se o servidor estava de backup do servidor que acabou de renascer, ele
passa a estrutura interna para este, e para de responder no lugar deste.
* Atualização -> (Quando servidor nasceu) Atualiza a estrutura interna.
* Derrubar -> Para controle, derruba o servidor.

Todas as mensagens importantes ele salva num log, que é usado para responder perguntas que 
algum servidor pode ter perdido.

Há um problema na implementação dos servidores, em que se dois servidores vizinhos morrerem, 
por exemplo, o servidor 1 e 2, o 3 irá servir de backup somente para o 2. Havia sido
feita uma implementação para o servidor ser responsável por todos os vizinhos abaixo dele
caso fosse necessário, mas foi abandonado a ideia pois a implementação começou a não ser
tão trivial.

## Monitor

O monitor é um loop infinito. Em cada loop, ele verifica se tem alguma mensagem que chegou.
Se tiver, ele atualiza a estrutura interna de heartbeats. Depois disso, ele passa pela
estrutura interna, vendo se algum servidor sofreu timeout. Se sim, ele envia uma mensagem
de morte para os servidores. Depois disso, ele dorme por 1ms, e volta pro inicio do loop.

## Client

O client é um servidor simples, que envia uma mensagem para os servidores.

É possível enviar uma mensagem falando para inserir, consultar, ou derrubar algum servidor.

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
# insere um valor
target\debug\client.exe [ID_CLIENTE] insere [CHAVE] [VALOR]
# consulta um valor
target\debug\client.exe [ID_CLIENTE] consulta [CHAVE]
# derruba um servidor
target\debug\client.exe [ID_CLIENTE] derrubar [SERVIDOR]
# derruba tudo
target\debug\client.exe [ID_CLIENTE] derrubar

# exemplos de execucao de client
client.exe 123 insere bom dia

client.exe 123 consulta bom
> Recebi de volta: { "idserv": 0, "resposta": "dia" }

# para controle
client.exe 123 derrubar     # derruba servidores e monitor
client.exe 123 derrubar 1   # derruba o servidor 1
```

# Testes

Esses são os casos de testes implementados:
```text
Casos de Teste de Funcionamento Normal:
    1. Teste de inserção e consulta:
            * insere e consulta em ordem
    2. Teste de consulta antes de inserção:
            * consulta e insere em ordem
    3. Teste de várias consultas e inserções desordenadas

Casos de Teste de Funcionamento com Falhas:
    4. Teste de resposta após crash:
            * insere, consulta, derruba e revive em ordem
    5. Teste de consulta de inserção após crash:
            * insere, derruba, consulta e revive em ordem
    6. Teste de inserção de chave de uma consulta pedida anteriormente ao crash:
            * consulta, derruba, insere e revive em ordem
    7. Teste de consulta, inserção e crash desordenados
```

Para executar, execute o script de testes ```projeto/tests.py```