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
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};

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

    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    for w in words {
        write!(writer, "{}\t", w.count)?;
        writer.write_all(&w.word)?;
        writer.write_all(&[b'\n'])?;
    }

    writer.flush()?;

    Ok(())
}
