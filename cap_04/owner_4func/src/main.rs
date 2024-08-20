fn main() {
    let s1 = String::from("hello");

	// calculate_length toma posesión de s1, por eso debe devolverla si necesitamos seguir usándola
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
   let length = s.len(); // len() returns the length of a String

   (s, length)
   //(s, s.len()) // esto no compila
}
