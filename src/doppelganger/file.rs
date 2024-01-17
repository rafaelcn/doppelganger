use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::{fs, io::Read};

pub struct File {
    pub hash: u64,
    path: String,
    pub duplicates: Vec<String>,
    pub duplicates_number: u64,
}

impl File {
    pub fn new(path: String) -> File {
        File {
            hash: 0,
            path,
            duplicates: Vec::new(),
            duplicates_number: 0,
        }
    }

    pub fn hash(&mut self) -> Result<u64, String> {
        if self.path.is_empty() {
            return Err(String::from("file path is empty"));
        }
        if self.hash > 0 {
            return Ok(self.hash);
        }

        let mut file = fs::File::open(self.path.clone()).expect("failed to open file");
        let mut buffer: Vec<u8> = Vec::new();

        let mut hasher = DefaultHasher::new();

        println!("calculating hash for file {}", self.path);

        match file.read_to_end(&mut buffer) {
            Ok(size) => buffer.iter().for_each(|b| hasher.write_u8(*b)),
            Err(err) => eprintln!("failed to read file, reason {}", err),
        }

        self.hash = hasher.finish();

        Ok(self.hash)
    }
}
