use std::io::Write;

fn ex3(){
    
    let mut buffer: String = String::new();
    
    println!("fahrenheit:");
    std::io::stdout().flush().expect("ERROR");
    std::io::stdin().read_line(&mut buffer).expect("ERROR");
    
    let fahrenheit: f32 = buffer.trim().parse().expect("ERROR");
    let celsius: f32 = (fahrenheit-32.0) * 5.0/9.0;
    
    println!("celsius: {celsius:.2}");
}

fn main(){
    ex3();
}
