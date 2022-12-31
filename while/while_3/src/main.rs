//programa para encontrar o maior divisor comum entre 15 e 40
fn main() {
    let mut a = 15;
    let mut b = 40;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
        
    println!("MMD de 15 e 40 é {}", a);
}

/*
tmp = 40
b = 15 % 40 = 15
a = 40

tmp = 15
b = 40 % 15 = 10
a = 15

tmp = 10
b = 15 % 10 = 5
a = 10

tmp = 5
b = 10 % 5 = 0
a = 5
*/
