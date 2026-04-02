// import de chrono 
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


// Ligne 1: Définition d'une structure Todo ameliorée

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

// bloc impl pour les méthodes associées à la structure Todo
impl Todo {
    // constructeur new avec plus de champs
    fn new(id: u32, title: String, description: String) -> Self {
        Todo {
            id: id,
            title: title,
            description: description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    // méthode pour marquer la tâche comme complétée ameliorée
    fn complete(&mut self) {
        if !self.completed {
            self.completed = true;
            self.completed_at = Some(Utc::now());
        }
    }

    // méthode pour afficher les détails de la tâche ameliorée
    fn display(&self) {
        let status = if self.completed { "✓" } else { "o"};
        println!("\n {} [{}] {}", status, self.id, self.title);
        println!("   Description: {}", self.description);
        println!("   Créée le: {}", self.created_at.format("%d/%m/%Y %H:%M:%S"));
    
        if let Some(completed_at) = self.completed_at {
            println!("  Complété le: {}", completed_at.format("%d/%m/%Y %H:%M"));
        }
    }
}

// Lignes 52-58: Structure TodoList
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}


impl TodoList {
    // Lignes 61-65: Constructeur
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }
    
    // Lignes 68-74: Méthode add
    fn add(&mut self, title: String, description: String) {
        let todo = Todo::new(self.next_id, title, description);
        self.todos.push(todo);
        self.next_id += 1;
        println!("Todo ajouté avec succès !");
    }
    
    // Lignes 77-87: Méthode list
    fn list(&self) {
        if self.todos.is_empty() {
            println!("\nAucune tâche pour le moment !");
            return;
        }
        
        println!("\n=== VOS TÂCHES ===");
        for todo in &self.todos {
            todo.display();
        }
    }
    
    // Lignes 90-103: Méthode complete
    fn complete(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            if !todo.completed {
                todo.complete();
                println!("Tâche {} complétée !", id);
                true
            } else {
                println!("Cette tâche est déjà complétée !");
                false
            }
        } else {
            println!("Tâche avec l'ID {} non trouvée !", id);
            false
        }
    }
    
    // Lignes 106-115: Méthode delete
    #[allow(dead_code)]
    fn delete(&mut self, id: u32) -> bool {
        let initial_len = self.todos.len();
        self.todos.retain(|todo| todo.id != id);
        
        if self.todos.len() < initial_len {
            println!("Tâche {} supprimée !", id);
            true
        } else {
            println!("Tâche avec l'ID {} non trouvée !", id);
            false
        }
    }


    // Méthode save_to_file
    
    #[allow(dead_code)]
    fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Ligne: Convertir en JSON
        let json = serde_json::to_string_pretty(self)?;
        // Ligne: Écrire dans le fichier
        fs::write(path, json)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Ligne: Vérifier si le fichier existe
        if !path.exists() {
            return Ok(TodoList::new());
        }
        
        // Ligne: Lire le fichier
        let json = fs::read_to_string(path)?;
        // Ligne: Désérialiser
        let todo_list: TodoList = serde_json::from_str(&json)?;
        Ok(todo_list)
    }


}


fn main() {


    // Lignes 122-123: Test
    let mut todo_list = TodoList::new();
    
    todo_list.add(String::from("Apprendre Rust"), String::from("Suivre le tutoriel"));
    todo_list.add(String::from("Créer une app"), String::from("Développer la todo list"));
    
    todo_list.list();
    
    todo_list.complete(1);
    
    todo_list.list();

}


