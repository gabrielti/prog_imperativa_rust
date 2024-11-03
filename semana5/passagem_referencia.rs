//passagem por referencia 
//fatorial_iterativo
/*
fn fatorial(mut numero_fatorial: i32) -> i32
{
    let mut multiplicacao: i32 = 1;

    while numero_fatorial > 1
    {
        multiplicacao = multiplicacao * numero_fatorial;
        numero_fatorial -= 1;
    }

    multiplicacao
}*/

//& indica a referencia (endereco da memoria)
//* indica que é um ponteiro, variavel que armazena o endereco de memória

fn dobro(i: &mut i32)
{
    *i = *i *2;
}

fn main()
{
    /*
    println!("digite um inteiro:");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); //read_line no endereco de memória de buffer 
    let numero: i32 = buffer.trim().parse().expect("erro ao converter valor");
    
    println!("fatorial:{}",fatorial(numero));
    */

    //variaveis do tipo complexo ele armazena o endereco
    //variaveis do tipo primitiva nao funcionam

    let x: i32 = 10;
    let x_ref: &i32 = &x

    dobro(&mut x);

    //println!("x esta no endereco{} e esta no endereco{}",x_ref,x);

}