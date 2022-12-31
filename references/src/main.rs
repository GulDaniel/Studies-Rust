fn main() {
    let x: u32 = 10;
    let y = &x; //não é possível realizar y+=1 

    let mut w: u32 = 10;
    let z = &mut w;
    *z += 1;

    println!("{}", y);
    println!("{}", z);
}
