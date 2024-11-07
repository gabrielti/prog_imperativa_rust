use crate::funcoes::{Usuario, Grupo};
use crate::utils;
use std::io;

fn criar_usuario() -> Usuario {
    let nome = utils::ler_input("Digite o nome do usuário: ");
    let uid = utils::ler_input("Digite o UID do usuário: ")
        .trim()
        .parse::<u16>()
        .expect("UID inválido");

    let nome_grupo = utils::ler_input("Digite o nome do grupo principal do usuário: ");
    let gid = utils::ler_input("Digite o GID do grupo: ")
        .trim()
        .parse::<u16>()
        .expect("GID inválido");

    let grupo_principal = Grupo::new(nome_grupo.clone(), gid);
    let usuario = Usuario::new(nome.clone(), uid, grupo_principal);
    println!("Usuário '{}' criado com sucesso!", nome);

    usuario
}

fn adicionar_usuario_grupo(usuario: &mut Usuario) {
    let nome_grupo = utils::ler_input("Digite o nome do grupo a ser adicionado: ");
    let gid = utils::ler_input("Digite o GID do grupo: ")
        .trim()
        .parse::<u16>()
        .expect("GID inválido");

    let grupo = Grupo::new(nome_grupo.clone(), gid);
    usuario.adiciona_grupo(grupo);
}

fn remover_usuario_grupo(usuario: &mut Usuario) {
    let nome_grupo = utils::ler_input("Digite o nome do grupo a ser removido: ");
    usuario.remove_grupo(&nome_grupo);
}

fn listar_grupos_usuario(usuario: &Usuario) {
    usuario.listar_grupos();
}

pub fn menu_usuario() {
    let mut usuario = criar_usuario(); // Criação inicial do usuário

    loop {
        println!("\n=== Menu Usuário ===");
        println!("1. Criar Usuário");
        println!("2. Adicionar Usuário a Grupo");
        println!("3. Remover Usuário de Grupo");
        println!("4. Listar Grupos do Usuário");
        println!("5. Voltar ao Menu Principal");

        let escolha = utils::ler_input("Escolha uma opção: ");
        match escolha.as_str() {
            "1" => {
                usuario = criar_usuario();
            }
            "2" => {
                adicionar_usuario_grupo(&mut usuario);
            }
            "3" => {
                remover_usuario_grupo(&mut usuario);
            }
            "4" => {
                listar_grupos_usuario(&usuario);
            }
            "5" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
