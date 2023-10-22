

fn saluer(nom: &str) -> String {
    format!("Bonjour, {}!", nom)
}

fn main() {
    let resultat = saluer("Alice");
    println!("{}", resultat); // Affiche "Bonjour, Alice!"
}
