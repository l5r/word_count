//! This module is used to index words and does the heavy lifting of our program.

use std::collections::HashMap;

/// A counted word.
///
/// This struct contains members for storing a word and the number of times it appeared. This
/// struct is intended for use with [`add_word`](fn.add_word.html).
///
/// # Examples
///
/// ```
/// use lib_word_count::word_index;
///
/// let indexed_word = word_index::IndexedWord{
///     word: "Text".to_string(),
///     appeared: 12
/// };
///
/// assert_eq!(indexed_word.word, "Text".to_string());
/// assert_eq!(indexed_word.appeared, 12i64);
/// ```
#[derive(Debug, PartialEq)]
pub struct CountedWord {
    /// The word that's indexed.
    pub word: Box<str>,
    /// The amount of times this word appeared.
    pub count: usize
}

#[derive(Debug, Default, PartialEq)]
pub struct WordStore(HashMap<Box<str>, usize>);

impl WordStore {
    pub fn new() -> WordStore {
        WordStore::default()
    }
/// Add a word to a given index.
///
/// This function prevents duplicates and increments the count of the word appearances
/// automatically. The vector will be modified accordingly.
///
/// # Arguments
///
/// * `word`    A string containing the word to add.
///
/// * `index`   A reference to a vector containing all the indexed words.
///
/// # Examples
///
/// ```
/// use lib_word_count::word_index;
///
/// let mut index = Vec::new();
///
/// word_index::add_word("Hello".to_string(), &mut index);
/// word_index::add_word("hELLO".to_string(), &mut index);
/// word_index::add_word("World".to_string(), &mut index);
/// word_index::add_word("HELLO".to_string(), &mut index);
/// word_index::add_word("PFUDOR".to_string(), &mut index);
///
/// assert_eq!(index[0], word_index::IndexedWord{
///     word: "hello".to_string(),
///     appeared: 3
/// });
/// assert_eq!(index[1], word_index::IndexedWord{
///     word: "world".to_string(),
///     appeared: 1
/// });
/// assert_eq!(index[2], word_index::IndexedWord{
///     word: "pfudor".to_string(),
///     appeared: 1
/// });
/// ```
pub fn add_word(&mut self, word: &str) {

    let hash_map = &mut self.0;

    if let Some(count) = hash_map.get_mut(word) {
            *count += 1;
    } else {
            hash_map.insert(word.to_owned().into_boxed_str(), 1);
        }
    }

pub fn sort_words(self) -> Vec<CountedWord> {
        let mut collected = self.0.into_iter()
            .map(|(word, count)| CountedWord { word, count })
            .collect::<Vec<CountedWord>>();
        collected.sort_unstable_by(|a, b| b.count.cmp(&a.count));
        collected
    }
}
