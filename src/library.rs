use std::fmt;
use crate::book::Borrowable;

// Structure représentant une bibliothèque générique.
pub struct Library<T: Borrowable + fmt::Display> {
    books: Vec<T>,
}

// Implémentation des méthodes pour la structure Library.
impl<T: Borrowable + fmt::Display> Library<T> {
    // Crée une nouvelle bibliothèque.
    pub fn new() -> Library<T> {
        Library { books: Vec::new() }
    }

    // Ajoute un livre à la bibliothèque.
    pub fn add_book(&mut self, book: T) {
        self.books.push(book);
    }

    // Emprunte un livre par son titre.
    pub fn borrow_by_title(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            if book.to_string().contains(title) {
                return book.borrow_book();
            }
        }
        Err(format!("Le livre '{}' n'est pas disponible dans la bibliothèque.", title))
    }

    // Retourne un livre par son titre.
    pub fn return_by_title(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            if book.to_string().contains(title) {
                return book.return_book();
            }
        }
        Err(format!("Le livre '{}' n'est pas trouvé dans la bibliothèque.", title))
    }

    // Affiche tous les livres de la bibliothèque.
    pub fn display_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }
}
