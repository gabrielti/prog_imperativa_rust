
fn media_aritmetica(vetor: [i32; 7] -> f64)
{
    let mut soma:f64 = 0.0;
    let n: f64 = vetor.len() as f64;

    for i in vetor
    {
        soma = soma + i as f64;
    }

}


fn main()
{
    let v:[i32. 7] = [1,2,3,4,5,6,7];
    let media: f64 = media_aritmetica(v); 
}