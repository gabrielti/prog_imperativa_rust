

struct Pessoa{
    nome: String,
    idade: u8
}

fn main() {
    //criacao var(pessoa) do tipo Pessoa(struct) = estrutura P

    let pessoa: Pessoa = Pessoa{
        nome: String::from("Gabriel"),
        idade: 28
    };
}

/*
Em Rust, o = Pessoa após let pessoa: Pessoa é usado para inicializar a variável pessoa com uma instância da struct Pessoa. Isso faz parte da sintaxe de construção de structs em Rust. Vamos entender passo a passo:

    Declaração da struct:

    rust

struct Pessoa {
    nome: String,
    idade: u32,
}

Aqui você está declarando um tipo chamado Pessoa que contém dois campos: nome e idade.

Criação de uma instância da struct: Quando você faz:

rust

let pessoa: Pessoa = Pessoa {
    nome: String::from("Gabriel"),
    idade: 28,
};

Você está criando uma nova instância da struct Pessoa. O = Pessoa é a parte onde você invoca o construtor da struct (que é gerado automaticamente) para criar essa instância.

Isso significa:

    Pessoa { ... } é o que cria a instância da struct.
    Atribuir com = significa que você está atribuindo essa nova instância à variável pessoa.

Equivalente em outra linguagem: Em linguagens orientadas a objetos, isso é semelhante a usar um construtor para criar um objeto. Por exemplo, em Python você faria:

python

    pessoa = Pessoa(nome="Gabriel", idade=28)

Em resumo, o = Pessoa { ... } é necessário porque você está criando e inicializando uma instância da struct Pessoa e atribuindo à variável pessoa.
*/