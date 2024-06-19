use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

pub fn plus_moins() {
	println!("Bienvenue dans le jeu du plus ou moins !");
	
	// Variables 
	let secret = rand::thread_rng().gen_range(1..101);
	let mut nb_essais = 10;
	
	// Boucle de jeu
	loop {
		println!("\nIl vous reste {} essai(s)", nb_essais);
		
		print!("Veuillez entrer un nombre :");
		io::stdout().flush().unwrap();
		
		// L'utilisateur rentre un nombre
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Echec de la lecture de l'entrée");
	
		// On converti la chaine en entier
		let nombre: u32 = match input.trim().parse() {
			Ok(n) => n,
			Err(_) => continue,
		};

		// On compare le nombre avec le nombre secret 
		match nombre.cmp(&secret) {
			Ordering::Less => println!("C'est plus !"),
			Ordering::Greater => println!("C'est moins !"),
			Ordering::Equal => {
				println!("Gagné !");
				break;
			}
		}

		// Si tout les essais sont épuisés
		nb_essais -= 1;
		if nb_essais == 0 {
			println!("\nPerdu, tout les essais sont épuisés !");
			break;
		}
	}
}