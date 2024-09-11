// Here’s an example where we’re interested in the value that gets assigned to the width field, as well as the value of the whole struct in rect1:


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
/* Salida:
Running `target/debug/struct_ejemplo2`
[src/main.rs:13:16] 30 * scale = 60
[src/main.rs:17:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}

*/
