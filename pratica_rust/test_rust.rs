
fn main() {
    let mut array: [i32; 3] = [0, 3, 4];
    let mut count: usize = 0; // Usar usize para indexar o array
    
    while count < 3 {
        println!("array[{}] + 3 = {}", count, array[count] + 3);
        count += 1;
    }
}

