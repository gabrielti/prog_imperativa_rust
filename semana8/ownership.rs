fn pega_propriedade(s1: String) // -> String
{
println!("Aqui eu peguei a propriedade da String {} ({:p})", s1, s1.as_ptr());
// s1
}
fn copiar(i: i32) {
println!("Copiei o valor {}", i);
}
fn main() {
let s: String = String::from("Luffy");
println!("Endereço de s={:p}", s.as_ptr());
pega_propriedade(s);

//println!("{s}"); depois que pega_propriedade foi utilizada ela desaloca a memoria de s... Ela nao exite mais porque termina o escopo e gera a funcao DROP()...Esta la no slide de ponteiros e referencias... Ele liberar a Heap e a Stack

//Um jeito de contornar isso é retornar o valor da variavel ao final de pega_propriedade

let x: i32 = 5;
copiar(x);
println!("Valor de x={}", x);
}
