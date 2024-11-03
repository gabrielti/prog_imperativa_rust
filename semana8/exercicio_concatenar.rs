/*      Resolucao Professor
fn concatenar(s1: String, s2: String) -> String {
    format!("{s1} {s2}");
}

fn concatenar(s1: String, s2: String) -> String {
    let mut s3 = s1; //s3 é dinamico e s1 é estatico
    s3.push_str(&s2); //conversao dinamico para estatico
    s3
}

fn concatenar(s1: String, s2: String) -> String {
    s1+&s2 o operador de soma consome valores s1 e s2, se fosse s1 + s2 ele consumiria as 2, mataria as ownership e nao
           retornatia nada!
}
*/

fn concatenar(s: String, r: String) -> String { //ownderships sao passadas para s e r
    
    let mut array = String::new();

    for letra in s.chars() {
        array.push(letra);
    }

    for letra in r.chars() {
        array.push(letra);
    }

    array
}

fn main() {
    let a: String = String::from("olá");
    let b: String = String::from("mundo");
    let c: String = String::from("!");

    let resultado1 = concatenar(a,b); //Depois do retorno da funcao, a string a e b nao existem mais, sao descartadas da memoria
    //println!("{a},{b}"); observe que da erro porque apos a funcao concatenar ele da um Drop
    let resultado2 = concatenar(resultado1,c);
    println!("{}",resultado2);
}
