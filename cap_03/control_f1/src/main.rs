fn main() {
	const LIM: i32 = 10;
	let mut i = 0;
    let result = loop {
		i += 1;
        println!("again!");
        if i>=LIM {
			println!("Fin del bucle");
			break i * 2
		}
    };
    println!("Resultado: {result}");
}
