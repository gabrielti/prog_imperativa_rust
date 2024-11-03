use std::io::Write;

fn ex2() {
	let mut contador = 0;
	let mut S = String::from("admin");
	let mut R = String::from("@adm1n");
    let mut buffer: String = String::new();
	let mut senha: String = String::new();

    print!("Nome: ");
    std::io::stdout().flush().expect("Erro ao dar flush no terminal");
	std::io::stdin().read_line(&mut buffer).expect("Erro ao ler linha")

	print!("Senha:");
	std::io::stdout().flush().expect("Erro ao dar flush no terminal");
	std::io::stdin().read_line(&mut senha).expect("Erro ao ler linha");

    loop{
		
		if buffer[contador] == S[contador] && R[contador] == senha[contador]{
			contador = contador + 1;
		}
		
		else{
			break;
		}
		
	}
}

fn ex3{
	//"Ola mundo" -> &str, string estatica
	//String::from("Ola mundo"); -> string dinamica
	//				(String) -> (&str) -> (string)
	
	println("Digite seu usuario:");
	let mut nome_usuario = String.new();
	std::io::stdin.read_line(&mut nome_usuario).expect("Erro ao ler linha"); //ele armazena o enter no final, o /n. vai precisar limpar esse dado para não comparar junto
	nome_usuario = nome_usuario.trim().to_string();
	
	println!("Digite sua senha:")
	let mut senha_usuario = String.new();
	std::io::stdin.read_line(&mut senha_usuario).expect("Erro ao ler linha"); //ele armazena o enter no final, o /n. vai precisar limpar esse dado para não comparar junto
	nome_usuario = senha_usuario.trim().to_string();
	
	if nome_usuario.eq("admin") && senha_usuario("@adm1n"){
		println!("Deu certo");
	}
	
	else{
		println!("Deu errado");
	}
}

fn main() {
    ex3();
}

//for i in range 0..100{
//	println!("()");
//}
//Quem faz metade ou ao final da lista do TDE consegue fazer a prova
//O TDE não precisa entregar
//A prova será no papel