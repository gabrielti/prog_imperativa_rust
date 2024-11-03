


fn main(){

    println!("digite sua idade:");

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("ERROR");

    let idade: u8 = buffer.trim().parse().expect("ERROR");

    println!("sua idade é: {idade}");
}
