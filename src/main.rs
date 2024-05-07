//!
//! Main executable.
//!
//! This file compiles to an executable.
//!
//! # Panics
//!
//! Panics when it can't read from stdin.
//!
//! # Examples
//!
//! ```sh
//! $ echo 'I like cookies. Mmm... Cookies.' | word_count
//! 'cookies':	  2
//! 'mmm':		  1
//! 'like':		  1
//! 'i':		  1
//! ```

extern crate lib_word_count;
use lib_word_count as word_count;

use std::io;
use std::io::Read;

/// This is the entry point for the program.
///
fn main() -> Result<(), std::io::Error> {

    let mut word_index = Vec::new();

    let mut dest = String::new();
    let _ = io::stdin().read_to_string(&mut dest)?;

    word_count::count_words(&dest, &mut word_index);

    Ok(for indexed_word in word_index {
        if indexed_word.word.len() >= 5 {
            println!("'{}':\t{}", indexed_word.word, indexed_word.appeared);
        } else {
            println!("'{}':\t\t{}", indexed_word.word, indexed_word.appeared)
        }
    })
}
