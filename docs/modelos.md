## ClientLeitura
```json
{
  "tipomsg": "leitura",
  "chave": "xxx",
  "topico-resp": "inf1406-xxx",
  "idpedido": 1234
}
```

## ClientInserção:

```json
{
  "tipomsg": "insercao",
  "chave": "xxx",
  "novovalor": "xxx",
  "topico-resp": "inf1406-xxx",
  "idpedido": 1234
}
```

## MonitorMorte:
```json
{
  "tipomsg": "falhaserv",
  "idserv": 1234,
  "vistoem": "1234578"
}
```

## ServidorNascimento:
```json
{
  "tipomsg": "novoserv",
  "idserv": 1234,
  "topicoresp": "inf1406-xxx"
}
```

## ServidorAtualizacao:
É enviado por outro topico
```json
{
  "tipomsg": "atualizacao",
  "conteudo": {
    "chave1": "valor1",
    "chave2": "valor2"
  }
}
```

## Controle para derrubar servidor
Serve para derrubar algum servidor específico.
```json
{
  "tipomsg": "assassinato",
  "idserv": 1234
}
```