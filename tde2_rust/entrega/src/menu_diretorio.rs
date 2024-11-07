use crate::funcoes::{Diretorio, Arquivo, Permissao, Usuario, Grupo};
use crate::utils;
use std::io::{self, Write};

fn criar_diretorio() {
    let mut nome = String::new();
    println!("Digite o nome do diretório:");
    io::stdin().read_line(&mut nome).expect("Falha na leitura do nome");

    let nome = nome.trim().to_string();

    // Criação de grupo padrão para que o usuário seja o dono
    let grupo_padrao = Grupo::new("grupo_padrao".to_string(), 0);
    let usuario_dono = Usuario::new("dono".to_string(), 1000, grupo_padrao); 

    let diretorio = Diretorio::new(nome.clone(), usuario_dono);
    println!("Diretório '{}' criado com sucesso!", nome);

}

fn adicionar_arquivo_diretorio(diretorio: &mut Diretorio) {
    let mut nome = String::new();
    let mut tamanho = String::new();
    let mut uid = String::new();
    let mut gid = String::new();

    println!("Digite o nome do arquivo:");
    io::stdin().read_line(&mut nome).expect("Falha na leitura do nome");

    println!("Digite o tamanho do arquivo (em bytes):");
    io::stdin().read_line(&mut tamanho).expect("Falha na leitura do tamanho");

    println!("Digite o UID do arquivo:");
    io::stdin().read_line(&mut uid).expect("Falha na leitura do UID");

    println!("Digite o GID do arquivo:");
    io::stdin().read_line(&mut gid).expect("Falha na leitura do GID");

    let tamanho: u64 = tamanho.trim().parse().expect("Tamanho inválido");
    let uid: u16 = uid.trim().parse().expect("UID inválido");
    let gid: u16 = gid.trim().parse().expect("GID inválido");

    let arquivo = Arquivo::new(nome.trim().to_string(), tamanho, uid, gid);
    diretorio.adicionar_arquivo(arquivo);
    println!("Arquivo '{}' adicionado ao diretório com sucesso!", nome.trim());
}

fn remover_arquivo_diretorio(diretorio: &mut Diretorio) {
    let nome = utils::ler_input("Digite o nome do arquivo a ser removido: ");
    diretorio.remove_arquivo(&nome);
}

fn listar_conteudo_diretorio(diretorio: &Diretorio) {
    diretorio.listar_conteudo();
}

pub fn menu_diretorio() {
    // Criação de um grupo e um usuário dono padrão para o diretório inicial
    let grupo_padrao = Grupo::new("grupo_padrao".to_string(), 0);
    let usuario_dono = Usuario::new("dono".to_string(), 1000, grupo_padrao);
    let mut diretorio = Diretorio::new("diretorio_exemplo".to_string(), usuario_dono);

    loop {
        println!("\n=== Menu Diretório ===");
        println!("1. Criar Diretório");
        println!("2. Adicionar Arquivo a Diretório");
        println!("3. Remover Arquivo de Diretório");
        println!("4. Listar Conteúdo do Diretório");
        println!("5. Voltar ao Menu Principal");

        let escolha = utils::ler_input("Escolha uma opção: ");
        match escolha.as_str() {
            "1" => {
                criar_diretorio();
            }
            "2" => {
                adicionar_arquivo_diretorio(&mut diretorio);
            }
            "3" => {
                remover_arquivo_diretorio(&mut diretorio);
            }
            "4" => {
                listar_conteudo_diretorio(&diretorio);
            }
            "5" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
