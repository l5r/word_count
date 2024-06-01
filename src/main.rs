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

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

/// This is the entry point for the program.
///
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    let words = match args.len() {
        0 => unreachable!(),
        1 => {
            let input = io::stdin().lock();
            let reader = BufReader::new(input);
            word_count::count_words(reader.bytes().map(|b| b.unwrap()))
        }
        2 => {
            let filename = &args[1];
            let input = File::open(filename)?;
            let reader = BufReader::new(input);
            word_count::count_words(reader.bytes().map(|b| b.unwrap()))
        }
        _ => {
            panic!(
                "Wrong number of arguments: {}, expected 0 to 1 arguments.",
                args.len() - 1
            );
        }
    };

    for indexed_word in words {
        if indexed_word.word.len() >= 5 {
            println!(
                "'{}':\t{}",
                std::str::from_utf8(&indexed_word.word)?,
                indexed_word.count
            );
        } else {
            println!(
                "'{}':\t\t{}",
                std::str::from_utf8(&indexed_word.word)?,
                indexed_word.count
            )
        }
    }

    Ok(())
}
