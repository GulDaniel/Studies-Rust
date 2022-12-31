//função que retorna o dobro de um valor inteiro positivo

fn double_number(x: u32) -> u32 {
    x * 2
} 

fn sum_ten(a: u32) -> f32 {
    (a + 10) as f32 //cast
}

fn main() {
    let x = 10;
    let y = double_number(x);
    let z = sum_ten(x);
    println!("O dobro de {} é {}", x, y);
    println!("{} + 10 é igual a {}", x, z);
}
