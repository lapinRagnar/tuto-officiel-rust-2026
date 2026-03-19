# jeu - deviner un chiffre
## les etapes du programmation

### Setting Up a New Project

```cmd
cargo new a1_guessing_game
```

# saisir un nombre et afficher à l'ecran

``` rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

```

### Generating a Secret Number 

la librairie rust est sur https://crates.io/

ajouter rand dans cargo.toml

puis faire,

``` rust
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
```

compiler

```
cargo build
```

### pour voir les docs sur les dependances installées
```
cargo doc --open
```

## Comparing the Guess to the Secret Number

```rust 
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```



