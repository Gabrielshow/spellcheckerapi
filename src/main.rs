#[macro_use]
extern crate rocket;

use std::env;
// use dotenv::dotenv;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs;
// use zspell::Dictionary;

#[derive(Deserialize)]
struct SpellCheckRequest {
    input: String,
}

#[derive(Serialize)]
struct SpellCheckResponse {
    valid: bool,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Spell Check API!"
}

#[post("/api/spellcheck", format = "json", data = "<request>")]
fn spellcheck(request: Json<SpellCheckRequest>) -> Json<SpellCheckResponse> {
    // Load the affix file and dictionary file content
    let aff_content =
        fs::read_to_string("../lib/Us.aff").expect("Failed to load config file");
    let dic_content =
        fs::read_to_string("../lib/Us.dic").expect("Failed to load wordlist file");

    // Create a zspell Dictionary instance and handle errors
    let dictionary = zspell::builder()
        .config_str(&aff_content)
        .dict_str(&dic_content)
        .build()
        .expect("Failed to build dictionary!");

    // Check if the input word is valid
    let is_valid = dictionary.check_word(&request.input);

    // Return the validity in the response
    Json(SpellCheckResponse { valid: is_valid })
}

#[launch]
fn rocket() -> _ {
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8000".to_string()).parse().unwrap();
    
    // Launch the Rocket application on the specified address and port
    rocket::build().mount("/", routes![index, spellcheck]).configure(rocket::config::Config {
        port,
        address: "0.0.0.0".parse().unwrap(),
        ..Default::default()
    })
}