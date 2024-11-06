// 1. Implementação das structs do trabalho
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: Permissao,
    uid: u16,
    gid: u16,
}

struct Permissao {
    leitura: u8,
    escrita: u8,
    execucao: u8,
}

struct Grupo {
    nome: String,
}

struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
}

struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (bool, bool, bool),
    dono: Usuario,
}

// 2. Implementação das funções relacionadas a cada estrutura
impl Permissao {
    // Preenche a struct de permissão
    fn new(leitura: u8, escrita: u8, execucao: u8) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    // Conversão para formato octal
    fn octal(&self) -> u8 {
        // Formato ---
        // a*2² + b*2¹ + c*2⁰
        self.leitura * 4 + self.escrita * 2 + self.execucao * 1
    }

    fn padrao(&self) -> String {
        let r = if self.leitura == 1 { 'r' } else { '-' };
        let w = if self.escrita == 1 { 'w' } else { '-' };
        let x = if self.execucao == 1 { 'x' } else { '-' };
        format!("{}{}{}", r, w, x)
    }
}

impl Arquivo {
    // Preenche a struct do arquivo
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        let permissoes = Permissao::new(1, 1, 0);
        Arquivo {
            nome,
            tamanho,
            permissoes,
            uid,
            gid,
        }
    }

    // Print da info na struct no formato desejado
    fn stat(&self) {
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

fn main() {
    let arquivo = Arquivo::new("exemplo.txt".to_string(), 1024, 1000, 1000); // Acessa a função new na impl Arquivo e passa para arquivo nome, tamanho, uid, gid
    arquivo.stat();
}
