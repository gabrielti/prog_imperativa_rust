use crate::funcoes::{Arquivo, Permissao};
use std::io::{self, Write};
use crate::utils;

static mut ARQUIVOS: Vec<Arquivo> = Vec::new();

fn criar_arquivo() {
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
    
    // Armazena o arquivo no vetor global
    unsafe {
        ARQUIVOS.push(arquivo);
    }

    println!("Arquivo '{}' criado com sucesso!", nome.trim());
}

fn alterar_permissao_arquivo() {
    let mut nome = String::new();
    println!("Digite o nome do arquivo ao qual deseja alterar a permissões:");
    io::stdin().read_line(&mut nome).expect("Falha na leitura do nome do arquivo");
    let nome = nome.trim();

    // Encontre o arquivo no vetor
    unsafe {
        if let Some(arquivo) = ARQUIVOS.iter_mut().find(|a| a.nome == nome) {
            let mut leitura = String::new();
            let mut escrita = String::new();
            let mut execucao = String::new();

            println!("Digite a permissão de leitura (0 ou 1):");
            io::stdin().read_line(&mut leitura).expect("Falha na leitura da permissão de leitura");

            println!("Digite a permissão de escrita (0 ou 1):");
            io::stdin().read_line(&mut escrita).expect("Falha na leitura da permissão de escrita");

            println!("Digite a permissão de execução (0 ou 1):");
            io::stdin().read_line(&mut execucao).expect("Falha na leitura da permissão de execução");

            let leitura: u8 = leitura.trim().parse().expect("Permissão de leitura inválida");
            let escrita: u8 = escrita.trim().parse().expect("Permissão de escrita inválida");
            let execucao: u8 = execucao.trim().parse().expect("Permissão de execução inválida");

            let nova_permissao = Permissao::new(leitura, escrita, execucao);
            arquivo.alterar_permissao(nova_permissao);

            println!("Permissões do arquivo '{}' alteradas com sucesso!", arquivo.nome);
        } else {
            println!("Arquivo '{}' não encontrado.", nome);
        }
    }
}

fn listar_arquivos() {
    unsafe {
        if ARQUIVOS.is_empty() {
            println!("Nenhum arquivo foi criado ainda.");
        } else {
            for arquivo in &ARQUIVOS {
                arquivo.stat();
            }
        }
    }
}

pub fn menu_arquivo() {
    loop {
        println!("\n=== Menu Arquivo ===");
        println!("1. Criar Arquivo");
        println!("2. Alterar Permissão de Arquivo");
        println!("3. Listar Arquivos");
        println!("4. Voltar ao Menu Principal");

        let escolha = utils::ler_input("Escolha uma opção: ");
        match escolha.as_str() {
            "1" => criar_arquivo(),
            "2" => alterar_permissao_arquivo(),
            "3" => listar_arquivos(),
            "4" => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
