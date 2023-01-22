use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
//use chatgpt::client::ChatGPT;
use async_openai::{Client, types::{CreateCompletionRequestArgs}};
use serde::{Serialize, Deserialize};
use rand::Rng;
use rust_bert::roberta::RobertaForQuestionAnswering;

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

#[tokio::main]
async fn main() {
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let mut file = File::open(res.as_path()).unwrap();
    let mut json_string:String = String::new();
    file.read_to_string(&mut json_string).unwrap();
    let mut MiniHumans: Vec<MiniHuman> = serde_json::from_str(&json_string).unwrap();
    let mut Humans: Vec<Human> = Vec::new();
    let save_res = rfd::FileDialog::new().set_directory(&current_path).save_file().unwrap();
    let mut client = Client::new();
    while MiniHumans.len() > 1 {
        let (mut firstName, mut firstGender) = getRngName(&mut MiniHumans);
        let (mut lastName, mut lastGender) = getRngName(&mut MiniHumans);
        if firstName == "" || lastName == "" || (firstGender == "" && lastGender == "") { continue }
        if firstGender == "" {firstGender = lastGender.clone()}
        if lastGender == "" {lastGender = firstGender.clone()}
        let gender: String = decideGender(firstGender, lastGender);
        let h = match getHuman(&mut client, firstName, lastName, gender).await {
            Ok(h) => Humans.push(h),
            Err(e) => println!("some err occured: {:?}", e.to_string()),
        };
        break;
    }
    let serialized: String = serde_json::to_string(&Humans).unwrap();
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

async fn getHuman(client: &mut Client, firstName: String, lastName: String, gender: String) -> Result<Human, Box<dyn std::error::Error>> {
    let request = CreateCompletionRequestArgs::default()
        .model("text-ada-001")
        .prompt(format!("Write a short bio for a character called {} {} making sure to mention their age, gender, current country of residence and current jobtitle", firstName, lastName))
        .max_tokens(200_u16)
        .build()?;
    let res = client.completions().create(request).await;
    let response = String::from(format!("{}", res?.choices.first().unwrap().text));

    let (finalGender, age, country, job) = getHumanFromContext(response.clone());

    return Ok(Human{
        firstName: firstName,
        lastName: lastName,
        gender: finalGender,
        age: age,
        country: country,
        job: job,
        bio: response,
    });
}
//returns in order: gender, age, country, job
fn getHumanFromContext(context: String, firstName: String) -> (String, String, String, String) {
    //TODO use the other ai to get answers from a given context
    let qa_model = QuestionAnsweringModel::new(Default::default())?;
    let gender = String::from(format!("What is {}'s gender?", firstname));
    let age = String::from(format!("What is {}'s age?", firstName));
    let country= String::from(format!("Where does {} live?", firstName));
    let job = String::from(format!("What is {}'s job?", firstName));
    let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);
    return ("".to_string(), "".to_string(), "".to_string(), "".to_string())
}