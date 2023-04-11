use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::io::Reader;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, ParallelBridge};

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    arguments.par_iter().for_each(|argument| {
        let source_file = Path::new(&argument);
        let mut dest_file = source_file.to_path_buf();
        dest_file.set_extension("csv");
        let dest_file = dest_file.as_path();
        let Ok(source) = Reader::open(source_file) else {
            println!("Unable to open file! {}", argument);
            return;
        };
        let Ok(source) = source.decode() else {
            println!("Unable to decode file as image! {}", argument);
            return;
        };
        let source = source.into_luma8();
        let output: String = source.rows().par_bridge().map(
                    |row| {
                        row.par_bridge().map(
                            |pixel| {
                                pixel.0[0].to_string() // Looks a little funky, it's just a little arcane to get the raw integer out of the Luma8.
                            }).reduce(String::new, |acc, next| acc + ", "  + &next)
                        }).reduce(String::new, |acc, next| acc + "\n" + &next);
        let Ok(mut file) = File::create(dest_file) else {
            println!("Unable to create file! {}", dest_file.display());
            return;
        };
        if file.write_all(output.as_bytes()).is_err() {
            println!("Failed to write to file! {}", dest_file.display());
        };
    });
}
