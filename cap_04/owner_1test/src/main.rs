fn main() {
    let x = 5;
    let y = x;
    println!("x:{x}, y:{y}");

    let s1 = String::from("hello");
    //let s2 = s1; // al hacer esto s1 se va del ámbito. s1 se ha MOVIDO a s2
    println!("{s1}, world!");
    { 
		let s2 = s1; 
		println!("{s2}, world!");
	}
	//println!("{s1}, world!"); // no compila
}
