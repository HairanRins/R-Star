# Tauri

Tauri est un framework open‑source permettant de créer des applications de bureau (desktop) multiplateformes (Windows, macOS, Linux) en combinant un frontend web (HTML/CSS/JS) et un backend natif écrit en Rust.

---

## 1. Fondamentaux de Tauri

1. **Architecture « WebView + Rust »**

   - **Frontend** : vous développez votre interface utilisateur comme une application web classique (React, Vue, Svelte, ou même simple HTML/CSS/JS).
   - **Backend** : Tauri embarque un serveur local minimal écrit en Rust, qui communique avec votre UI via un canal sécurisé (IPC).

2. **Embeddage de WebView**

   - Tauri utilise la WebView native de chaque OS (WebKit sur macOS, WebView2 sur Windows, GTK WebKit sur Linux).
   - Contrairement à Electron, il n’emballe pas Chromium entier : l’app s’appuie sur le navigateur déjà présent, permettant des binaires très légers (généralement < 3 Mo pour l’exécutable).

3. **Sécurité renforcée**

   - **Isolation** : par défaut, le code Rust ne peut être invoqué que via des commandes explicites (API RPC).
   - **Politique de contenu** : on définit une liste blanche des domaines/ressources autorisées à charger.

4. **API Rust puissante**

   - Accès natif au système de fichiers, notifications OS, raccourcis clavier, processus, cryptographie…
   - Extensible via des plugins (communauté Tauri et crates Rust).

---

## 2. Cas d’usage typiques

- **Apps desktop légères** : éditeurs de texte, visionneuses d’images, petits utilitaires (pomodoro, gestionnaires de mots de passe).
- **Outils internes d’entreprise** : tableaux de bord embarquant des libs JS existantes, sans lourdeur d’Electron.
- **Portage d’app web** : transformer un PWA (Progressive Web App) en application bureau native, avec accès aux API système.
- **Prototypes cross‑platform** : tester rapidement une interface web en version desktop, avec contrôle fin de la sécurité et packaging simplifié.

---

## 3. Forces de Tauri

| Point fort                  | Description                                                                            |
| --------------------------- | -------------------------------------------------------------------------------------- |
| **Taille réduite**          | Exécutables très légers (< 3 Mo), installation rapide et peu de dépendances externes.  |
| **Performance**             | Rust en backend assure un faible usage mémoire et des temps de démarrage très courts.  |
| **Sécurité**                | Modèle IPC strict, politique de chargement de contenu, isolation du code natif et web. |
| **Écosystème Rust**         | Accès à tout l’écosystème crates.io (cryptographie, base de données embarquée, etc.).  |
| **Multiplateforme unifiée** | Un même projet compile pour Windows, macOS et Linux sans changement majeur de code.    |
| **Personnalisation**        | Contrôle précis du build (options de linker, stripping, compression, etc.).            |

---

## 4. Faiblesses et limites

| Limite                                  | Conséquence / Description                                                                        |
| --------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Maturité de l’écosystème**            | Moins de plugins et ressources qu’Electron ; parfois moins de documentation sur des cas avancés. |
| **Courbe d’apprentissage Rust**         | Pour les développeurs web, découvrir Rust et son système de types peut être un frein initial.    |
| **Dépendance à la WebView OS**          | Rendu et fonctionnalités peuvent varier selon la version de WebKit/WebView2 installée.           |
| **Fonctionnalités graphiques limitées** | Pas de support direct d’OpenGL/Vulkan pour interfaces 3D complexes (mais possible via plugins).  |
| **Debugging un peu plus complexe**      | Nécessite parfois de jongler entre l’inspecteur web et le debugger Rust.                         |

---

## 5. Pour démarrer rapidement

1. **Installer le CLI Tauri**

   ```bash
   cargo install tauri-cli
   ```

2. **Initialiser un projet** (avec votre framework JS favori)

   ```bash
   # ex. avec Vue
   npm create vue@latest my-app
   cd my-app
   npm install
   tauri init
   npm run tauri dev
   ```

3. **Définir vos commandes Rust**
   Dans `src-tauri/src/main.rs` :

   ```rust
   #[tauri::command]
   fn greet(name: &str) -> String {
     format!("Bonjour, {} !", name)
   }

   fn main() {
     tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![greet])
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
   }
   ```

4. **Appeler depuis le frontend**

   ```js
   import { invoke } from "@tauri-apps/api/tauri";

   async function sayHello() {
     const message = await invoke("greet", { name: "Alice" });
     console.log(message);
   }
   ```

---

En résumé, **Tauri** est un excellent choix si vous voulez une application desktop moderne, sécurisée et légère, tout en gardant la flexibilité d’un développement web.
Son principal compromis est la nécessité de monter en compétences sur Rust et de composer avec un écosystème plus jeune que celui d’Electron.
