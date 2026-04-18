use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::io::{self, Write};
use std::env;

fn main() {
    // 1. On regarde ce que l'utilisateur a tapé après le nom du programme.
    // Si il n'a rien mis, on utilise "127.0.0.1" (c'est l'adresse de notre propre PC).
    let args: Vec<String> = env::args().collect();
    let target_ip = if args.len() > 1 { &args[1] } else { "127.0.0.1" };
    
    // On définit un temps d'attente : si le port ne répond pas en 200ms, on passe au suivant.
    let timeout = Duration::from_millis(200);

    println!("--- Scan de ports sur {} ---", target_ip);

    // On lance une boucle pour tester tous les ports de 1 jusqu'à 3009.
    for port in 1..3010 {
        // On fabrique l'adresse complète, par exemple "127.0.0.1:80".
        let address = format!("{}:{}", target_ip, port);
        
        // Rust a besoin que l'adresse soit dans un format spécial (SocketAddr).
        // On vérifie donc que notre texte est bien écrit.
        if let Ok(addr) = address.parse::<SocketAddr>() {
            
            // C'est l'étape magique : on tente de se connecter !
            // Si le serveur nous répond "OK", c'est que le port est ouvert.
            match TcpStream::connect_timeout(&addr, timeout) {
                Ok(_) => {
                    // Super, on a trouvé un port ouvert ! On l'affiche en gros.
                    println!("\n[+] Port {} : OUVERT", port);
                }
                Err(_) => {
                    // Si le port est fermé ou ne répond pas, on affiche juste un petit point.
                    // Ça permet de voir que le programme avance et n'est pas planté.
                    print!(".");
                    
                    // Cette ligne force l'ordinateur à afficher le point immédiatement.
                    let _ = io::stdout().flush();
                }
            }
        }
    }
    println!("\n--- Scan terminé ---");
}