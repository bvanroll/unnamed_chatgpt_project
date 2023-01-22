use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize};

#[derive(Serialize)]
struct MiniHuman {
    name: String,
    gender: String,
}


fn main() {
    let mut miniHumans:Vec<MiniHuman> = Vec::new();
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let file = File::open(res.as_path()).unwrap();
    let reader = BufReader::new(file);
    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            //# = comment
            // let mut chars = line.chars();
            // let mut gender:String = String::from(chars.next().unwrap());
            // if gender != "?" && gender != "F" && gender != "M"  { continue; }
            // if gender == "?" {
            //     let preference = chars.next().unwrap();
            //     gender = gender + &*(preference).to_string();
            // }
            let mut parts = line.split(" ");
            let gender = parts.next().unwrap().to_string();
            if gender.chars().next().unwrap() == '#' { continue; }
            parts.next();
            let mut name = parts.next().unwrap().to_string();
            name = name.replace("+", " ");
            miniHumans.push(MiniHuman{name: name, gender: gender})
        }

    }
    let serialized: String = serde_json::to_string(&miniHumans).unwrap();
    let save_res = rfd::FileDialog::new().set_directory(&current_path).save_file().unwrap();
    let mut file = File::create(save_res.as_path()).unwrap();
    file.write_all(serialized.as_bytes()).expect("oopsie");
}
