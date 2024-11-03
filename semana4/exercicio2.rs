

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