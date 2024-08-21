### Consigne de l'Exercice Supplémentaire : Recherche de Livres par Auteur

#### Objectif
Ajouter une fonctionnalité permettant de rechercher et d'afficher tous les livres d'un auteur spécifique dans votre bibliothèque numérique. Cette fonctionnalité doit être implémentée en utilisant les structures existantes (`Book`, `Library`) et respecter les concepts fondamentaux de Rust tels que les traits, les génériques, et la gestion des erreurs.

#### Étapes à Suivre

1. **Modification de la Structure `Library`** :
   - Ajoutez une méthode `find_by_author` à la structure `Library<T>`.
   - Cette méthode doit prendre en entrée le nom d'un auteur (en tant que `&str`) et retourner une liste de références (`Vec<&T>`) vers les livres écrits par cet auteur.
   - La méthode doit fonctionner pour tout type `T` qui implémente les traits `Borrowable` et `Display`.

2. **Test de la Fonctionnalité** :
   - Ajoutez du code dans la fonction principale pour tester cette nouvelle fonctionnalité.
   - Ajoutez plusieurs livres d'auteurs différents à la bibliothèque.
   - Utilisez la méthode `find_by_author` pour rechercher et afficher tous les livres d'un auteur donné.
   - Gérez le cas où aucun livre n'est trouvé pour l'auteur recherché.

3. **Critères de Réussite** :
   - La méthode `find_by_author` doit fonctionner correctement et retourner une liste de références vers les livres correspondants.
   - Le code doit être propre, lisible et respecter les concepts de base de Rust.
   - Les résultats de la recherche doivent être affichés correctement dans la console, y compris un message approprié si aucun livre n'est trouvé pour l'auteur recherché.

#### Exemple d'Utilisation

Voici un exemple de ce à quoi pourrait ressembler le code pour tester cette fonctionnalité dans votre fonction principale :

```rust
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
#### Temps Limite : 10 Minutes
#### Bonnes pratiques :

- Utilisez des traits pour s'assurer que le type T dans Library implémente les méthodes nécessaires.
- Utilisez les références correctement pour éviter les problèmes d'ownership.
Assurez-vous que votre code est bien organisé et lisible.
Bonne chance avec l'implémentation de cette nouvelle fonctionnalité !
