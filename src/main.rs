use std::io;

fn chiffrement_mot_passe(mot_passe: &str, decalage: u8) -> String {

    let mut resultat = String::new();
    
    for c in mot_passe.chars() {
        match c {
            'a'..='z' => {
                let code = (c as u8 - b'a' + decalage) % 26 + b'a';
                resultat.push(code as char);
            }
            'A'..='Z' => {
                let code = (c as u8 - b'A' + decalage) % 26 + b'A';
                resultat.push(code as char);
            }
            _ => resultat.push(c),
        }
    }
    resultat
}

fn main(){

    println!("Entrez votre mot de passe à faire chiffrer : ");
    let mut mot_passe = String::new();
    io::stdin()
        .read_line(&mut mot_passe)
        .expect("Erreur lors de la lecture du mot de passe");
    let mot_passe = mot_passe.trim();

    let mut result = String::new();

    for c in mot_passe.chars() {
        match c {
            '0'..='9' => {
                let num = (c as u8 - b'0' - 6) % 10;
                result.push(std::char::from_digit(num as u32, 10).unwrap());
            }
            'a'..='z' => {
                let code = (c as u8 - b'a' - 3) % 26 + b'a';
                result.push(code as char);
            }
            'A'..='Z' => {
                let code = (c as u8 - b'A' + 8) % 26 + b'A';
                result.push(code as char);
            }
            _ => result.push(c),
        }
    }
    result.push('#');
    println!("Le mot de passe chiffré est : {:?}", result);
}
