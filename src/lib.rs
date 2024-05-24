//! This is a library to count the words in a text.
//!
//! Words will be sorted by the amount of times they appear in a piece of text.

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
/// let input = "I like cookies. Mmm... Cookies.";
///
/// let word_index = lib_word_count::count_words(input.bytes());
///
/// assert_eq!(word_index.len(), 4);
///
/// let words = ["i".as_bytes(), "like".as_bytes(), "mmm".as_bytes()];
///
/// assert_eq!(word_index[0].word.as_ref(), Into::<&[u8]>::into("cookies".as_bytes()));
/// assert!(words.contains(&word_index[1].word.as_ref()));
/// assert!(words.contains(&word_index[2].word.as_ref()));
/// assert!(words.contains(&word_index[3].word.as_ref()));
///
/// assert_eq!(word_index[0].count, 2);
/// assert_eq!(word_index[1].count, 1);
/// assert_eq!(word_index[2].count, 1);
/// assert_eq!(word_index[3].count, 1);
/// ```
pub fn count_words(input: impl IntoIterator<Item=u8>) -> Vec<CountedWord> {

    let mut word_store = WordStore::new();
    let mut current_word = Vec::<u8>::new();

    for character in input {

        // char is part of a word
        if character.is_ascii_alphanumeric() {
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

