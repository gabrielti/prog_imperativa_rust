mod menu_arquivo;
mod menu_diretorio;
mod menu_grupo;
mod menu_usuario;
mod funcoes;
mod utils;

fn main() {
    loop {
        println!("\n===== Menu Principal =====");
        println!("1. Configuração Arquivo");
        println!("2. Configuração Diretório");
        println!("3. Configuração Grupo");
        println!("4. Configuração Usuário");
        println!("5. Sair");

        let escolha = utils::ler_input("Escolha uma opção: ");
        match escolha.as_str() {
            "1" => menu_arquivo::menu_arquivo(),
            "2" => menu_diretorio::menu_diretorio(),
            "3" => menu_grupo::menu_grupo(),
            "4" => menu_usuario::menu_usuario(),
            "5" => {
                println!("Saindo do programa...");
                break;
            }
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
