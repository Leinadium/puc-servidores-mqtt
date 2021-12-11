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
  "vistoem": "hora-formato-a-definir"
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
*TODO*