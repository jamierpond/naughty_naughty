#![feature(proc_macro_hygiene, decl_macro)]

mod bad_words;

use bad_words::BAD_WORDS;

#[macro_use]
extern crate rocket;

fn contains_bad_word(text_input: &str) -> bool {
    for word in BAD_WORDS.iter() {
        if text_input.contains(word) {
            return true;
        }
    }
    return false;
}

#[test]
fn test_bad_word() {
    assert!(contains_bad_word("bum") == true);
    assert!(contains_bad_word("happy dogs \n are the best") == false);
    assert!(contains_bad_word("happy dogs \n are the best bum") == true);

    // test on all escape characters
    assert!(contains_bad_word("lemons \n \r bu m bum \\3432423\r \t \t"))
}


#[get("/")]
fn home() -> String {
    return "Bad Word Checker\ndo /ask/encoded%20string%20like%20this \
            return `true` if the string is bad or `false` if the string is clean 😎".to_string();
}

#[get("/ask/<text_input>")]
fn bad_word(text_input: String) -> String {
    println!("text_input: {}", text_input);
    if contains_bad_word(text_input.as_str()) {
        return "true".to_string();
    }
    return "false".to_string();
}


fn main() {
    rocket::ignite().mount("/", routes![home, bad_word]).launch();
}