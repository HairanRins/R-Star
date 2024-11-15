fn main() {
    let x = 5; // variable immutable 
    println!("La valeur de x est: {}", x);

    let mut y = 10; // variable mutable
    println!("La valeur de y est: {}", y);
    y = 20;
    println!("La nouvelle valeur de y est: {}", y);

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Nom: {}, Âge: {}", person1.name, person1.age);

    for i in 1..=5 {
        println!("i vaut: {}", i);
    }

    // Exemple d'Ownership
    let s1 = String::from("Bonjour");
    let s2 = s1; // `s1` est maintenant invalide, `s2` en prend possession

    // println!("{}", s1); // Cela causerait une erreur, car `s1` n'est plus valide.
    println!("{}", s2); // Affiche "Bonjour"

    // Exemple de Borrowing
    let s1 = String::from("Hello");
    let s2 = &s1;  // `s2` emprunte une référence de `s1`
    println!("s1: {}, s2: {}", s1, s2);  // `s2` peut être utilisé sans prendre possession de `s1`

    // Gestion des erreurs 
    match division(10, 2) {
        Ok(result) => println!("Résultat: {}", result),
        Err(e) => println!("{}", e),
    }
}

struct Person {
    name: String,
    age: u32,
}

fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Erreur: Division par zéro"))
    } else {
        Ok(a / b)
    }
}
