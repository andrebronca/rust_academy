use std::io;

fn main() {
    println!("Advinhe o número!");

    println!("Digite um número.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha para ler a linha!");

    println!("Você acertou: {guess}");
}
