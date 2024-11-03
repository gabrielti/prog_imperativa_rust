use std::io:: Write;

fn ex2(){

    print!("Digite o numero de dias que o carro será utilizado:");
    std::io::stdout().flush().expect("erro ao dar flush no terminal");

    let mut buffer: String = String::new();

    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha");

    let numero_dias: u16 = buffer.trim().parse().expect("erro ao converter");

    println!("Preço a pagar é:{}",numero_dias*100);
}

//fn ex1() {
//    print!("Digite seu ano de nascimento:");
//    std::io::stdout().flush().expect("erro ao dar flush no terminal");

//    let mut buffer: String = String::new();

//    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha");

//    let ano_nascimento: u16 = buffer.trim().parse().expect("erro ao converter");
//    let ano_corrente: u16 = 2024;

//    println!("Sua idade é {}",ano_corrente - ano_nascimento);
//}

fn main(){
    //ex1();
    ex2();
}
