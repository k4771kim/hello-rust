// const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let spaces = "   ";
    let spaces = spaces.len();
    
    //error
    // let mut spaces = "   ";
    //spaces = spaces.len();
    
    let guess: u32 = "42".parse().expect("NOT A NUMBER!");
    // let guess = "42".parse().expect("NOT A NUMBER!"); // type ERROR
    
    println!("{}",guess);
}
