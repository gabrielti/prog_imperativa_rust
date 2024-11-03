enum Direcoes {
    Frente,
    Tras,
    Esquerda,
    Direita,
}

fn frase(escolha: Direcoes) {
    match escolha {
        Direcoes::Frente => println!("Voce esta indo para Frente!"),
        Direcoes::Tras => println!("Voce esta indo para Tras!"),
        Direcoes::Esquerda => println!("Voce esta indo para Esquerda!"),
        Direcoes::Direita => println!("Voce esta indo para Direita!"),
    }
}

fn main() {
    println!("Opcoes:");
    println!("1 - Frente");
    println!("2 - Tras");
    println!("3 - Esquerda");
    println!("4 - Direita");

    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha");

    let escolha: i32 = buffer.trim().parse().expect("erro ao converter valor");

    let direcao = match escolha {
        1 => Direcoes::Frente,
        2 => Direcoes::Tras,
        3 => Direcoes::Esquerda,
        4 => Direcoes::Direita,
        _ => {
            println!("Escolha inválida!");
            return;
        }
    };

    frase(direcao);
}

/*O erro está na maneira como você está tentando combinar uma String (ou &str) com a enum Direcoes no match. O match precisa ser feito diretamente com a enum, mas você está tentando passar uma string, o que não é compatível. Além disso, a enum Direcoes não implementa o String diretamente.

Aqui está uma solução que corrige esses problemas, modificando o código para que a escolha do usuário seja transformada corretamente em uma enum Direcoes e usada no match:

    Use um match para converter o número escolhido em um valor da enum Direcoes.
    Passe a enum diretamente para a função frase.

    O valor escolhido pelo usuário é convertido em um valor da enum Direcoes usando um match.
    A função frase agora aceita um valor de tipo Direcoes, e o match dentro dela funciona diretamente com a enum.

Agora, o código vai funcionar corretamente, e as opções escolhidas pelo usuário vão se traduzir em uma mensagem apropriada.*/
