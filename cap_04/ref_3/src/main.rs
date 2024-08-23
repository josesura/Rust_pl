fn main() {
    //let reference_to_nothing = dangle();
    let reference_to_s = no_dangle();
    println!("{reference_to_s}")
}

// dangle: punteros a áreas de memoria perdidas
/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
