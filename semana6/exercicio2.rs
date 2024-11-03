// Definindo a enumeração Carta com naipes e valores
enum Naipe {
    Paus,
    Copas,
    Espadas,
    Ouros,
}

enum Valor {
    As,
    Dois,
    Tres,
    Quatro,
    Cinco,
    Seis,
    Sete,
    Oito,
    Nove,
    Dez,
    Valete,
    Dama,
    Rei,
}

// Definindo a enumeração Carta que combina Naipe e Valor
enum Carta {
    Carta { naipe: Naipe, valor: Valor },
}

// Função para obter o valor em pontos da carta
fn valor_da_carta(carta: &Carta) -> i32 {
    match carta {
        Carta::Carta { valor, .. } => match valor {
            Valor::As => 1,
            Valor::Dois => 2,
            Valor::Tres => 3,
            Valor::Quatro => 4,
            Valor::Cinco => 5,
            Valor::Seis => 6,
            Valor::Sete => 7,
            Valor::Oito => 8,
            Valor::Nove => 9,
            Valor::Dez => 10,
            Valor::Valete => 11,
            Valor::Dama => 12,
            Valor::Rei => 13,
        },
    }
}

fn main() {
    // Exemplo de uso da função valor_da_carta
    let carta = Carta::Carta {
        naipe: Naipe::Paus,
        valor: Valor::Dama,
    };

    let valor = valor_da_carta(&carta);
    println!("O valor da carta é: {}", valor);
}

//O Naipe nesse caso não interfere no valor, porque para todos são iguais
//Se eu colocar por exemplo, Copas = 9, eu atribui um valor a soma em Copas, exemplo: TresCopas = 3+9 = 12

/*

Entao na verdade o Enum só serve para nao deixar que haja um input diferente do que eu espero ao invez de ficar lidando com comparacao de strings por exemplo?
ChatGPT said:
ChatGPT

Exatamente! O uso de enums em Rust serve para representar um conjunto fixo de valores ou opções de maneira segura e clara. Aqui estão algumas vantagens principais do uso de enums:
Vantagens dos Enums

    Segurança de Tipo:
        Enums garantem que apenas valores válidos sejam utilizados, prevenindo erros que poderiam ocorrer com entradas inválidas ou inesperadas.

    Facilidade de Manutenção:
        Se você precisar adicionar novos valores ou opções, você só precisa modificar o enum em um único lugar. Isso ajuda a manter o código mais organizado e fácil de manter.

    Correspondência de Padrões (Pattern Matching):
        Enums trabalham muito bem com o match, permitindo uma correspondência de padrões clara e concisa, o que facilita a manipulação e o controle dos diferentes valores.

    Legibilidade e Clareza:
        Usar enums torna o código mais legível e expressivo. Você pode nomear os valores de forma que reflitam claramente o que eles representam.

    Erro de Compilação:
        O compilador Rust verifica se todos os casos possíveis de um enum são tratados, reduzindo o risco de erros em tempo de execução.

Exemplo Prático

No contexto do seu código para o baralho de cartas:

    Enums são usados para definir os naipes e valores das cartas, o que evita a necessidade de comparar strings ou lidar com entradas erradas.


+50

You should always use enums when a variable (especially a method parameter) can only take one out of a small set of possible values. Examples would be things like type constants (contract status: "permanent", "temp", "apprentice"), or flags ("execute now", "defer execution").

If you use enums instead of integers (or String codes), you increase compile-time checking and avoid errors from passing in invalid constants, and you document which values are legal to use.

BTW, overuse of enums might mean that your methods do too much (it's often better to have several separate methods, rather than one method that takes several flags which modify what it does), but if you have to use flags or type codes, enums are the way to go.


pra que serva :: em rust?
ChatGPT said:
ChatGPT

Em Rust, o operador :: é usado para acessar itens de um módulo ou namespace, como funções, structs, enums e constantes. Ele ajuda a qualificar nomes e a referenciar itens que estão dentro de um módulo específico.

Aqui estão alguns exemplos de como o :: é usado:

    Acesso a Funções e Métodos:

    rust

mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn main() {
    let sum = math::add(5, 3);
    println!("A soma é: {}", sum);
}

Acesso a Constantes:

rust

mod constants {
    pub const PI: f64 = 3.14159;
}

fn main() {
    println!("O valor de PI é: {}", constants::PI);
}

Acesso a Enums:

rust

mod shapes {
    pub enum Shape {
        Circle,
        Square,
    }
}

fn main() {
    let my_shape = shapes::Shape::Circle;
    match my_shape {
        shapes::Shape::Circle => println!("É um círculo!"),
        shapes::Shape::Square => println!("É um quadrado!"),
    }
}

Acesso a Métodos de Structs:

rust

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    fn main() {
        let rect = Rectangle { width: 30, height: 50 };
        println!("Área do retângulo: {}", rect.area());
    }

O operador :: permite que você acesse esses itens de maneira clara e organizada, mantendo a modularidade e a encapsulação no código.


->Chaves denotão structs {}


*/