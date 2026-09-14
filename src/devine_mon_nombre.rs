use std::io;

fn main() {
    println!("Devine mon nombre !\n");
    println!("Saisissez votre proposition.");

    let mut proposition = String::new();
    io::stdin().read_line(&mut proposition).unwrap();

    println!("Vous avez saisi : {}", proposition);
}