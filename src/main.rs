extern crate  rand;
use std::io; // import statement
use std::cmp::Ordering; // import statement
use rand::Rng;

fn main() {
    println!("Advinhe um numero");
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop{        
        println!("Digite seu palpite");
    
        let mut input = String::new();
    
        io::stdin().read_line(&mut input)
            .expect("Falha ao ler entrada");
        
        let input: u32 = match input.trim().parse() {
            Ok(num)=> num,
            Err(_) => {
                println!("{} nao e um numero!", input.trim());
                continue
            },
        };
    
        println!("Voce disse {}", input);
    
        match input.cmp(&secret_number){
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Voce acertou!");
                break;
            }
        }
    }
}
