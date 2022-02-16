extern crate rand;
use::std::io;
use::rand::Rng;
use::std::cmp::Ordering;
fn main() {
    loop{
        println!("Guess Number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read input");
        println!(" thank you for your Guess {}",guess);
        let random = rand::thread_rng().gen_range(10,101);
        println!("The secrete number is {}",random);
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

         match guess.cmp(&random){
            Ordering::Less => println!(" Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal =>{ println!("equal");
                break;
    }
}
}
}
