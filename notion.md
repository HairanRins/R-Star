# Les bases de Rust

### 1. Variables et Mutabilité
En Rust, les variables sont immutables par défaut. Cela signifie qu'une fois qu'une variable est assignée à une valeur, elle ne peut pas être modifiée, sauf si elle est explicitement déclarée comme mutable.

Exemple : 

```
fn main() {
    let x = 5; // Variable immuable
    println!("La valeur de x est: {}", x);

    let mut y = 10; // Variable mutable
    println!("La valeur de y est: {}", y);
    y = 20;
    println!("La nouvelle valeur de y est: {}", y);
}
```

* ``let x = 5;`` crée une variable immuable.
* ``let mut y = 10;``crée une variable mutable, ce qui permet de changer sa valeur après sa déclaration.
* Rust garantit la sécurité de la mémoire en ne permettant pas la modification accidentelle des valeurs immutables.

### 2. Fonctions

Elles définies avec le mot-clé ``fn``.
Les paramètres sont passés par valeur (par défaut) et les résultats peuvent être retournés avec return ou implicitement.

Exemple : 

```
fn addition(a: i32, b: i32) -> i32 {
    a + b // Rust retourne la dernière expression implicitement
}

fn main() {
    let result = addition(5, 3);
    println!("Le résultat de l'addition est: {}", result);
}
```

* La fonction ``addition`` prend deux entiers en paramètres et retourne leur somme.
* Le type de retour de la fonction est spécifié après ``->``, ici i32 (un entier de 32 bits).
* La dernière expression dans une fonction en Rust est retournée implicitement, donc ``a + b`` est renvoyée sans avoir besoin du mot-clé ``return``.

### 3. Structures (Structs)

Les structures en Rust sont des types personnalisés qui permettent de regrouper plusieurs valeurs. Elles sont similaires aux ``classes`` dans d'autres langages, mais ne supportent pas l'héritage.

Exemple :

```
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Nom: {}, Âge: {}", person1.name, person1.age);
}
```

* Une ``struct`` est définie avec ``struct`` suivi du nom et des champs.
* Ici, ``Person`` est une structure qui contient un ``name`` (de type String) et un ``age`` (de type u32).
* Une instance de ``Person`` est créée dans ``main`` et ses champs sont accédés à l'aide de la notation pointée (``person1.name``).

### 4. Contrôle de flux 
Rust propose les constructions classiques de contrôle de flux : `if`, `else`, et les boucles `loop`, `for`, et `while`.

Exemple (if-else) :

```
fn main() {
    let x = 5;
    
    if x < 10 {
        println!("x est inférieur à 10");
    } else {
        println!("x est supérieur ou égal à 10");
    }
}
```

* L'instruction `if` permet de vérifier une condition, et si elle est vraie, le bloc correspondant est exécuté.
* Rust nécessite que toutes les branches de l'`if` retournent un type compatible, ce qui améliore la sécurité.

Exemple (boucle for) :

```
fn main() {
    for i in 1..=5 {
        println!("i vaut: {}", i);
    }
}
```

* La boucle `for` en Rust est utilisée pour itérer sur des plages de valeurs.
* `1..=5` définit un intervalle fermé allant de 1 à 5 inclus.

### 5. Gestion de la mémoire avec Ownership et Borrowing

L'un des aspects les plus puissants de Rust est son système de gestion de la mémoire basé sur **ownership** et **borrowing**, qui garantit qu'il n'y a pas de fuites de mémoire ni de références nulles.

Exemple d'Ownership :

```
fn main() {
    let s1 = String::from("Bonjour");
    let s2 = s1; // `s1` est maintenant invalide, `s2` en prend possession

    // println!("{}", s1); // Cela causerait une erreur, car `s1` n'est plus valide.
    println!("{}", s2); // Affiche "Bonjour"
}
```

* La variable `s1` possède la chaîne `"Bonjour"`. Lorsque `s1` est assignée à `s2`, l'ownership de la chaîne est transféré à `s2`, et `s1` devient invalide.
* Ce système garantit que la mémoire est libérée une fois que l'ownership est perdu, évitant ainsi les fuites de mémoire.

Exemple de Borrowing :

```
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; // `s2` emprunte une référence de `s1`

    println!("s1: {}, s2: {}", s1, s2); // `s2` peut être utilisé sans prendre possession de `s1`
}
```

* `s2` emprunte une référence de `s1` (avec `&`). Cela signifie que `s2` peut lire `s1` sans en prendre l'ownership.

### 6. Gestion des erreurs
Rust utilise un modèle basé sur ``Result`` pour la gestion des erreurs.
Cela permet de manipuler les erreurs de manière explicite et sûre.

```
fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Erreur: Division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match division(10, 2) {
        Ok(result) => println!("Résultat: {}", result),
        Err(e) => println!("{}", e),
    }
}
```

* La fonction `division` retourne un `Result`, qui peut être soit `Ok` avec le résultat, soit `Err` avec un message d'erreur.
* Le bloc `match` est utilisé pour gérer les deux cas possibles du `Result`.
