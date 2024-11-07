use std::io;

pub fn ler_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");
    input.trim().to_string()
}

