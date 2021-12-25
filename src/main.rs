extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering; 
 
fn main() {
    println!("Jogo de Advinhacao");
    let numero_secreto = rand::thread_rng().gen_range(1..10);
    
    loop {
        println!("Digite seu palpite: ");
        let mut palpite = String::new();
        io::stdin().read_line(&mut palpite).expect("Erro ao ler entrada.");
        let palpite: u32 = match palpite.trim().parse() {  
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("Se palpite {}", palpite);
            
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("valor muito baixo"),
            Ordering::Greater => println!("Valor muito grande"),
            Ordering::Equal => {
                println!("Voce acertou {}", palpite);
                break;
            }
        }
        
    }  
    
}