fn main() {
    let mut n1: i32 = 30;
    let mut n2: i32 = 50;

    println!("Before swap - n1: {n1}, n2: {n2}");

    // Criar referências mutáveis para n1 e n2
    let ptr1: &mut i32 = &mut n1;
    let ptr2: &mut i32 = &mut n2;
    ////cria uma variavel que aponta para o conteúdo mutavel de tipo i32 recebera o endereço de memória de n1(que é mutavel)

    // Realizar o swap manualmente usando uma variável auxiliar
    let aux = *ptr1; //operadores de referencia&(pegar o endereco) e *dereferencia(pegar o conteudo)
    *ptr1 = *ptr2; //ptr1 aponta para o conteúdo do endereço de memória n1 
    *ptr2 = aux;

    println!("After swap - n1: {n1}, n2: {n2}");

    //o conteúdo é mutavel, o endereço de memória quando a variavel é criada não é 
}

/*

o let ptr1: &mut i32 = &mut n1; cria uma variavel de tipo i32 ponteiro mutavel que recebera o endereço de memória de n1? Não entendi o que o &mut significa
O ChatGPT disse:
ChatGPT

Vamos explicar o que o &mut faz em Rust para tornar isso mais claro.
O que é &mut em Rust?

Em Rust, o &mut significa uma referência mutável. Isso é diferente de um ponteiro tradicional em C/C++, mas pode ser pensado como um "ponteiro seguro". Quando você usa &mut, está criando uma referência a uma variável que permite modificar o valor original. Em resumo:

    &mut T: Cria uma referência mutável para um valor do tipo T.
    &T: Cria uma referência imutável para um valor do tipo T.

Detalhando o exemplo

No trecho de código let ptr1: &mut i32 = &mut n1;, o que acontece é o seguinte:

    &mut n1: Isso cria uma referência mutável para a variável n1. Pense nisso como um "endereço de memória" que permite a modificação do valor apontado.

    ptr1: &mut i32: Aqui, ptr1 é uma variável do tipo "referência mutável para i32", ou seja, ela aponta para um i32 e permite modificá-lo.

Diferença entre referência mutável e ponteiro em C/C++

Em C/C++, um ponteiro mutável (int *ptr) é basicamente um endereço de memória que aponta para um valor. Você pode tanto ler quanto modificar o valor apontado por esse ponteiro.

Em Rust, &mut T é semelhante, mas com garantias de segurança:

    Safety: Rust garante que não existam referências mutáveis concorrentes para o mesmo valor. Ou seja, enquanto você tem uma &mut T, nenhuma outra referência pode modificar o valor.
    No Null: Diferente de ponteiros em C/C++, referências em Rust não podem ser nulas, eliminando uma classe inteira de bugs relacionados a ponteiros nulos.

Exemplo prático:

Vamos considerar um exemplo simples para ilustrar o uso de referências mutáveis:

rust

fn main() {
    let mut n1 = 30;
    let mut n2 = 50;

    let ptr1: &mut i32 = &mut n1;  // Cria uma referência mutável para n1
    let ptr2: &mut i32 = &mut n2;  // Cria uma referência mutável para n2

    // Agora você pode modificar n1 e n2 através de ptr1 e ptr2
    *ptr1 = *ptr1 + 10;  // Modifica o valor de n1 para 40
    *ptr2 = *ptr2 - 10;  // Modifica o valor de n2 para 40

    println!("n1: {}, n2: {}", n1, n2);  // Exibe os novos valores: n1 = 40, n2 = 40
}

Resumo:

    &mut T: Uma referência mutável em Rust, que permite modificar o valor que ela aponta.
    Ponteiros em C/C++: &mut T é semelhante a int* em C/C++, mas com as garantias de segurança de Rust.

Com isso, let ptr1: &mut i32 = &mut n1; cria uma referência mutável que pode acessar e modificar o valor de n1 de forma segura.

*/