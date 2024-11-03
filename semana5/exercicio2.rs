

fn media_aritmetica(vetor: &Vec<i32>) -> f64 //novo vetor, as posicoes apontam para o endereco de memoria de v
{
    let mut soma:f64 = 0.0;
    let n: f64 = vetor.len() as f64;

    for &i in vetor //para cada endereco de memoria no vetor
    {
        soma += i as f64; //loop acessando o conteúdo no endereço de memória armazenado nas posições de vetor
    }

   soma / n

}


fn main()
{
    let mut v: Vec<i32> = Vec::new();

    println!("digite quantos números serão digitados:");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 
    let tamanho: i32 = buffer.trim().parse().expect("erro ao converter valor");

    for i in 1..=tamanho
    {
        println!("inteiro:");
        let mut buffer: String = String::new();
        std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 
        let numero: i32 = buffer.trim().parse().expect("erro ao converter valor");

        v.push(numero);
    }

    println!("media do vetor:{}",media_aritmetica(&v));

}

/* 
soma += i as f64; , porque aqui eu não preciso colocar *i? 
O ChatGPT disse:
ChatGPT

Ótima pergunta! Vou explicar a diferença e o motivo pelo qual você não precisa usar *i na linha soma += i as f64.
O que está acontecendo no laço for &i in vetor?

Em Rust, quando você faz um loop como for i in vetor, o que está acontecendo é que i é uma referência para os elementos do vetor. Ou seja, i tem o tipo &i32 (uma referência imutável para um valor i32). Se você quiser o valor ao qual i aponta, precisa desreferenciar com *i, como você fez anteriormente.

No entanto, quando você usa for &i in vetor, você está dizendo a Rust para desreferenciar automaticamente cada elemento ao iterar sobre o vetor. Nesse caso, i já não é mais uma referência, mas sim o próprio valor i32. Por isso, não há necessidade de usar *i, porque i já é o valor inteiro.
Resumindo:

    for i in vetor: i é do tipo &i32 (referência), então você precisa usar *i para obter o valor.
    for &i in vetor: i é do tipo i32 (valor), então você pode usar i diretamente, sem precisar de *.

Exemplos:

    Com referência (for i in vetor):

    rust

for i in vetor {
    soma += *i as f64;  // Necessário desreferenciar (*i)
}

Com desreferenciamento automático (for &i in vetor):

rust

    for &i in vetor {
        soma += i as f64;  // i já é o valor, não precisa de *
    }

Usar for &i in vetor é uma maneira de simplificar o código e evitar a necessidade de desreferenciar explicitamente com *. 
*/