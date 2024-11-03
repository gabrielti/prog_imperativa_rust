

fn is_palindromo(texto: &String) -> bool
{
    let texto_reverso: String = texto.clone();

    for(c1,c2) in texto.chars().zip(texto_reverso.chars().rev()){

        if c1 != c2{
            return false;
        }

    }

    true

}


fn main()
{
    let texto1 = String::from("teste");
    println!("o texto é um palindromo:{}",is_palindromo(&texto1));
}
