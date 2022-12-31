/*
Usuário insere um numero inteiro positivo
e recebe a soma dos algarismos desse valor
132 -> 6 (1+2+3)
25 -> 7 (2+5)
 */
use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut soma = 0;
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Erro ao ler o valor");
    let mut valor_i32 = convert_to_int(&valor);

    while valor_i32 != 0 {
        let r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;
    }
    //132 -> 132 % 10 = i32 = 13 r = 2 soma = 2
    //13 % 10 = i32 = 1 r = 3 soma = 5
    //1 % 10 = i32 = 0 r = 1 soma = 6
    
    println!("Valor da soma dos dígitos: {}", soma);
}
