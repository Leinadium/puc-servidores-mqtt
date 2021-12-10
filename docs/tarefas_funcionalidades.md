# Servidor:
    * repositório de duplas (chave, valor /sempre strings) (igual trab 3) consultado/atualizado pelos clientes
    * cada servidor tem um identificador (0 a n-1, sendo n o número total de servidores)
    * o serviço de fila entrega na mesma hora a requisição a todos os servidores
    * para consulta, apenas um servidor executará (todos recebem, mas verificam o hash) e enviará resposta pelo "topico-resp" do JSON
    * manda msg de heartbeat ao canal tópico do monitor
    * após falha, o servidor de id (idserv+1)mod n que recebe as requisições de leitura do servidor idserv até que o novo servidor idserv esteja pronto
    * mantém log de requisições recebidas (expira em tempo X, 10x maior que o timeout do monitor) usada pelo servidor substituto em caso de falhas (o cliente pode receber de novo o resultado de uma requisição que já havia recebido)
    * servidores novos (gerado após falha) após entrarem em atividade, enviam msg JSON (tipomsg ("novoserv"), topico-resp) ao tópico inf1406-reqs
    * o servidor substituto deve enviar para o topico-resp o estado atual (chave, valor)
    * em que momento sincronizar o servidor substituto com o novo servidor (funcionalidade em aberto)
    * para testes: requisição kill para matar um dos servidores

# Monitor:
    * escuta ao canal (tópico) inf1406-monitor
    * recebe mensagens de heartbeat em JSON (idServ)
    * estabelece um timeout para recebimento de heartbeats de cada servidor
    * servidores que passam o timeout, falharam (única falha considerada é crash)
    * quando há falha, envia a inf1406-reqs uma msg (tipomsg ("falhaserv"), idserv (quem falhou), vistoem (último momento de heartbeat)).

# Cliente:
    * enviam requisições ao tópico MQTT inf1406-reqs em JSON (tipomsg, chave, topico-resp e idpedido). Para inserção também há um campo "novovalor".
    * realiza operações:
      -> inserção (atualização): executada em todas as réplicas do serviço
      -> consulta (leitura): executada apenas por um dos servidores


# Testes
## Funcionamento normal:
    * inserção e consulta (testes iguais trabalho 3)
    * teste extra com muitos clientes requisitando aos servidores ao mesmo tempo
## Funcionamento com falhas
    * funcionamento normal até determinado tempo e depois realiza o kill de um servidor
    * sequência de passos para testar a resistência a falhas