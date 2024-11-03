def exercicio1():
    PI = 3.15159

    radius = float(input("raio:"))

    comprimento = PI*radius*2
    area = PI*radius**2

    print("area:",area)
    print("comprimento:",comprimento)


def exercicio2():
    contador = 1

    while(contador < 11):
        print("contador:",contador)
        contador += 1

def exercicio3():
    soma = 0

    for i in range(11):
        soma += i

    print("soma de 0 a 10:",soma)

def exercicio4():
    i = 1
    inteiro = int(input("digite um inteiro:"))

    for i in range(inteiro+1):
        print(i)


def exercicio5():
    numero = int(input("digite um numero inteiro positivo:"))

    for i in range(numero+1): #range(1,numero+1)
        if i%2 == 0 and i > 1:
            print(i)

def exercicio6():
    senha = '12345'
    senha_usuario = input("senha:")

    if(senha_usuario == senha):
        print("Acesso concedido")

    else:
        print("Senha Incorreta")

def exercicio7():
    idade_minima = 18
    idade = int(input("digite sua idade:"))

    if(idade >= idade_minima):
        print("Voce é maior de idade")

    else:
        print("Voce é menor de idade")

def exercicio8():
    fatorial = 1
    numero = int(input("digite um inteiro positivo:"))

    while(numero != 1):
        fatorial = numero*fatorial
        numero -= 1

    print("fatorial:",fatorial)

def exercicio9():
    numero = 10

    while(numero >= 0):
        print("numero:",numero)
        numero -= 2

def exercicio10():
    count = 0
    numero = int(input("digite um numero inteiro positivo:"))

    for i in range(numero + 1):
        if(i>0 and numero%i == 0):
            count+=1

    if(count == 2):
        print("numero primo")

    else:
        print("numero nao primo")

exercicio10()

