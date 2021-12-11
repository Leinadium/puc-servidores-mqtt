from paho.mqtt.client import Client, MQTTMessage
from json import dumps
from sys import argv


def on_connect(client: Client, userdata, flags, rc):
    print(f"Conectado com code: {rc}")
    client.subscribe("inf1406-reqs")
    client.subscribe("inf1406-resposta-teste")


def on_message(client: Client, userdata, msg: MQTTMessage):
    print(f'topico: {msg.topic}, mensagem: {msg.payload}')


c = Client()
c.on_connect = on_connect
c.on_message = on_message

c.connect('localhost', 1883, 60)

mensagens = {
    "i": dumps({
        "tipomsg": "insercao",
        "chave": 123,
        "novovalor": "bom dia",
        "topico-resp": "inf1406-resposta-teste",
        "idpedido": 1234
    }),

    "l": dumps({
        "tipomsg": "leitura",
        "chave": 123,
        "topico-resp": "inf1406-resposta-teste",
        "idpedido": 12345
    }),

    "m": dumps({
        "tipomsg": "falhaserv",
        "idserv": 0,
        "vistoem": "0.0"
    })
}


if __name__ == "__main__":
    if len(argv) > 1:
        mensagem = mensagens.get(argv[1])
        if mensagem is None:
            mensagem = mensagens['i']
    else:
        mensagem = mensagens['i']

    c.publish('inf1406-reqs', payload=mensagem, qos=1)
    c.loop_forever()
