struct Pessoa{
    nome: String,
    idade: u8,
    altura: f32
}


impl Pessoa{

    fn new(nome: String, idade: u8, altura: f32) -> Pessoa {
        Pessoa {nome,idade,altura}
    }

    fn falar(&self, mensagem: String)
    {
        println!("A pessoa {} disse {}",self.nome,mensagem)
    }
}

fn main(){
    let p: Pessoa = Pessoa::new(String::from("Gabriel"), 25, 1.77);
    p.falar("Olá, mundo".to_string());
}

//Fazer toda a lista do TDE