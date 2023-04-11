use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::io::Reader;

fn main() {
    for argument in env::args().skip(1) {
        let source_file = Path::new(&argument);
        let mut dest_file = source_file.to_path_buf();
        dest_file.set_extension("csv");
        let dest_file = dest_file.as_path();
        let Ok(source) = Reader::open(source_file) else {
            println!("Unable to open file! {}", argument);
            continue;
        };
        let Ok(source) = source.decode() else {
            println!("Unable to decode file as image! {}", argument);
            continue;
        };
        let source = source.into_luma8();
        let output: String = source.rows().map(
                    |row| {
                        row.map(
                            |pixel| {
                                pixel.0[0].to_string() // Looks a little funky, it's just a little arcane to get the raw integer out of the Luma8.
                            }).reduce(|acc, next| acc + ", "  + &next).unwrap()
                        }).reduce(|acc, next| acc + "\n" + &next).unwrap();
        let Ok(mut file) = File::create(dest_file) else {
            println!("Unable to create file! {}", dest_file.display());
            continue;
        };
        if file.write_all(output.as_bytes()).is_err() {
            println!("Failed to write to file! {}", dest_file.display());
        };
    };
}
