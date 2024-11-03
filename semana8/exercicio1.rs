

fn completar(texto: &String) -> String {
    let dot: String = ".";

    dot + &texto
} 



fn main(){
    let mut array = String::new();
    let resultado = completar(&array);
    println!("{resultado}");
}