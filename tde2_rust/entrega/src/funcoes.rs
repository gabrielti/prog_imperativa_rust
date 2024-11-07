use crate::utils;

// 1. Implementação das structs do trabalho
pub struct Arquivo {
   pub nome: String,
   pub tamanho: u64,
   pub permissoes: Permissao,
   pub uid: u16,
   pub gid: u16,
}

pub struct Permissao {
    pub leitura: u8,
    pub escrita: u8,
    pub execucao: u8,
}

pub struct Grupo {
    pub nome: String,
    pub gid: u16,
    pub membros: Vec<Usuario>,
}

pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    pub grupo: Grupo,
    pub grupos: Vec<Grupo>, 
}

pub struct Diretorio {
    pub nome: String,
    pub arquivos: Vec<Arquivo>,
    pub permissoes: Permissao,
    pub dono: Usuario,
}

// 2. Implementação das funções relacionadas a cada estrutura
impl Permissao {
    pub fn new(leitura: u8, escrita: u8, execucao: u8) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    pub fn octal(&self) -> u8 {
        self.leitura * 4 + self.escrita * 2 + self.execucao * 1
    }

    pub fn padrao(&self) -> String {
        let r = if self.leitura == 1 { 'r' } else { '-' };
        let w = if self.escrita == 1 { 'w' } else { '-' };
        let x = if self.execucao == 1 { 'x' } else { '-' };
        format!("{}{}{}", r, w, x)
    }
}

impl Arquivo {
    pub fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        let permissoes = Permissao::new(0, 1, 0);
        Arquivo {
            nome,
            tamanho,
            permissoes,
            uid,
            gid,
        }
    }

    pub fn alterar_permissao(&mut self, permissao_new: Permissao) {
        self.permissoes = permissao_new;
    }

    pub fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        println!(
            "Permissões: ({}/{})",
            self.permissoes.octal(),
            self.permissoes.padrao()
        );
        println!("UID: {}", self.uid);
        println!("GID: {}", self.gid);
    }
}

impl Diretorio {
    pub fn new(nome: String, dono: Usuario) -> Diretorio { 
        let permissoes = Permissao::new(0, 1, 0);
        Diretorio {
            nome,
            permissoes,
            dono,
            arquivos: Vec::new(),
        }
    }

    pub fn adicionar_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    pub fn remove_arquivo(&mut self, nome: &str) {
        let mut found = false;
        for i in 0..self.arquivos.len() {
            if self.arquivos[i].nome == nome {
                self.arquivos.remove(i);
                println!("Arquivo '{}' foi encontrado e removido com sucesso.", nome);
                found = true;
                break;
            }
        }
        if !found {
            println!("Arquivo '{}' não encontrado.", nome);
        }
    }

    pub fn listar_conteudo(&self) {
        println!("Arquivos contidos no diretório '{}':", self.nome);
        for arquivo in &self.arquivos {
            println!("- {}", arquivo.nome);
        }
    }
}

impl Usuario {
    pub fn new(nome: String, uid: u16, primeiro_grupo: Grupo) -> Usuario {
        Usuario {
            nome,
            uid,
            grupo: primeiro_grupo,
            grupos: vec![], 
        }
    }

    pub fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupos.push(grupo);
    }

    pub fn remove_grupo(&mut self, nome: &str) {
        let mut found = false;
        for i in 0..self.grupos.len() {
            if self.grupos[i].nome == nome {
                self.grupos.remove(i);
                println!("Grupo '{}' foi encontrado e removido com sucesso.", nome);
                found = true;
                break;
            }
        }
        if !found {
            println!("Grupo '{}' não encontrado.", nome);
        }
    }

    pub fn listar_grupos(&self) {
        println!("Grupos do usuário '{}':", self.nome);
        for grupo in &self.grupos {
            println!("- {}", grupo.nome);
        }
    }
}

impl Grupo {
    pub fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros: Vec::new(),
        }
    }

    pub fn adiciona_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
        println!("Usuário adicionado ao grupo '{}'.", self.nome);
    }

    pub fn remove_membro(&mut self, nome_usuario: &str) {
        let mut found = false;
        for i in 0..self.membros.len() {
            if self.membros[i].nome == nome_usuario {
                self.membros.remove(i);
                println!("Membro '{}' removido do grupo '{}'.", nome_usuario, self.nome);
                found = true;
                break;
            }
        }
        if !found {
            println!("Membro '{}' não encontrado no grupo '{}'.", nome_usuario, self.nome);
        }
    }

    pub fn listar_membros(&self) {
        println!("Membros do grupo '{}':", self.nome);
        for membro in &self.membros {
            println!("- {}", membro.nome);
        }
    }
}
