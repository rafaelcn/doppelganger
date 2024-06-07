use std::collections::HashMap;
use std::{env, fs};

use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

mod doppelganger;

use crate::doppelganger::file::File;

#[allow(unused)]
fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        return Err(String::from("gimme a directory"));
    }

    let arg = args.get(1);

    match arg {
        Some(directory) => {
            let files = fs::read_dir(directory).expect("failed to read directory");

            let doppelganger: HashMap<u64, File> = HashMap::new();
            let mutex = Arc::new(Mutex::new(doppelganger));

            let files_list: Vec<_> = files.into_iter().collect();

            files_list.into_par_iter().for_each(|file_entry| {
                let mutex = mutex.clone();

                let dir_entry = file_entry.expect("failed to capture a directory entry");

                let file_name = dir_entry.file_name().to_str().unwrap().to_string();
                let file_path = dir_entry.path();
                let file_type = dir_entry.file_type().expect("failed to get the file type");

                if let Some(v) = file_path.to_str() {
                    if file_type.is_file() {
                        let mut file = File::new(v.to_string());

                        file.hash().expect("failed to calculate file hash");

                        let mut map = mutex
                            .lock()
                            .expect("couldn't get a hold in the doppelganger map");

                        if map.contains_key(&file.hash) {
                            let duplicated = map.get(&file.hash).unwrap();

                            file.duplicates.push(file_name);
                            file.duplicates.append(&mut duplicated.duplicates.clone());

                            file.duplicates_number += duplicated.duplicates_number + 1;
                        } else {
                            file.duplicates.push(file_name);
                        }

                        map.insert(file.hash, file);
                    }
                }
            });

            let map = mutex.lock().unwrap();

            let duplicates = map
                .iter()
                .fold(0u64, |acc, file| acc + file.1.duplicates_number);

            if duplicates == 0 {
                println!("no duplicates found")
            } else {
                map.iter()
                    .filter(|(_k, v)| v.duplicates_number >= 1)
                    .for_each(|(_k, v)| {
                        println!(
                            "[{}] file is duplicated {} time(s)",
                            v.hash, v.duplicates_number
                        );
                        v.duplicates.iter().for_each(|name| println!("\t{}", name));
                    });
            }
        }
        None => eprintln!("not a valid directory (?)"),
    }

    Ok(())
}
