use std::io;

// return integer_list from stdin input
fn get_word_list_from_input() -> Vec<String> {
    let mut input = String::new();

    println!("Please enter some words.");
    io::stdin().read_line(&mut input)
        .expect("failed to read_line");

    let mut word_list: Vec<String> = Vec::new();
    for word in input.split_whitespace() {
        word_list.push(word.to_string());
    }

    word_list
}

fn convert_to_pig_latin_word_list(word_list: &mut Vec<String>) {
    for word in word_list.iter_mut() {
        convert_to_pig_latin_word(word);
    }
}

fn does_start_with_vowel(word: &String) -> bool {
    word.starts_with('a')
        || word.starts_with('i')
        || word.starts_with('u')
        || word.starts_with('e')
        || word.starts_with('o')
}

// If the word start with a vowel, the word have “hay” added to the end, so “apple” becomes “apple-hay”.
// If the word start widht a consonant, the first consonant of the word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay".
fn convert_to_pig_latin_word(word: &mut String) {
    if word.len() == 0 {
        return
    }

    if does_start_with_vowel(word) {
        convert_to_pig_latin_word_vowel_case(word);
    } else {
        convert_to_pig_latin_word_consonant_case(word);
    }
}

fn convert_to_pig_latin_word_vowel_case(word: &mut String) {
    word.push_str("-hay");
}

fn convert_to_pig_latin_word_consonant_case(word: &mut String) {
    let first_consonant = word.remove(0);
    word.push('-');
    word.push(first_consonant);
    word.push_str("ay");
}

fn main() {
    let mut word_list = get_word_list_from_input();
    convert_to_pig_latin_word_list(&mut word_list);
    print!("pig latin: ");
    for word in word_list {
        print!("{word} ");
    }
    println!("");
}
