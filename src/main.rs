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

    let mut dest = String::new();
    let _ = io::stdin().read_to_string(&mut dest)?;

    let words = word_count::count_words(&dest);

    for indexed_word in words {
        if indexed_word.word.len() >= 5 {
            println!("'{}':\t{}", indexed_word.word, indexed_word.count);
        } else {
            println!("'{}':\t\t{}", indexed_word.word, indexed_word.count)
        }
    }

    Ok(())
}
