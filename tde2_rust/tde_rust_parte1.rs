
struct Arquivo{
    nome: String,
    tamanho: u64,
    permissoes:(bool,bool,bool),
    uid:u16,
    gid:u16,
}

struct Permissao{
    leitura: u8,
    escrita: u8,
    execucao: u8,
}

//todas as funcoes associadas a struct Arquivo seram implementados dentro dessa funcao, é uma correlação unica
impl Permissao{

    //preenche a struct arquivo 
    fn new(leitura: u8, escrita: u8, leitura:u8) -> Arquivo{
        Permissao{
            leitura,
            escrita,
            execucao
        }
    }

    //conversao para formato octal cada uma das posicoes --- --- --- 
    fn octal(&self) -> u8{
        //formato --- 
        //a*2²+b*2¹+c*2⁰
        self.leitura*4 + self.escrita*2  + self*1
    }

    fn padrao(&self) -> String{
        let r = if self.leitura == 1 {'r'} else {'-'};
        let w = if self.escrita == 1 {'w'} else {'-'};
        let x = if self.execucao == 1 {'x'} else {'-'};
        format!("{}{}{}",r,w,x) 
    }
}


impl Arquivo{
    fn new(nome:String ,tamanho:u64,uid:u16, gid:u16) -> Arquivo{
        Permissao{
            nome,
            tamanho,
            permissao: Permissao.new(1,1,0),
            uid,
            gid,
        }
    }

    fn stat(&self){
        println!("Arquivo:{self.nome}");
        println!("Tamanho:{self.tamanho}");
        println!("Permissões: ({self.Permissao.octal()}/{self.Permissao.padrao()})");
        println!("UID: {self.uid}");
        println!("GID: {self.gid}");
    }

}

fn main(){
    let arquivo = Arquivo::new("exemplo.txt".to_string(), 1024, 1000, 1000); //Passa para arquivo nome, tamanho,uid,gid
    arquivo.stat();
}