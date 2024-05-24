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
/// let indexed_word = word_index::CountedWord{
///     word: "Text".as_bytes().into(),
///     count: 12
/// };
///
/// assert_eq!(indexed_word.word.as_ref(), "Text".as_bytes());
/// assert_eq!(indexed_word.count, 12);
/// ```
#[derive(Debug, PartialEq)]
pub struct CountedWord {
    /// The word that's indexed.
    pub word: Box<[u8]>,
    /// The amount of times this word appeared.
    pub count: usize
}

#[derive(Debug, Default, PartialEq)]
pub struct WordStore(HashMap<Box<[u8]>, usize>);

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
/// use lib_word_count::word_index::WordStore;
///
/// let mut word_store = WordStore::new();
///
/// word_store.add_word("Hello".as_bytes());
/// word_store.add_word("hELLO".as_bytes());
/// word_store.add_word("World".as_bytes());
/// word_store.add_word("HELLO".as_bytes());
/// word_store.add_word("PFUDOR".as_bytes());
/// ```
pub fn add_word(&mut self, word: &[u8]) {

    let hash_map = &mut self.0;

    if let Some(count) = hash_map.get_mut(word) {
            *count += 1;
    } else {
            hash_map.insert(word.to_owned().into(), 1);
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
