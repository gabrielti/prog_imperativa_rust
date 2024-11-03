//passagem por copia 
//fatorial_iterativo
fn fatorial(mut numero_fatorial: i32) -> i32
{
    let mut multiplicacao: i32 = 1;

    while numero_fatorial > 1
    {
        multiplicacao = multiplicacao * numero_fatorial;
        numero_fatorial -= 1;
    }

    multiplicacao

    //crescente
    //for i in 1..=n
    //{
    //resultado = resultado * i;
    //}

    //descrescente
    //for i in (1..=n).rev() -> Equivale a minha funcao
    //{
    //resultado = resultado * i;
    //}

    /*Recursão -> Teste de mesa
    fn fatorial_recursivo(n:i32) -> i32{

    if n == 0
    {
        return 1;
    }

    fatorial_recursivo(n-1) * n

    }
    */

}


fn main()
{
    println!("digite um inteiro:");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 
    let numero: i32 = buffer.trim().parse().expect("erro ao converter valor");
    
    println!("fatorial:{}",fatorial(numero));

}