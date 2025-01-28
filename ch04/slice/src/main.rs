fn main() {
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s); // borrowing s immutably
    // returns reference to a slice of the string stored in s

    // s.clear(); // needs mutable borrow

    // but immutable borrow is not done
    println!("the first word is {}", word);

    println!("\n");

    let s = String::from("hello world");

    let word = flex_first_word(&s[0..6]);
    let word = flex_first_word(&s[..]);
    let word = first_word(&s);

    let s_literal = "hello world";

    let word = flex_first_word(&s_literal[0..6]);
    let word = flex_first_word(&s_literal[..]);
    let work = flex_first_word(&s_literal);
     /*
     This is because a String can be borrowed as a &str,
     and string literals are inherently &str.
      */


}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn flex_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}