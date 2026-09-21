//! Jeu « Devine mon nombre ».

use console::style;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Lit un entier `u32` sur l'entrée standard en ignorant les erreurs.
fn read_int_from_stdin() -> Option<u32> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    input.trim().parse::<u32>().ok()
}

/// Compare la proposition au nombre secret.
fn get_ordering(secret_number: u32, input: u32) -> Ordering {
    input.cmp(&secret_number)
}

/// Affiche le message associé au résultat de la comparaison.
fn display_result(comparison: Ordering) {
    match comparison {
        Ordering::Less => println!("C'est plus grand !"),
        Ordering::Greater => println!("C'est plus petit !"),
        Ordering::Equal => println!("Félicitations, vous avez trouvé !"),
    }
}

/// Indique si le nombre secret a été découvert.
///
/// # Examples
///
/// ```
/// use std::cmp::Ordering;
/// assert!(has_found(Ordering::Equal));
/// ```
fn has_found(comparison: Ordering) -> bool {
    comparison == Ordering::Equal
}

fn main() {
    println!("{}", style("=== Jeu : Devine mon nombre ===").blue());

    let secret_number = rand::thread_rng().gen_range(1..=100);

    if cfg!(debug_assertions) {
        println!("[DEBUG] Nombre secret : {secret_number}");
    }

    loop {
        println!("Veuillez entrer un nombre :");
        let input = read_int_from_stdin();

        if let Some(input) = input {
            let comparison = get_ordering(secret_number, input);
            display_result(comparison);

            if has_found(comparison) {
                break;
            }
        } else {
            println!("Saisie incorrecte");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_equals_twelve() {
        assert_eq!(get_ordering(12, 12), Ordering::Equal);
    }

    #[test]
    fn twelve_greater_than_five() {
        assert_eq!(get_ordering(5, 12), Ordering::Greater);
    }

    #[test]
    fn twelve_lesser_than_twenty() {
        assert_eq!(get_ordering(20, 12), Ordering::Less);
    }
}