//primeiros 10 números primos acima de 100
fn main() {
    let (mut x, mut y): (u32, u32) = (0, 100);
	let mut flag: bool;
    while x < 10 {
        y += 1;
        flag = false;
        for n in 2..y/2 {
            if y % n == 0 {
                flag = true;
                break;
            }
        }
        
        if flag == false {
            println!("{y}");
            x += 1;
        }
    }
}
