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
  "vistoem": "12345.78"
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