import subprocess
from sys import argv
from time import sleep


def create_start_servers(n_servers: int):
    # inicializa todos os n_servers servers
    for id_server in range(n_servers):
        with open("log/server_" + str(id_server) + ".txt", "wb") as output:
            subprocess.Popen([
                server_path, str(id_server), str(n_servers),
                #"> server_" + str(id_server) + ".txt"
            ], stdout=output, stderr=output)


def create_server_revive(n_servers: int, id_server: int):
    # revive um server que crashou
    with open("log/server_" + str(id_server) + ".txt", "ab") as output:
        subprocess.Popen([
            server_path, str(id_server), str(n_servers), "-b",
            #"> server_" + str(id_server) + ".txt"
        ], stdout=output)


def create_monitor():
    # inicializa o monitor
    with open("log/monitor.txt", "wb") as output:
        subprocess.Popen([
            monitor_path,
            #"> monitor.txt"
        ], stdout=output)


def create_client_insere(id_cliente: int, chave: str, valor: str):
    # cria um cliente de inserção
    with open("log/client_" + str(id_cliente) + ".txt", "wb") as output:
        subprocess.Popen([
            client_path, str(id_cliente), "insere", chave, valor,
            #"> cliente_" + str(id_cliente) + ".txt"
        ], stdout=output)


def create_client_consulta(id_cliente: int, chave: str):
    # cria um cliente de consulta
    with open("log/client_" + str(id_cliente) + ".txt", "wb") as output:
        subprocess.Popen([
            client_path, str(id_cliente), "consulta", chave,
            #"> cliente_" + str(id_cliente) + ".txt"
        ], stdout=output)


def create_client_derruba(id_cliente: int, id_server: str):
    # cria um cliente que derruba um servidor
    with open("log/client_" + str(id_cliente) + ".txt", "wb") as output:
        subprocess.Popen([
            client_path, str(id_cliente), "derrubar", id_server,
            #"> cliente_" + str(id_cliente) + ".txt"
        ], stdout=output)


def create_client_derruba_todos(id_cliente):
    # cria um cliente para terminar todos os servidores e o monitor
    with open("log/client_derruta_todos.txt", "wb") as output:
        subprocess.Popen([
            client_path, str(id_cliente), "derrubar"
        ], stdout=output)


def test1():
    # inicializa infraestrutura
    create_start_servers(2)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    # insere e consulta em ordem
    create_client_insere(0, "teste", "teste")
    sleep(2)
    create_client_consulta(1, "teste")
    sleep(5)

    # termina
    create_client_derruba_todos(2)


def test2():
    # inicializa infraestrutura
    create_start_servers(2)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    # consulta e insere em ordem
    create_client_consulta(1, "teste")
    sleep(2)
    create_client_insere(0, "teste", "teste")
    sleep(5)

    # termina
    create_client_derruba_todos(2)


def test3():
    # inicializa infraestrutura
    create_start_servers(8)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    create_client_consulta(0, "teste10")
    create_client_consulta(1, "teste")
    create_client_insere(2, "teste", "teste")
    create_client_insere(3, "teste", "teste")
    create_client_consulta(4, "teste")
    create_client_insere(5, "teste", "teste")
    create_client_consulta(6, "teste")
    create_client_insere(7, "teste", "teste")
    sleep(5)

    # termina
    create_client_derruba_todos(2)


def test4():
    # inicializa infraestrutura
    create_start_servers(2)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    # insere, consulta, derruba e revive em ordem
    create_client_insere(0, "teste2", "abc")
    sleep(2)
    create_client_consulta(1, "teste2")
    sleep(2)
    create_client_derruba(2, 1)
    sleep(11)
    create_server_revive(2, 1)
    sleep(5)

    # termina
    create_client_derruba_todos(2)


def test5():
    # inicializa infraestrutura
    create_start_servers(2)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    # insere, derruba, consulta e revive em ordem
    create_client_insere(0, "teste2", "abc")
    sleep(2)
    create_client_derruba(2, 2)
    sleep(11)
    create_client_consulta(1, "teste2")
    sleep(2)
    create_server_revive(2, 2)
    sleep(5)

    # termina
    create_client_derruba_todos(2)


def test6():
    # inicializa infraestrutura
    create_start_servers(8)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    # consulta, derruba, insere e revive em ordem
    create_client_consulta(1, "teste2")
    sleep(2)    
    create_client_derruba(2, 2)
    sleep(11)
    create_client_insere(0, "teste2", "abc")
    sleep(2)
    create_server_revive(2, 2)
    sleep(5)

    # termina
    create_client_derruba_todos(2)


def test7():
    # inicializa infraestrutura
    create_start_servers(8)
    sleep(2)
    create_monitor()
    sleep(1)

    # testa
    # create_client_insere(0, "teste2", "abc")
    # sleep(1)
    # create_client_consulta(1, "teste2")
    # sleep(1)
    # create_client_derruba(2, 2)
    # sleep(1)
    # create_server_revive(2, 2)
    # sleep(1)

    # termina
    create_client_derruba_todos(2)


def select_test():
    while True:
        print(
            """
            Casos de Teste de Funcionamento Normal:

            \t1. Teste de inserção e consulta:
            \t\t* insere e consulta em ordem
            \t2. Teste de consulta antes de inserção:
            \t\t* consulta e insere em ordem
            \t3. Teste de várias consultas e inserções desordenadas
            
            Casos de Teste de Funcionamento com Falhas:

            \t4. Teste de resposta após crash:
            \t\t* insere, consulta, derruba e revive em ordem
            \t5. Teste de consulta de inserção após crash:
            \t\t* insere, derruba, consulta e revive em ordem
            \t6. Teste de inserção de chave de uma consulta pedida anteriormente ao crash:
            \t\t* consulta, derruba, insere e revive em ordem
            \t7. Teste de consulta, inserção e crash desordenados
            """
        )
        test_case = input("Escolha um caso de teste:\n")
        try:
            test_case = int(test_case)
            if not (test_case > 0 and test_case <= 6):
                raise
            return test_case
        except:
            print("Caso de teste inválido. Por favor, escolha um caso de teste entre 1 e 6.\n\n")


def main():    
    # SETUP
    test_case = select_test()
    path = input("Diretório dos executáveis:\n")
    if path == "":
        path = "./target/release"
    global server_path, client_path, monitor_path
    server_path = path + "/server.exe"
    client_path = path + "/client.exe"
    monitor_path = path + "/monitor.exe"

    # RUN TEST
    switch = {
        "1": test1,
        "2": test2,
        "3": test3,
        "4": test4,
        "5": test5,
        "6": test6,
        "7": test7
    }

    case = switch.get(test_case, test1)
    case()


if __name__ == '__main__':
    main()