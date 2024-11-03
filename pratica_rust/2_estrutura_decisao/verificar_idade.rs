use std::io::Write;


fn verifica_idade(){

    let mut buffer: String = String::new();
    std::io::stdout().flush().expect("ERROR");

    println!("Digite sua idade:");
    std::io::stdin().read_line(&mut buffer).expect("ERROR");

    let idade: u8 = buffer.trim().parse().expect("ERROR");

    if idade >= 18{
        println!("maior de idade!");
    }

    else{
        println!("menor de idade!");
    }
}

fn main(){
    verifica_idade();
}
