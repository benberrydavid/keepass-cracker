pub fn read_file(file_path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).expect("Unable to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read file");
    data
}

pub fn write_output(entries: &[super::keepass::KeePassEntry>) {
    for entry in entries {
        println!("Title: {}, Username: {}, Password: {}", entry.title, entry.username, entry.password);
    }
}