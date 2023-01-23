use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

use serde::{Serialize, Deserialize};

use rust_bert::bert::{BertConfigResources, BertModelResources, BertVocabResources};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::question_answering::Answer;
use rust_bert::pipelines::question_answering::{
    QaInput, QuestionAnsweringConfig, QuestionAnsweringModel,
};
use rust_bert::resources::RemoteResource;



#[derive(Deserialize, Serialize)]
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
    //load in the file with bio's and names
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let mut file = File::open(res.as_path()).unwrap();
    let mut json_string:String = String::new();
    file.read_to_string(&mut json_string).unwrap();
    let mut Humans: Vec<Human> = serde_json::from_str(&json_string).unwrap();
    //prep final file
    let save_res = rfd::FileDialog::new().set_directory(&current_path).save_file().unwrap();
    let mut i = 0;
    let mut l = &Humans.len().clone();
    println!("there are {} humans to process", l - i);

    for mut human in &mut Humans {
        //let (gender, age, country, job) = getHumanFromContext(
        match getHumanFromContext(human.bio.clone(), human.firstName.clone()) {
            Ok((gender, age, country, job)) => {
                human.gender = gender;
                human.age = age;
                human.country = country;
                human.job = job;
                println!("just did {} at index {}", human.firstName.clone(), i);
            },
            Err(e) => {
                println!("skipping {} because of {}", human.firstName.clone(), e.to_string())
            }
        }
        println!("There are {} humans left to process", l - (i+1));
        i = i+1;
        if (i > 100) { break; }
    }

    let serialized: String = serde_json::to_string(&Humans).unwrap();
    let mut file = File::create(save_res.as_path()).unwrap();
    file.write_all(serialized.as_bytes()).expect("oopsie");
}


fn getHumanFromContext(context: String, firstName: String) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
    //TODO use the other ai to get answers from a given context
    let bertconfig = QuestionAnsweringConfig::new(
        ModelType::Bert,
        RemoteResource::from_pretrained(BertModelResources::BERT_QA),
        RemoteResource::from_pretrained(BertConfigResources::BERT_QA),
        RemoteResource::from_pretrained(BertVocabResources::BERT_QA),
        None, //merges resource only relevant with ModelType::Roberta
        false,
        false,
        None,
    );
    let mut model = QuestionAnsweringModel::new(bertconfig)?;
    let mut genderQuestion = QaInput {
        question: format!("What is {}'s gender?", firstName),
        context: context.clone()
    };
    let mut ageQuestion = QaInput {
        question: format!("What is {}'s age?", firstName),
        context: context.clone()
    };
    let mut countryQuestion = QaInput {
        question: format!("Where does {} live?", firstName),
        context: context.clone()
    };
    let mut jobQuestion = QaInput {
        question: format!("What is {}'s job?", firstName),
        context: context.clone()
    };

    let mut answers = model.predict(&[genderQuestion, ageQuestion, countryQuestion, jobQuestion], 1, 32);
    let mut looper = answers.iter();
    let mut gender = looper.next().expect("euh").first();
    let mut age = looper.next().expect("euh").first();
    let mut country= looper.next().expect("euh").first();
    let mut job = looper.next().expect("euh").first();


    return Ok((gender.unwrap().answer.clone(), age.unwrap().answer.clone(), country.unwrap().answer.clone(), job.unwrap().answer.clone()))
}
