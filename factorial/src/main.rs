use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn factorial(x: i32) -> i32 {
    if x == 1 {
        x
    }else {
        factorial(x-1) * x
    }
}

fn main() {
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Erro ao ler o valor");
    println!("O fatorial de {} é {}", convert_to_int(&valor), factorial(convert_to_int(&valor)));
}
