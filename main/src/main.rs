use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct MiniHuman {
    name: String,
    gender: String,
}
#[derive(Serialize)]
struct Human {
    firstName: String,
    lastName: String,
    gender: String,
    age: String,
    country: String,
    bio: String,
    job: String,
}
fn main() {
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let mut file = File::open(res.as_path()).unwrap();
    let mut json_string:String = String::new();
    file.read_to_string(&mut json_string).unwrap();
    let MiniHumans: Vec<MiniHuman> = serde_json::from_str(&json_string).unwrap();
    while MiniHumans.len() > 1 {
        let firstName: String = getRngName(&MiniHumans);
        let lastName: String = getRngName(&MiniHumans);

    }
}


fn getRngName(miniHumans: &Vec<MiniHuman>) -> String {
   return "".to_string()
}