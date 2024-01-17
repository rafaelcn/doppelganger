use std::collections::HashMap;
use std::{env, fs};

mod doppelganger;

use crate::doppelganger::file::File;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        return Err(String::from("gimme a directory"));
    }

    let arg = args.get(1);

    match arg {
        Some(directory) => {
            let mut doppelganger: HashMap<u64, File> = HashMap::new();

            let files = fs::read_dir(directory).expect("failed to read directory");

            for entry in files {
                let dir_entry = entry.expect("failed to capture a directory entry");

                let file_name = dir_entry.file_name().to_str().unwrap().to_string();
                let file_path = dir_entry.path();
                let file_type = dir_entry.file_type().expect("failed to get the file type");

                if let Some(v) = file_path.to_str() {
                    if file_type.is_file() {
                        let mut file = File::new(v.to_string());

                        file.hash().expect("failed to calculate file hash");

                        if let Some(duplicated) = doppelganger.get(&file.hash) {
                            file.duplicates.push(file_name);
                            file.duplicates.append(&mut duplicated.duplicates.clone());

                            file.duplicates_number += duplicated.duplicates_number + 1;

                            doppelganger.insert(file.hash, file);
                        } else {
                            file.duplicates.push(file_name);

                            doppelganger.insert(file.hash, file);
                        }
                    }
                }
            }

            doppelganger
                .iter()
                .filter(|(k, v)| v.duplicates_number >= 1)
                .for_each(|(k, v)| {
                    println!(
                        "[{}] file is duplicated {} time(s)",
                        v.hash, v.duplicates_number
                    );
                    v.duplicates.iter().for_each(|name| println!("\t{}", name));
                });
        }
        None => eprintln!("not a valid directory (?)"),
    }

    Ok(())
}
