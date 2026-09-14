use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Devine mon nombre !\n");

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    // Ligne de triche/debug (tu peux la commenter avec // pour jouer pour de vrai)
    println!("Le nombre secret est : {}", nombre_secret);

    loop {
        println!("Saisissez votre proposition.");

        let mut proposition = String::new();
        io::stdin().read_line(&mut proposition).unwrap();

        let proposition: u32 = match proposition.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        match proposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est trop petit !"),
            Ordering::Greater => println!("C'est trop grand !"),
            Ordering::Equal => {
                println!("Gagné !");
                break;
            }
        }
    }
}