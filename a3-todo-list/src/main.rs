use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

impl Todo {
    fn new(id: u32, title: String, description: String) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    fn complete(&mut self) {
        if !self.completed {
            self.completed = true;
            self.completed_at = Some(Utc::now());
        }
    }

    fn display(&self) {
        let status = if self.completed { "✓" } else { "○" };
        println!("\n{} [{}] {}", status, self.id, self.title);
        println!("  Description: {}", self.description);
        println!("  Créé le: {}", self.created_at.format("%d/%m/%Y %H:%M"));
        if let Some(completed_at) = self.completed_at {
            println!("  Complété le: {}", completed_at.format("%d/%m/%Y %H:%M"));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: String, description: String) {
        let todo = Todo::new(self.next_id, title, description);
        self.todos.push(todo);
        self.next_id += 1;
        println!("✓ Todo ajouté avec succès !");
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("\n📭 Aucune tâche pour le moment !");
            return;
        }

        println!("\n=== 📋 VOS TÂCHES ===");
        for todo in &self.todos {
            todo.display();
        }
    }

    fn list_incomplete(&self) {
        let incomplete: Vec<&Todo> = self.todos
            .iter()
            .filter(|todo| !todo.completed)
            .collect();

        if incomplete.is_empty() {
            println!("\n🎉 Toutes vos tâches sont complétées !");
            return;
        }

        println!("\n=== 🔄 TÂCHES EN COURS ===");
        for todo in incomplete {
            todo.display();
        }
    }

    fn complete(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            if !todo.completed {
                todo.complete();
                println!("✅ Tâche {} complétée !", id);
                true
            } else {
                println!("⚠️ Cette tâche est déjà complétée !");
                false
            }
        } else {
            println!("❌ Tâche avec l'ID {} non trouvée !", id);
            false
        }
    }

    fn delete(&mut self, id: u32) -> bool {
        let initial_len = self.todos.len();
        self.todos.retain(|todo| todo.id != id);

        if self.todos.len() < initial_len {
            println!("🗑️ Tâche {} supprimée !", id);
            true
        } else {
            println!("❌ Tâche avec l'ID {} non trouvée !", id);
            false
        }
    }

    fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(TodoList::new());
        }

        let json = fs::read_to_string(path)?;
        let todo_list: TodoList = serde_json::from_str(&json)?;
        Ok(todo_list)
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn show_help() {
    println!("\n=== 📚 COMMANDES DISPONIBLES ===");
    println!("  add                     - Ajouter une nouvelle tâche");
    println!("  list                    - Lister toutes les tâches");
    println!("  incomplete              - Lister les tâches non complétées");
    println!("  complete [ID]           - Marquer une tâche comme complétée");
    println!("  delete [ID]             - Supprimer une tâche");
    println!("  help                    - Afficher cette aide");
    println!("  quit                    - Quitter l'application");
    println!("=================================");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_file = PathBuf::from("todos.json");
    let mut todo_list = TodoList::load_from_file(&data_file)?;

    println!("\n🎯 BIENVENUE DANS VOTRE TO-DO LIST !");
    println!("Tapez 'help' pour voir les commandes disponibles.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                let title = get_input("📝 Titre de la tâche: ");
                if title.is_empty() {
                    println!("⚠️ Le titre ne peut pas être vide !");
                    continue;
                }

                let description = get_input("📄 Description (optionnel): ");
                todo_list.add(title, description);
                todo_list.save_to_file(&data_file)?;
            }

            "list" => {
                todo_list.list();
            }

            "incomplete" => {
                todo_list.list_incomplete();
            }

            "complete" => {
                if parts.len() < 2 {
                    println!("⚠️ Usage: complete [ID]");
                    continue;
                }

                if let Ok(id) = parts[1].parse::<u32>() {
                    todo_list.complete(id);
                    todo_list.save_to_file(&data_file)?;
                } else {
                    println!("❌ ID invalide !");
                }
            }

            "delete" => {
                if parts.len() < 2 {
                    println!("⚠️ Usage: delete [ID]");
                    continue;
                }

                if let Ok(id) = parts[1].parse::<u32>() {
                    todo_list.delete(id);
                    todo_list.save_to_file(&data_file)?;
                } else {
                    println!("❌ ID invalide !");
                }
            }

            "help" => {
                show_help();
            }

            "quit" | "exit" => {
                println!("\n👋 Au revoir ! À bientôt !\n");
                break;
            }

            _ => {
                println!("❌ Commande inconnue. Tapez 'help' pour voir les commandes disponibles.");
            }
        }
    }

    Ok(())
}