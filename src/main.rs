fn main() {
    let nom = "Rust";
    println!("{}", saluer(nom));

    let personne = Personne::new("Rust".to_string(), 5, Etat::Actif);
    personne.saluer();
}

// saluer est une fonction qui prend un paramètre nom de type &str et retourne une String / str qui est le résultat de la fonction format!.
fn saluer(nom: &str) -> String {
    format!("Bonjour, {}!", nom)
}

// Personne est une structure qui a trois champs: nom, age et etat. Le champ etat est de type Etat qui est une énumération avec un seul membre Actif.
struct Personne {
    nom: String, // String est une chaîne de caractères allouée sur le tas.
    age: u32, // u32 est un entier non signé sur 32 bits / non signe qui signifie qu'il ne peut pas être négatif.
    etat: Etat, // Etat est une énumération.
}

// Etat est une énumération avec un seul membre Actif.
#[derive(Debug)] // Debug est un trait qui permet d'afficher le contenu de l'énumération.
enum Etat {
    Actif,
}
// impl est un mot-clé qui permet d'implémenter des méthodes pour une structure ou une énumération.
// impl Personne permet d'implémenter des méthodes pour la structure Personne.
impl Personne {
    fn new(nom: String, age: u32, etat: Etat) -> Self {
        Personne { nom, age, etat }
    }
}
// la difference entre impl et trait est que impl permet d'implémenter des méthodes pour une structure ou une énumération et trait permet de définir des méthodes pour une structure ou une énumération.
// Saluer est un trait qui permet de définir des méthodes pour une structure ou une énumération.
trait Saluer {
    fn saluer(&self);
}

// impl Saluer for Personne permet d'implémenter des méthodes pour la structure Personne.
impl Saluer for Personne {
    fn saluer(&self) {
        println!(
            "Bonjour, je suis {} et j'ai {} ans. Mon état est {:?}",
            self.nom, self.age, self.etat
        );
    }
}
