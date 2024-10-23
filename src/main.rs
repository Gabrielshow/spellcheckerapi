#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use hunspell::Hunspell;

#[derive(Deserialize)]
struct SpellCheckRequest {
    input: String,
}

#[derive(Serialize)]
struct SpellCheckResponse {
    suggestions: Vec<String>,
}

#[post("/api/spellcheck", format = "json", data = "<request>")]
fn spellcheck(request: Json<SpellCheckRequest>) -> Json<SpellCheckResponse> {
    // Create a Hunspell instance and handle the Result
    let hunspell_result: Result<Hunspell, _> = Hunspell::new("../lib/Us.dic", "../lib/Us.aff");
    
    match hunspell_result {
        Ok(mut hunspell) => {
            // If successful, get suggestions
            let suggestions = hunspell.suggest(&request.input);
            Json(SpellCheckResponse { suggestions })
        }
        Err(e) => {
            // Handle the error case
            eprintln!("Error creating Hunspell instance: {}", e);
            Json(SpellCheckResponse { suggestions: vec![] }) // Return empty suggestions on error
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![spellcheck])
}
