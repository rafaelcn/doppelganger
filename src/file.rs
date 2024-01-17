struct DoeppelgangerFile {
    hash: u64,
    path: String,
}

impl DoppelgangerFile {
    fn new(path: String) -> File {
        File { hash: 0, path }
    }

    fn hash(&self) -> Result<u64, String> {
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
            Ok(_n) => buffer.iter().for_each(|b| hasher.write_u8(*b)),
            Err(err) => eprintln!("failed to read file, reason {}", err),
        }

        Ok(hasher.finish())
    }
}
