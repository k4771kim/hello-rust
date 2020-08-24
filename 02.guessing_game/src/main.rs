extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("추리 게임");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("비밀 숫자는 {}", secret_number);


    println!("당신이 추측한 숫자를 적으세요");

    let mut guess = String::new();

    io::stdin().read_line (&mut guess).expect("실패");

    println!("당신이 추측한 숫자 {}", guess);
    
}
