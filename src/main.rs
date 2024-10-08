mod book;
mod corrige;
mod library;
mod library_test;

use book::Book;
use library::Library;

fn main() {
    let mut library = Library::new();

    let book1 = Book::new("1984", "George Orwell");
    let book2 = Book::new("Le Petit Prince", "Antoine de Saint-Exupéry");

    library.add_book(book1);
    library.add_book(book2);

    library.display_books();

    match library.borrow_by_title("1984") {
        Ok(_) => println!("Livre emprunté avec succès."),
        Err(e) => println!("Erreur: {}", e),
    }

    match library.borrow_by_title("1984") {
        Ok(_) => println!("Livre emprunté avec succès."),
        Err(e) => println!("Erreur: {}", e),
    }

    match library.return_by_title("1984") {
        Ok(_) => println!("Livre retourné avec succès."),
        Err(e) => println!("Erreur: {}", e),
    }

    library.display_books();

    // solution exos
    //corrige::main_solution();
}
