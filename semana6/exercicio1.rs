// Definindo o enum Voto
enum Voto {
    Favor,
    Contra,
    Branco,
}

// Função para contar os votos
fn contagem_de_votos(lista_voto: &Vec<Voto>) {
    // Inicializando contadores
    let mut favor = 0;
    let mut contra = 0;
    let mut branco = 0;

    // Iterando sobre a lista de votos
    for voto in lista_voto {
        match voto {
            Voto::Favor => favor += 1,   // Voto a favor
            Voto::Contra => contra += 1,  // Voto contra
            Voto::Branco => branco += 1,  // Voto branco
        }
    }

    // Exibindo a contagem de votos
    println!("Votos a favor: {}", favor);
    println!("Votos contra: {}", contra);
    println!("Votos em branco: {}", branco);
}

fn main() {
    // Lista de votos representados pela enumeração Voto
    let lista = vec![
        Voto::Favor, 
        Voto::Contra, 
        Voto::Contra, 
        Voto::Favor, 
        Voto::Branco, 
        Voto::Favor, 
        Voto::Favor, 
        Voto::Contra, 
        Voto::Favor, 
        Voto::Branco, 
        Voto::Branco, 
        Voto::Favor, 
        Voto::Branco
    ];
    // Chamando a função de contagem de votos
    contagem_de_votos(&lista);
}


/*

Poderia me explicar enums em Rust?
ChatGPT said:
ChatGPT

Enums em Rust são usados para definir um tipo que pode representar uma variedade de valores diferentes, mas pré-definidos. Eles são úteis quando você deseja representar um conjunto finito de possibilidades para um valor.

Um enum é declarado usando a palavra-chave enum, seguida pelo nome do enum e as variantes possíveis. Cada variante pode ser simplesmente um nome, ou pode conter dados associados.

Aqui está um exemplo básico:

rust

enum DiaDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

Neste exemplo, o enum DiaDaSemana define sete variantes, cada uma representando um dia da semana.
Usando Enums

Você pode usar enums de várias maneiras, inclusive em match expressions, que permitem combinar um valor com uma de suas variantes:

rust

fn main() {
    let hoje = DiaDaSemana::Quarta;

    match hoje {
        DiaDaSemana::Segunda => println!("Hoje é segunda!"),
        DiaDaSemana::Terca => println!("Hoje é terça!"),
        DiaDaSemana::Quarta => println!("Hoje é quarta!"),
        DiaDaSemana::Quinta => println!("Hoje é quinta!"),
        DiaDaSemana::Sexta => println!("Hoje é sexta!"),
        DiaDaSemana::Sabado => println!("Hoje é sábado!"),
        DiaDaSemana::Domingo => println!("Hoje é domingo!"),
    }
}

Enums com Dados

Enums em Rust podem conter dados associados a cada variante. Isso os torna mais poderosos do que enums em outras linguagens que só permitem variantes simples.

Aqui está um exemplo de um enum que representa um tipo de Mensagem que pode ter diferentes formas de dados associados:

rust

enum Mensagem {
    Texto(String),
    Imagem { url: String, largura: u32, altura: u32 },
    Video { url: String, duracao: u32 },
    Sair,
}

Aqui, a variante Texto contém uma String, enquanto a variante Imagem tem um objeto anônimo com url, largura e altura, e a variante Video também tem url e duração. A variante Sair não contém dados.
Manipulando Dados com Enums

Você pode acessar os dados dentro de uma variante do enum usando um match:

rust

fn processar_mensagem(msg: Mensagem) {
    match msg {
        Mensagem::Texto(texto) => println!("Texto recebido: {}", texto),
        Mensagem::Imagem { url, largura, altura } => {
            println!("Imagem recebida: {} ({}x{})", url, largura, altura);
        }
        Mensagem::Video { url, duracao } => {
            println!("Vídeo recebido: {} ({} segundos)", url, duracao);
        }
        Mensagem::Sair => println!("Recebido comando para sair."),
    }
}

fn main() {
    let m = Mensagem::Texto(String::from("Olá, mundo!"));
    processar_mensagem(m);
}

Vantagens dos Enums em Rust

    Segurança: Ao usar enums, você força o compilador a garantir que todas as possibilidades são tratadas, o que reduz a chance de erros.

    Versatilidade: Enums com dados associados permitem combinar variantes com diferentes tipos de informações, o que facilita a criação de tipos complexos e flexíveis.

    Clareza: Enums tornam o código mais legível e fácil de entender, especialmente em situações onde um valor pode ter múltiplos estados ou formas.

Essas características tornam os enums uma ferramenta poderosa para modelar dados e lógica em Rust.

*/