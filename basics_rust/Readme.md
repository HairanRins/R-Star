# Explore CLI 

un petit projet CLI qui explore les bases syntaxiques et sémantiques de Rust. 

Voici un explorateur de fichiers simple en ligne de commande qui illustre plusieurs concepts fondamentaux de Rust :

### Concepts syntaxiques et sémantiques illustrés :

1. **Structures de données** :
   - `struct FileInfo` pour organiser les données des fichiers
   - `enum Command` pour représenter les commandes disponibles

2. **Gestion des erreurs** :
   - Utilisation de `Result` et `Option`
   - Pattern matching avec `match`
   - Gestion des erreurs avec `unwrap_or_else`

3. **Ownership et emprunts** :
   - Passage de références (`&Path`, `&str`)
   - Utilisation de `String` et slices de strings

4. **Traits et implémentations** :
   - Utilisation implicite des traits pour le tri et la comparaison

5. **Closures** :
   - Fonction `format_time` définie dans `show_file_info`

6. **Entrées/sorties** :
   - Lecture du système de fichiers
   - Entrée/sortie de la console

7. **Collections** :
   - Utilisation de `Vec` pour stocker les informations des fichiers

8. **Modules** :
   - Importation et utilisation des modules standard (`std::fs`, `std::path`, etc.)

### Pour exécuter ce programme :

Exécutez avec `cargo run`

Ce programme vous permettra d'explorer votre système de fichiers en utilisant des commandes simples comme `list`, `info <fichier>`, `help` et `quit`.

![Capture d’écran du 2025-03-02 11-08-57](https://github.com/user-attachments/assets/7660ca6d-498a-4f35-afad-89bcbcecef55)

