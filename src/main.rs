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
use std::time::{Duration, Instant, SystemTime};

/// This is the entry point for the program.
///
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    let (input_file, output_file) = match args.len() {
        0 => unreachable!(),
        1 => (None, None),
        2 => (Some(&args[1]), None),
        3 => (Some(&args[1]), Some(&args[2])),
        _ => {
            panic!(
                "Wrong number of arguments: {}, expected up to 2 arguments: word_count [input] [output]",
                args.len() - 1
            );
        }
    };

    let count_start_time_mono = Instant::now();
    let count_start_time_wall = SystemTime::now();

    let words = match input_file {
        None => {
            let input = io::stdin().lock();
            let reader = BufReader::new(input);
            word_count::count_words(reader.bytes().map(|b| b.unwrap()))
        }
        Some(filename) => {
            let input = File::open(filename)?;
            let reader = BufReader::new(input);
            word_count::count_words(reader.bytes().map(|b| b.unwrap()))
        }
    };

    let counted_time_mono = count_start_time_mono.elapsed();
    let counted_time_wall = count_start_time_wall.elapsed()?;

    let start_time_mono = Instant::now();
    let start_time_wall = SystemTime::now();

    match output_file {
        None => {
            let stdout = io::stdout().lock();
            let mut writer = BufWriter::new(stdout);
            for w in words {
                write!(writer, "{}\t", w.count)?;
                writer.write_all(&w.word)?;
                writer.write_all(&[b'\n'])?;
            }

            writer.flush()?;
        }
        Some(filename) => {
            let output = File::open(filename)?;
            let mut writer = BufWriter::new(output);
            for w in words {
                write!(writer, "{}\t", w.count)?;
                writer.write_all(&w.word)?;
                writer.write_all(&[b'\n'])?;
            }

            writer.flush()?;
        }
    };

    let printed_time_mono = start_time_mono.elapsed();
    let printed_time_wall = start_time_wall.elapsed()?;

    eprintln!("count time (mono): {}ms", counted_time_mono.as_millis());
    eprintln!("count time (wall): {}ms", counted_time_wall.as_millis());

    eprintln!("print time (mono): {}ms", printed_time_mono.as_millis());
    eprintln!("print time (wall): {}ms", printed_time_wall.as_millis());

    eprintln!("total time (mono): {}ms", count_start_time_mono.elapsed().as_millis());
    eprintln!("total time (wall): {}ms", count_start_time_wall.elapsed()?.as_millis());

    Ok(())
}
