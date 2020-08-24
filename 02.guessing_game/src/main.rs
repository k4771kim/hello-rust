extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("추리 게임");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop{
        println!("당신이 추측한 숫자를 적으세요");

        let mut guess = String::new();
    
        io::stdin().read_line (&mut guess).expect("실패");
    
        let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    
    
        println!("당신이 추측한 숫자 {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("너무 작아요!"),
            Ordering::Greater => println!("너무 커요!"),
            Ordering::Equal   => {
                println!("당신의 승리입니다!");
                break;
            },
        }
    }
}