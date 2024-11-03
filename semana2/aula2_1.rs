fn main() {
    let mut x = 5;
    let mut y: i32 = 10;

    x = true; //da erro porque mesmo sendo mutavel, já está sendo atribuido como inteiro 32 bits, se eu tivesse colocado somente mut x;
}
