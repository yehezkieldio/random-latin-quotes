use std::sync::{Arc, RwLock, RwLockReadGuard};

use actix_web::{get, web, App, HttpServer, Responder};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    text: String,
    translation: Option<String>,
    author: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quotes {
    quotes: Vec<Quote>,
}

struct AppState {
    quotes: Arc<RwLock<Vec<Quote>>>,
}

#[get("/")]
async fn random_quote(data: web::Data<AppState>) -> impl Responder {
    let quotes: RwLockReadGuard<Vec<Quote>> = data.quotes.read().unwrap();
    let quote: Option<&Quote> = quotes.choose(&mut rand::thread_rng());

    match quote {
        Some(quote) => web::Json(Quote {
            text: quote.text.clone(),
            author: quote.author.clone(),
            translation: quote.translation.clone(),
        }),
        None => web::Json(Quote {
            text: "No quotes available".to_string(),
            author: "System".to_string(),
            translation: None,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let quotes_file: std::fs::File =
        std::fs::File::open("data/quotes.json").expect("file not found");
    let quotes: Quotes = serde_json::from_reader(quotes_file).expect("error while reading file");

    let amount_of_quotes: usize = quotes.quotes.len();
    println!("Amount of quotes: {}", amount_of_quotes);
    println!("Serving quotes at http://127.0.0.1:8080");

    let app_state: web::Data<AppState> = web::Data::new(AppState {
        quotes: Arc::new(RwLock::new(quotes.quotes)),
    });

    HttpServer::new(move || App::new().app_data(app_state.clone()).service(random_quote))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
