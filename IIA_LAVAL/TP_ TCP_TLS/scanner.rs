use std::net::TcpStream;
use std::time::Duration;
use std::io::{self, Write};

fn main() {

    //Scanne de l'adresse local avec un temps de 200 miliseconde par port
    let target_ip = "127.0.0.1";
    let timeout = Duration::from_millis(200);

    println!("Scan des ports sur {}...", target_ip);


    //Scanne de chaque port 
    for port in 1..3000 {
        

        //On construit l'adresse au format "IP:PORT" et on tente une connexion TCP.
        //L'utilisation de 'if let Ok' permet de ne traiter que les succès 
        let address = format!("{}:{}", target_ip, port);
        
        if let Ok(_stream) = TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
            println!("\n[+] Port {} est OUVERT", port);
        }


        //Pour éviter que l'écran ne reste figé, on affiche un point tous les 10 ports.
        if port % 10 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }


    println!("\nScan terminé !");
}