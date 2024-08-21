use std::fmt;

// Structure représentant un livre.
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
}

// Implémentation des méthodes pour la structure Book.
impl Book {
    // Crée un nouveau livre.
    pub fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_borrowed: false,
        }
    }

}

// Trait Borrowable pour emprunter et retourner un livre.
pub trait Borrowable {
    fn borrow_book(&mut self) -> Result<(), String>;
    fn return_book(&mut self) -> Result<(), String>;
}


impl Borrowable for Book {
    fn borrow_book(&mut self) -> Result<(), String> {
        if self.is_borrowed {
            Err(format!("Le livre '{}' est déjà emprunté.", self.title))
        } else {
            self.is_borrowed = true;
            Ok(())
        }
    }

    fn return_book(&mut self) -> Result<(), String> {
        if !self.is_borrowed {
            Err(format!("Le livre '{}' n'est pas actuellement emprunté.", self.title))
        } else {
            self.is_borrowed = false;
            Ok(())
        }
    }
}



// Implémentation du trait Display pour Book.
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.is_borrowed { "Emprunté" } else { "Disponible" };
        write!(f, "'{}' par {} [{}]", self.title, self.author, status)
    }
}

