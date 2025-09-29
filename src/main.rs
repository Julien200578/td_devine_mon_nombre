//! # Devine mon nombre
//!
//! Ce programme est un petit jeu où l’utilisateur doit deviner
//! un nombre secret choisi aléatoirement par l’ordinateur.
//!
//! ## Fonctionnement
//! - L’ordinateur choisit un nombre aléatoire entre 1 et 100.
//! - L’utilisateur entre une valeur au clavier.
//! - Le programme indique si le nombre est trop petit, trop grand,
//!   ou si c’est le bon nombre.

use console::style;       // pour colorer le texte
use std::cmp::Ordering;
use std::io;
use rand::Rng;

/// Lit un entier depuis l’entrée standard.
///
/// Retourne `Some(entier)` si la saisie est correcte, sinon `None`.
///
/// # Examples
///
/// ```
/// let n = read_int_from_stdin();
/// assert!(n.is_none() || n.is_some());
/// ```
fn read_int_from_stdin() -> Option<u32> {
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_ok() {
        if let Ok(num) = buffer.trim().parse::<u32>() {
            return Some(num);
        }
    }
    None
}

/// Compare la saisie avec le nombre secret et retourne un `Ordering`.
fn get_ordering(secret_number: u32, input: u32) -> Ordering {
    input.cmp(&secret_number)
}

/// Affiche le message correspondant au résultat de la comparaison.
fn display_result(comparison: Ordering) {
    match comparison {
        Ordering::Less => println!("Trop petit !"),
        Ordering::Greater => println!("Trop grand !"),
        Ordering::Equal => println!("{}", style("Bravo, trouvé !").green().bold()),
    }
}

/// Retourne `true` si le joueur a trouvé le nombre.
fn has_found(comparison: Ordering) -> bool {
    matches!(comparison, Ordering::Equal)
}

fn main() {
    // Afficher le titre en bleu
    println!("{}", style("Devine mon nombre !").blue().bold());

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Affiche uniquement en mode debug
    #[cfg(debug_assertions)]
    println!("(debug) Le nombre secret est : {secret_number}");

    loop {
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

/// Module de tests unitaires
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
