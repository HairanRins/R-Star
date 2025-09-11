use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process;

struct FileInfo {
    name: String,
    is_dir: bool,
    size: u64,
}

enum Command {
    List,
    Info(String),
    Help,
    Quit,
    Unknown(String),
}

fn main() {
    println!("🦀 Explorateur de fichiers Rust 🦀");
    println!("Tapez 'help' pour voir les commandes disponibles.\n");

    // Obtenir le répertoire de travail actuel
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Erreur: Impossible d'accéder au répertoire courant: {}", e);
            process::exit(1);
        }
    };

    println!("Répertoire de travail: {:?}\n", current_dir);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");
        
        // Analyser la commande entrée par l'utilisateur
        let command = parse_command(&input.trim());
        
        // Traiter la commande
        match command {
            Command::List => list_directory(&current_dir),
            Command::Info(file_name) => show_file_info(&current_dir, &file_name),
            Command::Help => show_help(),
            Command::Quit => {
                println!("Au revoir!");
                break;
            },
            Command::Unknown(cmd) => {
                println!("Commande inconnue: '{}'. Tapez 'help' pour de l'aide.", cmd);
            }
        }
    }
}

// Analyse la commande entrée par l'utilisateur
fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Command::Unknown(String::from(""));
    }
    
    match parts[0] {
        "ls" | "list" => Command::List,
        "info" => {
            if parts.len() < 2 {
                println!("Usage: info <nom_fichier>");
                Command::Unknown(String::from("info"))
            } else {
                Command::Info(parts[1].to_string())
            }
        },
        "help" => Command::Help,
        "quit" | "exit" => Command::Quit,
        _ => Command::Unknown(parts[0].to_string()),
    }
}

// Affiche le contenu du répertoire actuel
fn list_directory(dir: &Path) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            // Collecte les informations sur les fichiers
            let mut files: Vec<FileInfo> = Vec::new();
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = fs::metadata(&path).unwrap_or_else(|_| {
                        eprintln!("Impossible d'obtenir les métadonnées pour {:?}", path);
                        process::exit(1);
                    });
                    
                    files.push(FileInfo {
                        name: entry.file_name().to_string_lossy().to_string(),
                        is_dir: metadata.is_dir(),
                        size: metadata.len(),
                    });
                }
            }
            
            // Trie les fichiers (répertoires d'abord, puis par nom)
            files.sort_by(|a, b| {
                if a.is_dir && !b.is_dir {
                    std::cmp::Ordering::Less
                } else if !a.is_dir && b.is_dir {
                    std::cmp::Ordering::Greater
                } else {
                    a.name.cmp(&b.name)
                }
            });
            
            println!("Contenu du répertoire:");
            for file in files {
                let file_type = if file.is_dir { "DIR" } else { "FILE" };
                println!("{:5} {:10} {}", file_type, format_size(file.size), file.name);
            }
        },
        Err(e) => {
            eprintln!("Erreur lors de la lecture du répertoire: {}", e);
        }
    }
}

// Affiche les informations détaillées sur un fichier
fn show_file_info(dir: &Path, file_name: &str) {
    let file_path = dir.join(file_name);
    
    // Utilise le type Result avec match (pas Option)
    match fs::metadata(&file_path) {
        Ok(metadata) => {
            println!("Informations sur: {}", file_name);
            println!("Type: {}", if metadata.is_dir() { "Répertoire" } else if metadata.is_file() { "Fichier" } else { "Spécial" });
            println!("Taille: {} ({} octets)", format_size(metadata.len()), metadata.len());
            
            // Fermeture (closure) pour formater une date
            let format_time = |time: std::time::SystemTime| -> String {
                match time.duration_since(std::time::UNIX_EPOCH) {
                    Ok(duration) => {
                        let secs = duration.as_secs();
                        format!("{}", secs)
                    },
                    Err(_) => String::from("Impossible de formater la date"),
                }
            };
            
            if let Ok(created) = metadata.created() {
                println!("Créé: {}", format_time(created));
            }
            
            if let Ok(modified) = metadata.modified() {
                println!("Modifié: {}", format_time(modified));
            }
        },
        Err(e) => println!("Erreur lors de l'accès au fichier '{}': {}", file_name, e),
    }
}

// Formate la taille d'un fichier en unités lisibles
fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if size >= GB {
        format!("{:.2}GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2}MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2}KB", size as f64 / KB as f64)
    } else {
        format!("{}B", size)
    }
}

// Affiche l'aide
fn show_help() {
    println!("Commandes disponibles:");
    println!("  ls, list    - Liste les fichiers du répertoire courant");
    println!("  info <file> - Affiche des informations détaillées sur un fichier");
    println!("  help        - Affiche cette aide");
    println!("  quit, exit  - Quitte le programme");
}
