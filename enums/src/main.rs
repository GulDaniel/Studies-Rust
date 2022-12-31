enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn movement(dir: Direction) {
    match dir{
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Left => println!("O jogador foi para a esquerda"),
        Direction::Right => println!("O jogador foi para a direita")
    }
}

fn main() {
    let player1: Direction = Direction::Up;
    let player2: Direction = Direction::Down;
    let player3: Direction = Direction::Left;
    let player4: Direction = Direction::Right;

    movement(player1);
    movement(player2);
    movement(player3);
    movement(player4);

}
