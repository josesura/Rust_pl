fn main() {
    let my_string = String::from("hello world");
	let mut i = 0;
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    i+=1;
    println!("word.{i}: {word}");
    let word = first_word(&my_string[..]);
    i+=1;
    println!("word.{i}: {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    i+=1;
    println!("word.{i}: {word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    i+=1;
    println!("word.{i}: {word}");
    let word = first_word(&my_string_literal[..]);
    i+=1;
    println!("word.{i}: {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    i+=1;
    println!("word.{i}: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
