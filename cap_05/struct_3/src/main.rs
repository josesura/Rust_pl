struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    
    println!("black:{0}, {1}, {2}", black.0, black.1, black.2);
    // if black == white { //Error binary operation `==` cannot be applied to type `Color`
						   // an implementation of `PartialEq<Point>` might be missing for `Color`
    if 1 == 2 {
	    println!("Iguales");
    } else {
	    println!("No Iguales");
	}
}
