use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Será sorteado um número de 1 a 100.");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    loop{
        //após finalizado, não deve exibir o secret_number
        //println!("O número secreto é: {secret_number}");

        println!("Tente advinhar, digite um número.");
        //criando uma variável mutável
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha para ler a linha!");

        //convertendo string para número
        let guess: u32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            //.expect("Insira um número para funcionar!");

        //println!("Você acertou: {guess}");
        //outra forma
        println!("digitou {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!!!");
                break;
            }
        }
    }
}
