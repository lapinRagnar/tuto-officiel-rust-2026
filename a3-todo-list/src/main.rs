// import de chrono
use chrono::{DateTime, Utc};


// Ligne 1: Définition d'une structure Todo ameliorée
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
    fn new(id: u32, title: String, description: String) -> Todo {
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
    }
}

fn main() {


    // tester la nouvelle structure Todo
    let mut todo = Todo::new(
        1,
        String::from("Apprendre Rust"),
        String::from("Suivre le tuto complet"),
    );

    todo.display();
    todo.complete();
    todo.display();

}


