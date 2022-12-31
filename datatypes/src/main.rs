fn main() {
    let x = 23; //inteiro padrão i32 (32bits)
    let y: i64 = 23; //i8 i16 i32...
    let z: u8 = 23; //unsigned (u8 0-255)
    let f: f32 = 23.5; //float seguem a mesma lógica de int
                       //existe float 32 e 64, sendo 64 padrão
    let b: bool = false; //true
    let c: char = 'c';

    println!("{} {} {} {} {} {}", x, y, z, f, b, c);
}
