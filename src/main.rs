#![feature(proc_macro_hygiene, decl_macro)]

mod bad_words;

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;
use bad_words::BAD_WORDS;

// allows for spaces so that 'bum' is not found in 'bumblebee'
fn contains_bad_word(text_input: &str) -> Vec<String> {
    let mut results = vec![];
    for unsanitised_word in text_input.split_whitespace() {
        let mut word = unsanitised_word;

        let first_char = &word.chars().next().unwrap();
        if first_char.to_lowercase().to_string() == first_char.to_uppercase().to_string() {
            word = &word[1..word.len()]
        }
        if word.chars().last().is_none() {
            continue;
        }
        let last_char = &word.chars().last().unwrap();
        if last_char.to_lowercase().to_string() == last_char.to_uppercase().to_string() {
            word = &word[..word.len() - 1]
        }

        if BAD_WORDS.contains(&word.to_lowercase().to_string().as_str()) {
            results.push(word.to_string());
        }
    }

    return results;
}

#[test]
fn test_bad_word() {
    assert!(contains_bad_word("bum").len() == 1);
    assert!(!contains_bad_word("happy dogs \n are the best").len() == 0);
    assert!(contains_bad_word("happy dogs \n are the best bum").len() == 1);
    assert!(!contains_bad_word(" test").len() == 0);
    assert!(!contains_bad_word("test ").len() == 0);
    assert!(!contains_bad_word("test    test").len() == 0);

    // test on all escape characters
    assert!(contains_bad_word("lemons \n \r bu m bum \\3432423\r \t \t").len() == 1);

    assert!(!contains_bad_word("thousand").len() == 0);
    assert!(!contains_bad_word("bumblebee").len() == 0);
    assert!(contains_bad_word("bla-bla (bum)").len() == 1);
    assert!(contains_bad_word("Hey bum: foo").len() == 1);
    assert!(contains_bad_word("Bum foobar").len() == 1);
    assert!(contains_bad_word("Bum bum").len() == 2);
}


#[get("/")]
fn home() -> String {
    return "Bad Word Checker\ndo /ask/encoded%20string%20like%20this \
            return `true` if the string is bad or `false` if the string is clean 😎".to_string();
}

#[get("/ask/<text_input>")]
fn bad_word_get(text_input: String) -> Json<Vec<String>> {
    println!("text_input: {}", text_input);

    return rocket_contrib::json::Json(contains_bad_word(text_input.as_str()))
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct BadTextRequest {
    transcription: String
}

#[post("/ask", format = "json", data = "<text_input>")]
fn bad_word_post(text_input: Json<BadTextRequest>) -> Json<Vec<String>> {
    println!("text_input: {:?}", text_input);

    return rocket_contrib::json::Json(contains_bad_word(text_input.transcription.as_str()))
}


fn main() {
    rocket::ignite().mount("/", routes![home, bad_word_get, bad_word_post]).launch();
}