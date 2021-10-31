extern crate  rand;
use std::io; // import statement
use std::cmp::Ordering; // import statement
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop{        
        println!("Enter a number:");
    
        let mut input = String::new();
    
        io::stdin().read_line(&mut input)
            .expect("Error while reading input!");
        
        let input: u32 = match input.trim().parse() {
            Ok(num)=> num,
            Err(_) => {
                println!("{} not a number!", input.trim());
                continue
            },
        };
    
        println!("your input: {}", input);
    
        match input.cmp(&secret_number){
            Ordering::Less => println!("Number is less!"),
            Ordering::Greater => println!("Number is greater!"),
            Ordering::Equal => {
                println!("Gotcha!");
                break;
            }
        }
    }
}
