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
    permissoes: Permissao,
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
        let permissoes = Permissao::new(0, 1, 0);
        Arquivo {
            nome,
            tamanho,
            permissoes,
            uid,
            gid,
        }
    }

    //Seção de modificação das permissões do arquivo utilizando &mut self, autoreferencia mutavel
    fn alterar_permissao(&mut self,  permissao_new: Permissao){
        self.permissoes = permissao_new;
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

impl Diretorio {
    // Preenche a struct do diretorio
    fn new(nome: String, dono: String) -> Diretorio {
        let permissoes = Permissao::new(0, 1, 0);
        Arquivo {
            nome,
            permissoes,
            dono,
            arquivos: Vec::new(), //inicializa vetor
        }
    } 
    
    // Adicionar Arquivo
    fn adicionar_arquivo(&mut self, arquivo: Arquivo){
        self.arquivos.push(arquivo); //empurra para o vetor o nome do arquivo adicionado ao diretorio
    }


    //Função para procurar o arquivo dentro do vetor e remover o nome
    fn remove_arquivo(&mut self, nome: &str) {
        // Loop para encontrar o índice do arquivo e remove-lo
        let mut found = false;
        for i in 0..self.arquivos.len() {
            if self.arquivos[i].nome == nome {
                self.arquivos.remove(i);
                println!("Arquivo '{}' removido com sucesso.", nome);
                found = true;
                break;
            }
        }

        if !found {
            println!("Arquivo '{}' não encontrado.", nome);
        }

        else{
            println!("Arquivo '{} foi encontrado e removido com sucesso", nome);
        }
    }

    //Função para listar todos os arquivos contidos no diretorio
    fn listar_conteudo(&self){
        println!("Arquivos contidos no atual diretório '{}':", self.nome);
        for arquivo in &self.arquivos {
            println!("- {}", arquivo.nome);
        }
    }
}

fn main() {
    let arquivo = Arquivo::new("exemplo.txt".to_string(), 1024, 1000, 1000); // Acessa a função new na impl Arquivo e passa para arquivo nome, tamanho, uid, gid
    arquivo.stat();
}
