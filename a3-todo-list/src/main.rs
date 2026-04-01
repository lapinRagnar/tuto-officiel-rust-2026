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

    // méthode pour afficher les détails de la tâche
    fn display(&self, id: usize) {
        let status = if self.completed { "✓" } else { "o"};
        println!("{} [{}] {}", status, id, self.title);
    }
}

fn main() {
    /* 
    // utilisation du constructeur pour créer une nouvelle tâche
    let mut todo = Todo::new(String::from("apprendre rust"));

        println!("Tache avant: {}, completée: {}", todo.title, todo.completed);

    // appel de la méthode
    todo.complete();

    println!("Tache après: {}, completée: {}", todo.title, todo.completed);

   */

    // creation d'un vecteur todos
    let mut todos: Vec<Todo> = Vec::new();

    // ajout de tâches au vecteur
    todos.push(Todo::new(String::from("apprendre rust")));
    todos.push(Todo::new(String::from("creer un todo list")));

    // affichage de toutes les tâches
    println!("\n === Liste des tâches ===");

    for (i, todo) in todos.iter_mut().enumerate() {
        todo.display(i + 1);
    }

    // marquer la première tâche comme complétée
    if let Some(todo) = todos.get_mut(0) {
        todo.complete();
    }

    // reaffichage après modification

        println!("\n === Liste des tâches - Modifiée ===");

    for (i, todo) in todos.iter_mut().enumerate() {
        todo.display(i + 1);
    }

}


