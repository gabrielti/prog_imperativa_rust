fn exercicio1(){
    let PI: f32 = 3.14159;
    let mut raio = String::new();
    println!("Digite o raio:");
    std::io::stdin().read_line(&mut raio).unwrap();

    let raio_float: f32 = raio.trim().parse().unwrap();
    let area: f32 = PI * raio_float.powi(2);
    println!("Area da circunferencia de raio{raio_float} é:{area}");
}

fn main(){
    exercicio1();
}
