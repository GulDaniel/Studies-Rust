static mut STATIC_VARIABLE: u32 = 15;

fn main() {
    let a: u32 =10;
    println!("Valor de a: {}", a);
    
    unsafe{
       let a: f32 = 20.20; 
       println!("Valor de a: {}", a);
       println!("Valor de STATIC_VARIABLE: {}", STATIC_VARIABLE);
    }
    
    println!("Valor de a: {}", a);
}
