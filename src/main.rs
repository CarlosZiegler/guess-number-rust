extern crate  rand;
use std::io; // import statement
use std::cmp::Ordering; 
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    // create random number between 1 and 100 (gen_range includes the first and excludes the last params)
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    //https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
    loop{        
        println!("Enter a number:");
        
        // create an empty variable typed to String
        let mut input = String::new();
    
        // read input of user and reference value to input
        io::stdin().read_line(&mut input)
            .expect("Error while reading input!");
        
        // Change type input to number u32. The match return OK or Error. I think it is like an Object in Javascript, that we can use the response of parse like "key"
        let input: u32 = match input.trim().parse() {
            // if is an number that return your value.
            Ok(num)=> num,
            // this is like a catch, that not throws an error, but jump to next loop
            Err(_) => {
                println!("{} not a number!", input.trim());
                continue
            },
        };
        
        // shows user's input
        println!("your input: {}", input);
    
        // Compare input with cmp method with secret_number, that the possible result to compare number is a Enum Less, Greater or equal from Ordering
        // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
        match input.cmp(&secret_number){
            // this is an Enum from Ordering, I thik it is like an callback from result to match, or something
            Ordering::Less => println!("Number is less!"),
            Ordering::Greater => println!("Number is greater!"),
            Ordering::Equal => {
                println!("Gotcha!");
                break;
            }
        }
    }
}
