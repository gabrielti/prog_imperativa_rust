
fn swap(x: &mut i32, y:&mut i32)
{
    let aux: i32;

    aux = *x;
    *x = *y;
    *y = aux;

}

fn main()
{
    let mut a: i32 = 10;
    let mut b: i32 = -7;

    println!("a:{a}");
    println!("b:{b}");

    swap(&mut a, &mut b);

    println!("Swap");
    println!("a:{a}");
    println!("b:{b}");
}