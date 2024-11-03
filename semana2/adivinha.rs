fn main(){

    println!("digite sua idade:");
    let mut buffer: String = String::new(); //nome buffer do tipo string que vai receper como parametro uma string nova
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); // entender :: & e .

    let idade: u8 = buffer.trim().parse().expect("erro ao converter valor"); //trim remove a quebra de linha por causa do enter
    
    println!("sua idade é: {idade}");
}
