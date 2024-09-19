fn main() {

    let s = String::from("hello world");

    let word = first_word(&s);

    //s.clear();
    println!("The first word is: {word}");
    
    let my_string = String::from("hello world");

    let word = melhoria_first_word(&my_string[0..6]);
    let word = melhoria_first_word(&my_string[..]);

    let word = melhoria_first_word(&my_string);

    let my_string_literal = "hello world";

    let word = melhoria_first_word(&my_string_literal[0..6]);
    let word = melhoria_first_word(&my_string_literal[..]);

    let word = melhoria_first_word(my_string_literal);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
fn melhoria_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item ==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
