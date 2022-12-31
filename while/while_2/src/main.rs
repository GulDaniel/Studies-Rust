/* Programa que recebe médias dos alunos
   e retorna a quantidade que está 
   em recuperação (3 >= nota < 6)
 */
use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut medias = String::new();
    io::stdin().read_line(&mut medias).expect("Erro ao ler valor");
    let mut soma_rec = 0;
    let mut i = 0;

    while convert_to_int(&medias) > i {
        let mut media_aluno = String::new();
        io::stdin().read_line(&mut media_aluno).expect("Erro ao ler valor");
        i+=1;

        if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6 {
            soma_rec += 1;
        }
    }

    println!("{} alunos ficaram em recuperação.", soma_rec);
}
