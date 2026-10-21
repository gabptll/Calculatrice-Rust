fn main() {
    loop {

        let mut history : Vec<String> = Vec::new();

        //declaration variable string pour contenir le texte
        let input = lecture_entrees();

        if input == "\n" {
            println!("ERREUR entrée vide\n");
            continue;
        }
        //.trim() enleve le \n de l'input
        //.split_whitespace() separe a chaque fois qu'il y a des espaces
        //.collect ca prends le resultat de ^ et ca le met dans un tableau dynamique "Vec"
        //.map(String::from) transforme &str en String
        //Vec<&str> -> Vec = liste, &str = morceau de texte
        let mut parts: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

        //1/ gere la separation des elements de la requete utilisateur
        let a = &parts[0];
        //gere les fonctions
        match a.as_str() {
            "help" | "h" => {
                print_help();
                continue;
            }
            "sqrt" | "racine" => {
                let input = lecture_entrees_sqrt();
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() > 1 || parts.len() < 1{
                    println!("ERREUR demande mal formulée");
                    continue;
                }
                else {
                    let mut a : f64 = 0.0;
                    match parts[0].parse() {
                        Ok(n) => a = n,
                        Err(_) => println!("PARSING ERROR"),
                    }
                    println!("{}", a.sqrt());
                }
                continue;
            }
            "quit" | "q" => break,
            "integrate" => {
                println!("IN DEV");
                clr();
            }
            "clear" | "clr" => {
                clr();
                continue;
            }
            "history" => {
                for h in &history {
                    println!("{}", h);
                }
                continue;
            }
            _ => (),
        }

        //verifie la taille de la liste pour assurer que 3 elements sont bien presents
        if parts.len() < 3 && parts.len() % 2 != 0{
            println!("ERREUR la demande n'a pas bien été formulée\n");
            continue;
        }


    

        //1/
        let mut _result : f64 = 0.0;
        let mut a1;
        let mut b1; 
        let mut op : &str;

            /*
            - check les operateur
            - si (^)
            - si (*, /, %)
            - si (+, -)
            */

            while let Some(i) = parts.iter().position(|x| x == "^") {
                op = &parts[i];
                match parsage(&parts[i-1]) {
                    Ok(nb) => {
                        a1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }

                match parsage(&parts[i+1]) {
                    Ok(nb) => {
                        b1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }

                match calcul_resultat(a1, &op, b1, &mut history) {
                    Ok(res) => {
                        parts[i] = res.to_string();
                        parts.remove(i-1);
                        parts.remove(i);
                    },
                    Err(e) => println!("ERREUR {}", e),
                }
            }
            while let Some(i) = parts.iter().position(|x| x == "*" || x == "/" || x == "%") {
                op = &parts[i];
                match parsage(&parts[i-1]) {
                    Ok(nb) => {
                        a1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }

                match parsage(&parts[i+1]) {
                    Ok(nb) => {
                        b1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }

                match calcul_resultat(a1, &op, b1, &mut history) {
                    Ok(res) => {
                        parts[i] = res.to_string();
                        parts.remove(i-1);
                        parts.remove(i);
                    },
                    Err(e) => println!("ERREUR {}", e),
                }
            }
            while let Some(i) = parts.iter().position(|x| x == "+" || x == "-") {
                op = &parts[i];
                match parsage(&parts[i-1]) {
                    Ok(nb) => {
                        a1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }

                match parsage(&parts[i+1]) {
                    Ok(nb) => {
                        b1 = nb;
                    }
                    Err(e) => {
                        println!("ERREUR {}", e);
                        continue;
                    }
                }

                match calcul_resultat(a1, &op, b1, &mut history) {
                    Ok(res) => {
                        parts[i] = res.to_string();
                        parts.remove(i-1);
                        parts.remove(i);
                    },
                    Err(e) => println!("ERREUR {}", e),
                }
            }
            println!("{}", parts[0]);
        }
    }



fn lecture_entrees() -> String{
    let mut input = String::new();
    
        //demande a l'utilisateur d'entrer un calcul
        println!("Entrez un calcul AVEC espace entre les caracteres (ou h pour l'aide) :");
    
        //prend la valeur entrée par l'utilisateur et la met dans la variable input
        use std::io::{self, Write};

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture\n");
    
    return input;

}

fn lecture_entrees_sqrt() -> String{
    let mut input = String::new();
    
        //demande a l'utilisateur d'entrer un calcul
        println!("Entrez la valeur a calculer :");
    
        //prend la valeur entrée par l'utilisateur et la met dans la variable input
        use std::io::{self, Write};

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture\n");
    
    return input;

}

fn calcul_resultat (a : f64, op : &str, b : f64, history : &mut Vec<String>) -> Result<f64, String> {
    history.push(a.to_string());
    history.push(op.to_string());
    history.push(b.to_string());
    history.push("\n".to_string());

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
            let g = f.floor();
            let h = g * b;
            (((a - h) * 10.0).round())/10.0
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

fn clr(){
    use std::process::Command;
    Command::new("clear").status().unwrap();
}

fn print_help(){
    println!("CALCULATRICE v0.1");
    println!("");
    println!("");
    println!("Pour calculer : écrivez votre équation sous le format : \"a OP b OP c ... n\"");
    println!("");
    println!("Opérateur : ");
    println!("  -\"^\" (a ^ b) renvoie a puissance b");
    println!("  -\"*\" (a * b) renvoie a fois b");
    println!("  -\"/\" (a / b) renvoie a divisé par b");
    println!("  -\"%\" (a % b) renvoie le reste de la division euclidienne de a par b");
    println!("  -\"+\" (a + b) renvoie a plus b");
    println!("  -\"-\" (a - b) renvoie a moins b");
    println!("");
    println!("La priorité est donnée a :");
    println!("- ^");
    println!("- *, /, %");
    println!("- +, -");
    println!("");
    println!("Fonctions :");
    println!("- help / h : affiche le panneau d'aide");
    println!("- quit / q : quitte le programme");
    println!("- clear / clr : efface tout le terminal");
    println!("- sqrt / racine : calcul la racine carrée d'un nombre");
    println!("");
}

fn _integrate(f : String, a : f64, b : f64, _n : i32){
    let _l = a + b;
    println!("{}", f);
}

/* si pas deja fait :
- ajouter le support des parentheses
- ajouter un historique de calcul
- refactoriser le code si besoin à la fin 
*/

