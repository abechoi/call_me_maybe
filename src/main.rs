use dotenv::dotenv;
use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::env;

#[allow(unused)]
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Call { from: String, to: String },
    Text { from: String, to: String },
}

#[tokio::main]
async fn main() {

    dotenv().expect("Error reading .env file");
    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Error reading Twilio Account SID");
    let api_key_sid = env::var("TWILIO_API_KEY_SID").expect("Error reading Twilio API key");
    let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Error reading Twilio API SID");

    let args = Cli::parse(); 

    let mut twilio_config = Configuration::default();
    twilio_config.basic_auth = Some((api_key_sid, Some(api_key_secret)));

    match &args.command {
        Commands::Call { from, to } => {
            println!("Twilio call test: {} -> {}", &from, &to);
            test_call(&twilio_config, &account_sid, &from, &to).await;
        },
        Commands::Text { from, to } => {
            println!("Twilio SMS test: {} -> {}", &from, &to);
            test_sms(&twilio_config, &account_sid, &from, &to).await;
        },
    }
}

// cmm text <from> <to>
async fn test_sms(twilio_config: &Configuration, account_sid: &str, from: &str, to: &str) {

    let body = "Twilio SMS test: SUCCESS".to_string();

    let message = twilio_api::create_message(
        &twilio_config,
        &account_sid,
        &to,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&from),
        None,
        Some(&body),
        None,
    )
    .await;

    //let result = message.expect("Twilio SMS test: FAIL");

    match message {
        Ok(_) => println!("Twilio SMS test: SUCCESS"),
        Err(_) => println!("Twilio SMS test: FAIL"),
    };
}

// cmm call <from> <to>
async fn test_call(twilio_config: &Configuration, account_sid: &str, from: &str, to: &str) {

    let url = "http://demo.twilio.com/docs/voice.xml".to_string();

    let call = twilio_api::create_call(
        &twilio_config,
        &account_sid,
        &to,
        &from,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&url), // url required
        None,
        None,
    ).await;

    //let result = call.expect("Twilio call test failed...");

    match call {
        Ok(_) => println!("Twilio call test: SUCCESS"),
        Err(_) => println!("Twilio SMcallS test: FAIL"),
    };
}