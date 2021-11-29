# Linguagem e biblioteca e broker

* **Linguagem:** Rust
* **API MQTT:** paho-mqtt
* **Outras bibliotecas para Rust:** [serde-rs/json](https://github.com/serde-rs/json)
* **Broker:** mosquitto
	
# Testes realizados

Mosquitto testado em máquina local

Testado uma implementação básica de comunicação, atráves da biblioteca em Rust e o broker local

# Estrutura

Servidor e Monitor como processos (igual a implementação do projeto anterior

Para o servidor tratar alguma mensagem importante (que aquele servidor precisa usar), *talvez* spawnar uma thread.

Para o monitor gerar heartbeats -> thread

Para o monitor receber heartbeats -> processamento do loop main normal, ou thread para tratar igual, igual o server

Para os servidor receberem e enviarem heartbeats -> thread

# Tópico mensagens

	* reqs
		client   -> servidor
		monitor  -> servidor (na morte de um server)
		servidor -> servidor (no nascimento de um server)

	* mon
		monitor  -> servidor (heartbeat)
		servidor -> monitor  (heartbeat)

# Testes

Um script para gerar todos os casos que a gente ache interessante

Em bash (verificação talvez mais manual), 
ou python (possível verificação automática), o que der tempo para fazer.