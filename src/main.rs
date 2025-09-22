//! Petit jeu « Devine mon nombre ».
//!
//! Le programme choisit un nombre aléatoire et l’utilisateur doit le deviner.

use std::cmp::Ordering;
use std::io;
use rand::Rng;
use console::style;

/// Lit un entier depuis l’entrée standard.
/// Retourne `Some(nombre)` si la conversion réussit, sinon `None`.
///
/// # Examples
///
/// ```
/// use devine_mon_nombre::read_int_from_stdin;
/// let _ = read_int_from_stdin();
/// ```
pub fn read_int_from_stdin() -> Option<u32> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    input.trim().parse::<u32>().ok()
}

/// Compare le nombre secret avec la saisie.
pub fn get_ordering(secret_number: u32, input: u32) -> Ordering {
    input.cmp(&secret_number)
}

/// Affiche un message adapté au résultat de la comparaison.
pub fn display_result(comparison: Ordering) {
    match comparison {
        Ordering::Less => println!("{}", style("Trop petit !").blue()),
        Ordering::Greater => println!("{}", style("Trop grand !").red()),
        Ordering::Equal => println!("{}", style("Bravo, trouvé !").green().bold()),
    }
}

/// Retourne `true` si le nombre a été trouvé.
pub fn has_found(comparison: Ordering) -> bool {
    matches!(comparison, Ordering::Equal)
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Titre en bleu
    println!("{}", style("Devinez le nombre !").blue().bold());

    loop {
        let input = read_int_from_stdin();

        if let Some(input) = input {
            let comparison = get_ordering(secret_number, input);
            display_result(comparison);

            if has_found(comparison) {
                break;
            }
        } else {
            println!("{}", style("Saisie incorrecte").yellow());
        }
    }
}

// -----------------------
// TESTS UNITAIRES
// -----------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_equals_twelve() {
        let result = get_ordering(12, 12);
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn twelve_greater_than_five() {
        let result = get_ordering(12, 5);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn twelve_lesser_than_twenty() {
        let result = get_ordering(12, 20);
        assert_eq!(result, Ordering::Less);
    }
}
