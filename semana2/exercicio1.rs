use std::io:: Write;

fn ex1() {
    print!("Digite seu ano de nascimento:");
    std::io::stdout().flush().expect("erro ao dar flush no terminal");

    let mut buffer: String = String::new();

    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha");

    let ano_nascimento: u16 = buffer.trim().parse().expect("erro ao converter");
    let ano_corrente: u16 = 2024;

    println!("Sua idade é {}",ano_corrente - ano_nascimento);
}

fn main(){
    ex1();
}
