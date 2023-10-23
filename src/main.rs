fn main() {
    let nom = "Rust";
    println!("{}", saluer(nom));

    let personne = Personne::new("Rust".to_string(), 5, Etat::Actif);
    personne.saluer();
}

fn saluer(nom: &str) -> String {
    format!("Bonjour, {}!", nom)
}

struct Personne {
    nom: String,
    age: u32,
    etat: Etat,
}
#[derive(Debug)]
enum Etat {
    Actif,
}

impl Personne {
    fn new(nom: String, age: u32, etat: Etat) -> Self {
        Personne { nom, age, etat }
    }
}

trait Saluer {
    fn saluer(&self);
}

impl Saluer for Personne {
    fn saluer(&self) {
        println!(
            "Bonjour, je suis {} et j'ai {} ans. Mon état est {:?}",
            self.nom, self.age, self.etat
        );
    }
}
