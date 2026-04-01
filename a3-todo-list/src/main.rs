// Ligne 1: Définition d'une structure Todo
struct Todo {
    title: String,
    completed: bool,
}

fn main() {
    // création d'une instance de Todo
    let todo = Todo {
        title: String::from("Apprendre rust"),
        completed: false,
    };

    // affichage de la tâche
    println!("Tache: {}, completée: {}", todo.title, todo.completed);
}


