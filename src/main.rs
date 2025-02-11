#[macro_use] extern crate rocket;

use std::fs;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};

#[get("/")]
fn index() -> Template {
    let file_content = fs::read_to_string("static/2025.json").expect("Failed to read JSON file");
    let balls: Vec<Ball> = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    let mayballs: Vec<&Ball> = balls.iter().filter(|ball| ball.season == Season::MAY).collect();
    let springballs: Vec<&Ball> = balls.iter().filter(|ball| ball.season == Season::SPRING).collect();
    let winterballs: Vec<&Ball> = balls.iter().filter(|ball| ball.season == Season::WINTER).collect();
    Template::render("index", context! { springballs: springballs, winterballs: winterballs, mayballs: mayballs })
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
enum Season {
    WINTER,
    SPRING,
    MAY,
}

#[derive(Deserialize, Serialize)]
struct Link {
    text: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
struct Ball {
    name: String,
    date: String,
    theme: String,
    price: String,
    season: Season,
    links: Vec<Link>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}
