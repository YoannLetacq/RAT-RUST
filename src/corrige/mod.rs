// ============================
// Début de la section Book
// ============================

use std::fmt;

// Structure représentant un livre avec un titre, un auteur, et un statut d'emprunt.
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
}

// Implémentation des méthodes pour la structure Book.
impl Book {
    // Crée un nouveau livre avec le titre et l'auteur spécifiés. Par défaut, le livre n'est pas emprunté.
    pub fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_borrowed: false,
        }
    }
}

// Définition du trait `Borrowable`, qui assure que les structures implémentant ce trait peuvent être empruntées et retournées.
pub trait Borrowable {
    fn borrow_book(&mut self) -> Result<(), String>;
    fn return_book(&mut self) -> Result<(), String>;
}

// Implémentation du trait `Borrowable` pour la structure `Book`.
impl Borrowable for Book {
    // Emprunte un livre, en vérifiant s'il n'est pas déjà emprunté.
    fn borrow_book(&mut self) -> Result<(), String> {
        if self.is_borrowed {
            Err(format!("Le livre '{}' est déjà emprunté.", self.title))
        } else {
            self.is_borrowed = true;
            Ok(())
        }
    }

    // Retourne un livre, en vérifiant s'il est bien emprunté.
    fn return_book(&mut self) -> Result<(), String> {
        if !self.is_borrowed {
            Err(format!("Le livre '{}' n'est pas actuellement emprunté.", self.title))
        } else {
            self.is_borrowed = false;
            Ok(())
        }
    }
}

// Implémentation du trait `Display` pour `Book`, afin de permettre un affichage personnalisé d'un livre.
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Détermine le statut du livre ("Emprunté" ou "Disponible") en fonction de son état.
        let status = if self.is_borrowed { "Emprunté" } else { "Disponible" };
        // Écrit une représentation lisible du livre.
        write!(f, "'{}' par {} [{}]", self.title, self.author, status)
    }
}

// ============================
// Fin de la section Book
// ============================



// ============================
// Début de la section Library
// ============================


// Structure générique représentant une bibliothèque contenant une collection d'objets de type `T`.
pub struct Library<T: Borrowable + fmt::Display> {
    books: Vec<T>,
}

// Implémentation des méthodes pour la structure Library.
impl<T: Borrowable + fmt::Display> Library<T> {
    // Crée une nouvelle bibliothèque vide.
    pub fn new() -> Library<T> {
        Library { books: Vec::new() }
    }

    // Ajoute un livre (ou tout autre objet `T`) à la bibliothèque.
    pub fn add_book(&mut self, book: T) {
        self.books.push(book);
    }

    // Emprunte un livre par son titre. Si le livre est trouvé, il est emprunté; sinon, une erreur est retournée.
    pub fn borrow_by_title(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            // Vérifie si le titre du livre contient le titre recherché (pour simplifier l'exemple).
            if book.to_string().contains(title) {
                return book.borrow_book();
            }
        }
        Err(format!("Le livre '{}' n'est pas disponible dans la bibliothèque.", title))
    }

    // Retourne un livre par son titre. Si le livre est trouvé, il est retourné; sinon, une erreur est retournée.
    pub fn return_by_title(&mut self, title: &str) -> Result<(), String> {
        for book in &mut self.books {
            // Vérifie si le titre du livre contient le titre recherché (pour simplifier l'exemple).
            if book.to_string().contains(title) {
                return book.return_book();
            }
        }
        Err(format!("Le livre '{}' n'est pas trouvé dans la bibliothèque.", title))
    }

    // Affiche tous les livres de la bibliothèque en utilisant leur implémentation du trait `Display`.
    pub fn display_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    // Méthode pour rechercher tous les livres d'un auteur spécifique.
    pub fn find_by_author(&self, author: &str) -> Vec<&T> {
        self.books.iter()
            .filter(|book| book.to_string().contains(author))
            .collect()
    }
}

// ============================
// Fin de la section Library
// ============================



// ============================
// Début de la section Main
// ============================

pub fn main_solution() {
    // Crée une nouvelle bibliothèque vide.
    let mut library = Library::new();

    // Crée deux livres avec leurs titres et auteurs respectifs.
    let book1 = Book::new("1984", "George Orwell");
    let book2 = Book::new("Le Petit Prince", "Antoine de Saint-Exupéry");

    // Ajoute les livres à la bibliothèque.
    library.add_book(book1);
    library.add_book(book2);

    // Affiche l'état actuel des livres dans la bibliothèque.
    library.display_books();

    // Tente d'emprunter le livre "1984".
    match library.borrow_by_title("1984") {
        Ok(_) => println!("Livre emprunté avec succès."),
        Err(e) => println!("Erreur: {}", e),
    }

    // Tente d'emprunter à nouveau le même livre, ce qui devrait échouer.
    match library.borrow_by_title("1984") {
        Ok(_) => println!("Livre emprunté avec succès."),
        Err(e) => println!("Erreur: {}", e),
    }

    // Retourne le livre "1984" à la bibliothèque.
    match library.return_by_title("1984") {
        Ok(_) => println!("Livre retourné avec succès."),
        Err(e) => println!("Erreur: {}", e),
    }

    // Affiche l'état final des livres après les opérations.
    library.display_books();

    // Recherche de tous les livres d'un auteur spécifique.
    let books_by_orwell = library.find_by_author("George Orwell");
    if books_by_orwell.is_empty() {
        println!("Aucun livre trouvé pour cet auteur.");
    } else {
        println!("Livres de George Orwell :");
        for book in books_by_orwell {
            println!("{}", book);
        }
    }

    // Testez également avec un auteur qui n'a pas de livre dans la bibliothèque.
    let books_by_unknown = library.find_by_author("Auteur Inconnu");
    if books_by_unknown.is_empty() {
        println!("Aucun livre trouvé pour cet auteur.");
    } else {
        println!("Livres de Auteur Inconnu :");
        for book in books_by_unknown {
            println!("{}", book);
        }
    }
}

// ============================
// Fin de la section Main
// ============================
