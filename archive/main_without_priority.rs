use std::io;

fn main() {
    loop {

        //declaration variable string pour contenir le texte
        let input = lecture_entrees();

        if input == "\n" {
            println!("ERREUR entrée vide\n");
            continue;
        }
        //.trim() enleve le \n de l'input
        //.split_whitespace() separe a chaque fois qu'il y a des espaces
        //.collect ca prends le resultat de ^ et ca le met dans un tableau dynamique "Vec"
        //Vec<&str> -> Vec = liste, &str = morceau de texte
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        //verifie la taille de la liste pour assurer que 3 elements sont bien presents
        if parts.len() < 3 && parts.len() % 2 != 0 && parts[0] != "q"{
            println!("ERREUR la demande n'a pas bien été formée\n");
            continue;
        }


    
        //1/ gere la separation des elements de la requete utilisateur
        let a = parts[0];
        //gere le cas ou l'utilisateur veut arreter le programme
        match a {
            "q" => break,
            _ => (),
            //ajouter les fonctions la
        }

        //1/
        let mut result : f64 = 0.0;
        let aa;
        let a1 = parts[0].parse();
        match a1 {
            Ok(nb) => aa = nb,
            Err(_) => {
                println!("ERREUR a1 non valable");
                continue;
            }
        } 
        let mut b1 = 0.0; 
        let mut op = "+";
        for i in 0..parts.len() {


            if i % 2 == 0 && i != 0{
                match parsage(parts[i]) {
                    Ok(nb) => {
                        b1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }
            }
            else {
                op = parts[i];
            }
            
            if i == 2 {
                match calcul_resultat(aa, op, b1) {
                    Ok(res) => result = res,
                    Err(e) => println!("ERREUR {}", e),
                }
            }
            if (i > 2) && (i % 2 == 0) {
                match calcul_resultat(result, op, b1) {
                    Ok(res) => result = res,
                    Err(e) => println!("ERREUR {}", e),
                }
            }
        }
        println!("{}", result);
    }
}

fn lecture_entrees() -> String{
    let mut input = String::new();
    
        //demande a l'utilisateur d'entrer un calcul
        println!("Entrez un calcul AVEC espace entre les caracteres (ou q pour quitter) :");
    
        //prend la valeur entrée par l'utilisateur et la met dans la variable input
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture\n");
    
    return input;

}

fn calcul_resultat (a : f64, op : &str, b : f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Division par 0"))
            }
            else {
                Ok(a / b)
            }
        }
        "%" => Ok(modulo(a, b)),
        "^" => Ok(a.powf(b)),
        _ => Err(String::from("Opérateur non valide")),
    }
}

fn modulo(a : f64, b : f64) -> f64{
    let c = a.floor();
    let d = b.floor();
    if c == a && d == b {
        c % d
    }
    else {
        if d == b {
            let e = a - c;
            (c % d) + e
        }
        else {
            let f : f64 = a / b;
            let g = f as i32;
            let h = g as f64 * a;
            b - h
        }   
    }
}

fn parsage(a : &str) -> Result<f64, String>{
    let b = a.parse();
    match b {
        Ok(nb) => Ok(nb),
        Err(_) => {
            Err("element non autorisé\n".to_string())
        }
    }
}

/* si pas deja fait :
- ajouter le support des parentheses et les calcul par priorité
- finir la gestion des erreur si pas deja fait
- ajouter un historique de calcul
- ajouter des commandes (help, clear, etc...)
- refactoriser le code si besoin à la fin 
- meilleur prompt cursor */