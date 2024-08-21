#[cfg(test)]
mod tests {
    // Importation du module corrigé.
    use crate::corrige::*;

    #[test]
    fn test_find_by_author() {
        // Crée une bibliothèque et y ajoute quelques livres.
        let mut library = Library::new();
        let book1 = Book::new("1984", "George Orwell");
        let book2 = Book::new("Le Petit Prince", "Antoine de Saint-Exupéry");
        let book3 = Book::new("Animal Farm", "George Orwell");
        library.add_book(book1);
        library.add_book(book2);
        library.add_book(book3);

        // Teste la recherche pour George Orwell.
        let books_by_orwell = library.find_by_author("George Orwell");
        assert_eq!(books_by_orwell.len(), 2); // Il doit y avoir deux livres.
        assert!(books_by_orwell.iter().any(|book| book.title == "1984"));
        assert!(books_by_orwell.iter().any(|book| book.title == "Animal Farm"));

        // Teste la recherche pour un auteur qui n'existe pas.
        let books_by_unknown = library.find_by_author("Auteur Inconnu");
        assert!(books_by_unknown.is_empty()); // Aucun livre ne doit être trouvé.
    }

    #[test]
    fn test_borrow_and_return_book() {
        // Crée une bibliothèque et y ajoute un livre.
        let mut library = Library::new();
        let book = Book::new("1984", "George Orwell");
        library.add_book(book);

        // Emprunte le livre.
        let borrow_result = library.borrow_by_title("1984");
        assert!(borrow_result.is_ok()); // L'emprunt doit réussir.

        // Vérifie que le livre ne peut pas être emprunté à nouveau.
        let borrow_again_result = library.borrow_by_title("1984");
        assert!(borrow_again_result.is_err()); // Le second emprunt doit échouer.

        // Retourne le livre.
        let return_result = library.return_by_title("1984");
        assert!(return_result.is_ok()); // Le retour doit réussir.

        // Vérifie que le livre peut être emprunté à nouveau après avoir été retourné.
        let borrow_after_return_result = library.borrow_by_title("1984");
        assert!(borrow_after_return_result.is_ok()); // L'emprunt doit réussir.
    }

    #[test]
    fn test_display_books() {
        // Crée une bibliothèque et y ajoute plusieurs livres.
        let mut library = Library::new();
        let book1 = Book::new("1984", "George Orwell");
        let book2 = Book::new("Le Petit Prince", "Antoine de Saint-Exupéry");
        library.add_book(book1);
        library.add_book(book2);

        // Capture la sortie de la fonction display_books.
        let output = std::panic::catch_unwind(|| {
            library.display_books();
        });

        // On vérifie simplement que l'affichage ne panique pas.
        assert!(output.is_ok());
    }
}
