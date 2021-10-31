use std::io; // import statement

fn main() {
    println!("Advinhe um numero");
    println!("Digite seu palpite");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Voce disse {}", palpite);
}
