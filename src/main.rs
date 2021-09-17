use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("the secret number is {}",secret_number);
    loop {
        // println!("please input guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail to read line");
        let guess:u32 = guess.trim().parse().expect("please input integer number!");
        // println!("your guess is {}",guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal=>{println!("congraturations!");break;},
            Ordering::Greater=>println!("too big"),
            Ordering::Less=>println!("too small")
        }
    }
    
}
