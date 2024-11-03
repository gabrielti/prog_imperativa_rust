

enum Carta
{
    Cartas {naipe: String, valor: String}, //estutura
    cars {naipe,valor}, //tupla
} 

enum pontuacao(carta: Carta) -> i32 {
    match carta {
        Carta::Carta{naipe,valor} => {
            match valor.as_str(){
                "Dois" => return 2;
                "Tres" => return 3;
                "Rei" => return 13;
                "Dama" => return 12;
            }
            _ => return 0;
        }
    }
}

fn main()
{
    let baralho: Vec<Carta> = vec![
        Carta::Carta{naipe: "Ouro".to_string(), valor: "Dois".to_string()},
        Carta::Carta{naipe: "Copas".to_string(), valor: "Um".to_string()},
        Carta::Carta{naipe: "Damas".to_string(), valor: "Quatro".to_string()},
    ]
}