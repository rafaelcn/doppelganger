use std::collections::HashMap;
use std::{env, fs};

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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
            let doppelganger: HashMap<u64, File> = HashMap::new();

            let mut handles = vec![];
            let mutex = Arc::new(Mutex::new(doppelganger));

            let files = fs::read_dir(directory).expect("failed to read directory");

            for entry in files {
                let resource = mutex.clone();

                let handle = thread::spawn(move || {
                    let dir_entry = entry.expect("failed to capture a directory entry");

                    let file_name = dir_entry.file_name().to_str().unwrap().to_string();
                    let file_path = dir_entry.path();
                    let file_type = dir_entry.file_type().expect("failed to get the file type");

                    if let Some(v) = file_path.to_str() {
                        if file_type.is_file() {
                            let mut file = File::new(v.to_string());

                            file.hash().expect("failed to calculate file hash");

                            let mut map = resource
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

                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            let map = mutex.lock().unwrap();

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
        None => eprintln!("not a valid directory (?)"),
    }

    Ok(())
}
