fn main() {
    let s1 = String::from("hello");

	// Pasamos una referencia a calculate_length, por lo que NO toma posesión de s1
	// crear una referencia: BORROWING prestar
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
