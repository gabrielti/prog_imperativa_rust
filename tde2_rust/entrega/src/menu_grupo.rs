use crate::funcoes::{Grupo, Usuario};
use crate::utils;
use std::io;

fn criar_grupo() -> Grupo {
    let mut nome = String::new();
    let mut gid = String::new();

    println!("Digite o nome do grupo:");
    io::stdin().read_line(&mut nome).expect("Falha na leitura do nome");

    println!("Digite o GID do grupo:");
    io::stdin().read_line(&mut gid).expect("Falha na leitura do GID");

    let gid: u16 = gid.trim().parse().expect("GID inválido");
    let nome = nome.trim().to_string();

    let grupo = Grupo::new(nome.clone(), gid);
    println!("Grupo '{}' criado com sucesso!", nome);

    grupo
}

fn adicionar_membro_grupo(grupo: &mut Grupo) {
    let nome_usuario = utils::ler_input("Digite o nome do usuário a ser adicionado: ");
    let uid = utils::ler_input("Digite o UID do usuário: ")
        .trim()
        .parse::<u16>()
        .expect("UID inválido");

    // Criar um grupo padrão para o usuário
    let grupo_padrao = Grupo::new("grupo_padrao".to_string(), 0);
    let usuario = Usuario::new(nome_usuario.clone(), uid, grupo_padrao);
    grupo.adiciona_membro(usuario);
}

fn remover_membro_grupo(grupo: &mut Grupo) {
    let nome_usuario = utils::ler_input("Digite o nome do usuário a ser removido: ");
    grupo.remove_membro(&nome_usuario);
}

fn listar_membros_grupo(grupo: &Grupo) {
    grupo.listar_membros();
}

pub fn menu_grupo() {
    let mut grupo = criar_grupo(); // Criação inicial do grupo

    loop {
        println!("\n=== Menu Grupo ===");
        println!("1. Criar Grupo");
        println!("2. Adicionar Membro ao Grupo");
        println!("3. Remover Membro do Grupo");
        println!("4. Listar Membros do Grupo");
        println!("5. Voltar ao Menu Principal");

        let escolha = utils::ler_input("Escolha uma opção: ");
        match escolha.as_str() {
            "1" => {
                grupo = criar_grupo();
            }
            "2" => {
                adicionar_membro_grupo(&mut grupo);
            }
            "3" => {
                remover_membro_grupo(&mut grupo);
            }
            "4" => {
                listar_membros_grupo(&grupo);
            }
            "5" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
