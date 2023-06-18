#![feature(proc_macro_hygiene, decl_macro)]

mod bad_words;

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;
use bad_words::BAD_WORDS;

// allows for spaces so that 'bum' is not found in 'bumblebee'
fn contains_bad_word(text_input: &str) -> bool {
    for unsanitised_word in text_input.split_whitespace() {
        let mut word = unsanitised_word;

        let first_char = &word.chars().next().unwrap();
        if first_char.to_lowercase().to_string() == first_char.to_uppercase().to_string() {
            word = &word[1..word.len()]
        }

        let last_char = &word.chars().last().unwrap();
        if last_char.to_lowercase().to_string() == last_char.to_uppercase().to_string() {
            word = &word[..word.len() - 1]
        }

        if BAD_WORDS.contains(&word.to_lowercase().to_string().as_str()) {
            return true;
        }
    }
    return false;
}

#[test]
fn test_bad_word() {
    assert!(contains_bad_word("bum"));
    assert!(!contains_bad_word("happy dogs \n are the best"));
    assert!(contains_bad_word("happy dogs \n are the best bum"));

    // test on all escape characters
    assert!(contains_bad_word("lemons \n \r bu m bum \\3432423\r \t \t"));

    assert!(!contains_bad_word("thousand"));
    assert!(!contains_bad_word("bumblebee"));
    assert!(contains_bad_word("bla-bla (bum)"));
    assert!(contains_bad_word("Hey bum: foo"));
    assert!(contains_bad_word("Bum foobar"));
}


#[get("/")]
fn home() -> String {
    return "Bad Word Checker\ndo /ask/encoded%20string%20like%20this \
            return `true` if the string is bad or `false` if the string is clean 😎".to_string();
}

#[get("/ask/<text_input>")]
fn bad_word_get(text_input: String) -> String {
    println!("text_input: {}", text_input);
    if contains_bad_word(text_input.as_str()) {
        return "true".to_string();
    }
    return "false".to_string();
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct BadTextRequest {
    transcription: String
}

#[post("/ask", format = "json", data = "<text_input>")]
fn bad_word_post(text_input: Json<BadTextRequest>) -> String {
    println!("text_input: {:?}", text_input);
    if contains_bad_word(text_input.transcription.as_str()) {
        return "true".to_string();
    }
    return "false".to_string();
}


fn main() {
    rocket::ignite().mount("/", routes![home, bad_word_get, bad_word_post]).launch();
}