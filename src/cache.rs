use std::borrow::Cow;
use std::fs::{create_dir_all, File};
use std::path::{PathBuf, Path};
use std::error::Error;
use std::io::{Write, Read};

pub struct ApnFile {
    path: String,
    url: String
}

impl ApnFile {
    pub fn new(path: &str, url: &str) -> ApnFile {
        let expanded_path: Cow<str> = shellexpand::full(path).unwrap();

        return ApnFile {
            path: String::from(expanded_path),
            url: String::from(url)
        };
    }

    pub fn read(&self, force_update: bool) -> String {
        if (force_update || !Path::new(&self.path).exists()) {
            self.update().unwrap();
        }

        let mut contents: String = String::new();

        match File::open(&self.path) {
            Ok(mut file) => file.read_to_string(&mut contents).expect(&format!("unable to read apn cache file {}", &self.path)[..]),
            Err(_) => panic!("unable to open apn cache file '{}'", &self.path)
        };

        return contents;
    }

    pub fn update(&self) -> Result<(), String> {
        self.create_path().unwrap();

        return match self.fetch() {
            Ok(xml) => Ok(self.write_file(&xml).unwrap()),
            Err(s) => Err(String::from(s))
        };
    }

    fn create_path(&self) -> Result<(), String> {
        let file_path = PathBuf::from(&self.path);
        let dir_path = file_path.parent().unwrap();

        return match create_dir_all(dir_path) {
            Ok(_) => Ok(()),
            Err(e) => return Err(format!("unable to create directory {}: {}", dir_path.display(), e.description()))
        };
    }

    fn write_file(&self, content: &String) -> Result<(), String> {
        let mut apn_file: File = match File::create(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(format!("unable to write to db cache file {}: {}", &self.path, e.description()))
        };

        return match apn_file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("unable to write to db cache file {}: {}", &self.path, e.description()))
        };
    }

    fn fetch(&self) -> Result<String, &str> {
        let mut body: String = match reqwest::get(&self.url) {
            Ok(mut req) => match req.text() {
                Ok(text) => text,
                _ => return Err("unable to fetch response")
            },
            _ => return Err("unable to place request")
        };

        let decoded_body: Vec<u8> = match base64::decode(&body) {
            Ok(decoded) => decoded,
            _ => return Err("unable to decode base64 data")
        };

        body = String::from(match std::str::from_utf8(&decoded_body) {
            Ok(utf8) => utf8,
            _ => return Err("unable to decode utf8 string")
        });

        return Ok(body);
    }
}
