use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
 file:///home/jose/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch02-00-guessing-game-tutorial.html
 https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
 */
fn main() {
    println!("¡Adivina el número!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("El número secreto es: {secret_number}");
    loop {
		println!("Por favor, introduce tu propuesta: ");
		
		//let mut guess = String::new(); // Cadena vacía
		let mut guess = String::with_capacity(3); // Reservamos espacio necesario para que no necesite recolocación
		//let mut guess = String::from("Valor inicial. "); // Probar lo de concatenación
		
		io::stdin()
			.read_line(&mut guess)		// No sobreescribe, contatena al final
			.expect("Error al leer la línea") // Tratamiento sencillo de errores
			;
		
		// como entero, haciendo shadow a la variable guess
		// let guess: u32 = guess.trim().parse().expect("Por favor, ¡Introduce un entero >= 0!!");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("¡Que sea un número!!");
				continue
			}
		};
		
		println!("Has probado {}", guess);
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Demasiado bajo!"),
			Ordering::Greater => println!("Demasiado alto!"),
			Ordering::Equal => {
				println!("¡Has ganado!!");
				break;
			}
		}
	}
}
