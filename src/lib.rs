//! This is a library to count the words in a text.
//!
//! Words will be sorted by the amount of times they appear in a piece of text.

pub mod pivot_sort;
pub mod word_index;
use  word_index::{WordStore, CountedWord};

/// Count the appearances of each word in a string.
///
/// # Arguments
///
/// * `input`        The string containing the text you want to analyze.
///
/// * `word_index`   A reference to a vector of IndexedWords. This will be filled with the counted
///                 words.
///
/// # Examples
///
/// ```
/// let mut word_index = Vec::new();
/// let input = "I like cookies. Mmm... Cookies.";
///
/// lib_word_count::count_words(input, &mut word_index);
///
/// assert_eq!(word_index.len(), 4);
///
/// assert_eq!(word_index[0].word, "cookies".to_string());
/// assert_eq!(word_index[1].word, "i".to_string());
/// assert_eq!(word_index[2].word, "like".to_string());
/// assert_eq!(word_index[3].word, "mmm".to_string());
///
/// assert_eq!(word_index[0].appeared, 2);
/// assert_eq!(word_index[1].appeared, 1);
/// assert_eq!(word_index[2].appeared, 1);
/// assert_eq!(word_index[3].appeared, 1);
/// ```
pub fn count_words(input: &str) -> Vec<CountedWord> {

    let mut word_store = WordStore::new();
    let mut current_word = String::new();

    for character in input.chars() {

        // char is part of a word
        if character.is_alphanumeric() {
            current_word.push(character);

        // multiple non-word characters
        } else if current_word.is_empty()  {
            continue;

        // new word
        } else {
            current_word.make_ascii_lowercase();
            word_store.add_word(&current_word);
            current_word.clear();
        }
    }

    // sort the output
    word_store.sort_words()
}

