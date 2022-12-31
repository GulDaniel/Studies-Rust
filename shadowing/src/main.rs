fn main() {
    let a: u32 =10;
    println!("Valor de a: {}", a);
    
    {
       let a: f32 = 20.20; 
       println!("Valor de a: {}", a);
    }
    
    println!("Valor de a: {}", a);
}
