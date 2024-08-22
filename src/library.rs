use std::fmt;
use crate::book::Borrowable;

pub struct Library<T: Borrowable + fmt::Display> {
    books: Vec<T>,
}

impl<T: Borrowable + fmt::Display> Library<T> {
    // Crée une nouvelle bibliothèque.
    pub fn new() -> Library<T> {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: T) {
        self.books.push(book);
    }

    pub fn borrow_by_title(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            if book.to_string().contains(title) {
                return book.borrow_book();
            }
        }
        Err(format!("Le livre '{}' n'est pas disponible dans la bibliothèque.", title))
    }

    pub fn return_by_title(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            if book.to_string().contains(title) {
                return book.return_book();
            }
        }
        Err(format!("Le livre '{}' n'est pas trouvé dans la bibliothèque.", title))
    }

    pub fn display_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }
}
