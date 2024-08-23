fn main() {
    let s = String::from("hello");
	// Referencia
    intenta_change(&s);
    
    // Referencia mutable. Sólo admite una
    let mut ss = String::from("hello");
    change(& mut ss);
    println!("Ahora ss es {ss}");

    // no se permite más que una referencia mutable declarada como variable
    let _r1 = & mut ss; // empiezan por _ para evitar warnings al comentar la linea println!...
    let _r2 = & mut ss;
    // Error al tratar de usarlas
    println!("{}, {}", _r1, _r2);
}

fn intenta_change(some_string: &String) {
	// No se puede porque s no es mutable
    //some_string.push_str(", world");
    println!("Ahora some_string es {some_string}");
}

// Se hace así
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    //println!("Ahora some_string es {some_string}");
}

