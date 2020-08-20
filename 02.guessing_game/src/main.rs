use std::io;

fn main() {
    println!("추리 게임");
    println!("당신이 추측한 숫자를 적으세요");

    let mut guess = String::new();

    io::stdin().read_line (&mut guess).expect("실패");

    println!("당신이 추측한 숫자 {}", guess);
    
}
