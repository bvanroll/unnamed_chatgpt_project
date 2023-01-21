use std::fs::File;
use std::io::{BufReader, Read, Write};
use chatgpt::client::ChatGPT;
use serde::{Serialize, Deserialize};
use rand::Rng;

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
    job: String,
    bio: String,
}
fn main() {
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let mut file = File::open(res.as_path()).unwrap();
    let mut json_string:String = String::new();
    file.read_to_string(&mut json_string).unwrap();
    let mut MiniHumans: Vec<MiniHuman> = serde_json::from_str(&json_string).unwrap();
    let mut Humans: Vec<Human> = Vec::new();
    while MiniHumans.len() > 1 {
        let (mut firstName, mut firstGender) = getRngName(&mut MiniHumans);
        let (mut lastName, mut lastGender) = getRngName(&mut MiniHumans);
        if firstName == "" || lastName == "" || (firstGender == "" && lastGender == "") { continue }
        if firstGender == "" {firstGender = lastGender.clone()}
        if lastGender == "" {lastGender = firstGender.clone()}
        let gender: String = decideGender(firstGender, lastGender);
        Humans.push(getHuman(firstName, lastName, gender))
    }
    let serialized: String = serde_json::to_string(&Humans).unwrap();
    let save_res = rfd::FileDialog::new().set_directory(&current_path).save_file().unwrap();
    let mut file = File::create(save_res.as_path()).unwrap();
    file.write_all(serialized.as_bytes()).expect("oopsie");
}


fn getRngName(miniHumans: &mut Vec<MiniHuman>) -> (String, String) {
    let index = rand::thread_rng().gen_range(0..miniHumans.len());
    let temp = miniHumans.remove(index);
    return (temp.name, temp.gender)
}

fn decideGender(first: String, second: String) -> String {
    if rand::thread_rng().gen_bool(1.0/5.0) { return second; } //1 in 5 chance to use gender of last name... maybe interesting idea
    return first;
}

async fn getHuman(firstName: String, lastName: String, gender: String) -> Human {
    let mut token: String = std::env::var("SESSION_TOKEN").unwrap(); // obtain the session token. More on session tokens later.
    let mut client = ChatGPT::new(token).unwrap();
    client.refresh_token().await.unwrap(); // it is recommended to refresh token after creating a client
    let mut conversation = client.new_conversation();
    let mut finalGender = writeGender(gender);
    let age: String = conversation.send_message(&client, "Can you give me the age of a hypothetical {} named {} {}").await.unwrap();
    let country: String = conversation.send_message(&client, "What country would this person be from?").await.unwrap();
    let job: String = conversation.send_message(&client, "What would this persons job title be?").await.unwrap();
    let bio: String = conversation.send_message(&client, "Could you write a short bio this person could use on their online profiles?").await.unwrap();
    return Human{
        firstName: firstName,
        lastName: lastName,
        gender: finalGender,
        age: age,
        country: country,
        job: job,
        bio: bio,
    };
}

fn writeGender(gender: String) -> String {
    if gender == "F" {
        return "female".to_string()
    } else if gender == "M" {
        return "male".to_string()
    } else if gender == "1M" {
        if rand::thread_rng().gen_bool(1.0/9.0) { return "male".to_string() }
        return "female".to_string()
    } else if gender == "1F" {
        if rand::thread_rng().gen_bool(1.0/9.0) { return "female".to_string() }
        return "male".to_string()
    } else if gender == "?" {
        if rand::thread_rng().gen_bool(1.0/2.0) { return "male".to_string() }
        return "female".to_string()
    } else if gender == "?F" {
        if rand::thread_rng().gen_bool(1.0/5.0) { return "male".to_string() }
        return "female".to_string()
    } else if gender == "?M" {
       if rand::thread_rng().gen_bool(1.0/5.0) { return "female".to_string() }
        return "male".to_string()
    }

    return "male".to_string()
}