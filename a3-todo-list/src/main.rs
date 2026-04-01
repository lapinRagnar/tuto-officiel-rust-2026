// Ligne 1: Définition d'une structure Todo
struct Todo {
    title: String,
    completed: bool,
}

// bloc impl pour les méthodes associées à la structure Todo
impl Todo {
    // constructeur new
    fn new(title: String) -> Todo {
        Todo {
            title: title,
            completed: false, 
        }
    }

    // méthode pour marquer la tâche comme complétée
    fn complete(&mut self) {
        self.completed = true;
    }
}

fn main() {
    
    // utilisation du constructeur pour créer une nouvelle tâche
    let mut todo = Todo::new(String::from("apprendre rust"));


    // affichage de la tâche
    println!("Tache avant: {}, completée: {}", todo.title, todo.completed);

    // appel de la méthode
    todo.complete();

    println!("Tache après: {}, completée: {}", todo.title, todo.completed);
}


