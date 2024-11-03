use std::io::Write;

fn ex1(){

    println!("Primeiro Número:");
    let mut buffer: String = String::new(); 
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 

    let number1: i32 = buffer.trim().parse().expect("erro ao converter valor"); 

    println!("Segundo Número:");
    let mut buffer: String = String::new(); 
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 

    let number2: i32 = buffer.trim().parse().expect("erro ao converter valor"); 

    println!("Operação(+,-,*,/):");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha");

    buffer = buffer.trim().to_string();

    if buffer == "+"{
        println!("Soma(numero1+numero2) = {}",number1 + number2);
    }

    else if buffer == "-"{
        println!("Subtração(numero1-numero2) = {}",number1 - number2);
    }

    else if buffer == "*"{
        println!("Multiplicação(numero1*numero2) = {}",number1*number2);
    }

    else{
        println!("Divisão(numero1/numero2) = {}",number1/number2);
    }

}

fn ex4(){ 

    println!("Medida lado:");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 

    let lado1: f32 = buffer.trim().parse().expect("erro ao converter valor"); 

    println!("Medida lado:");
    std::io::stdout().flush().expect("Erro ao dar flush no terminal");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 

    let lado2: f32 = buffer.trim().parse().expect("erro ao converter valor"); 

    println!("Medida lado:");
    std::io::stdout().flush().expect("Erro ao dar flush no terminal");
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("erro ao ler linha"); 

    let lado3: f32 = buffer.trim().parse().expect("erro ao converter valor"); 

    if lado1 == lado2 && lado2 == lado3{
        println!("triangulo equilatero");
    }

    else if lado1 == lado2 || lado1 == lado3{
        if lado2 != lado3{
            println!("triangulo isosceles");
        }
    }

    else{
        println!("triangulo escaleno")
    }

}

fn ex2(){
    // vetor estatico -> let vetor: [i32;10] = [3;4;56;3;5;8;1;0;10;900];
    // let vetor1: [i32;4];let mut buffer: String = String::new();
    // vetor dinamico Vec<i32> = vec[];

    let vetor: [i32;10] = [3,4,56,1,45,90,140,9,7,120];
    let mut maior = vetor[0];

    for &elemento in &vetor{ //para cada endereço de memória em vetor

        if elemento > maior{
            maior = elemento;
        }
        
    }
    println!("maior:{}",maior);

}

fn ex3(){
    // vetor estatico -> let vetor: [i32;10] = [3;4;56;3;5;8;1;0;10;900];
    // let vetor1: [i32;4];let mut buffer: String = String::new();
    // vetor dinamico Vec<i32> = vec[];

    let vetor: [i32;10] = [7,4,56,15,45,90,140,9,7,120];
    let mut menor = vetor[0];

    for &elemento in &vetor{

        if elemento < menor{
            menor = elemento;
        }
        
    }
    println!("menor:{}",menor);

}

fn main(){
    //ex1();
    //ex2();
    //ex3();
    ex3();
}