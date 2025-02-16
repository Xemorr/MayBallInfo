mod types;

#[macro_use] extern crate rocket;

use std::env::temp_dir;
use std::fs;
use std::io::Write;
use chrono::NaiveDate;
use icalendar::{Calendar, Component};
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::State;
use rocket_dyn_templates::{Template, context};
use tempfile::NamedTempFile;
use crate::types::{Ball, MayballInfo};

#[get("/")]
fn index(state: &State<MayballInfo>) -> Template {
    Template::render("index", context! { springballs: &*state.springballs, winterballs: &*state.winterballs, mayballs: &*state.mayballs })
}

async fn generate_ics(ball: &Ball) -> Option<NamedFile> {
    let mut calendar: Calendar = Calendar::new();
    let date: NaiveDate = NaiveDate::parse_from_str(&ball.date, "%Y/%m/%d").unwrap();

    let mut ical_event = icalendar::Event::new();
    ical_event.add_property("SUMMARY", ball.name.clone());
    ical_event.add_property("DTSTART;VALUE=DATE", date.format("%Y%m%d").to_string());
    ical_event.add_property("DTEND;VALUE=DATE", date.format("%Y%m%d").to_string());
    calendar.push(ical_event);

    let mut temp_file = NamedTempFile::new().expect("Could not create temp file");
    let temp_path = temp_dir().join(format!("{}.ics", ball.name));
    temp_file.write_all(calendar.to_string().as_bytes()).unwrap();
    fs::rename(temp_file.path(), &temp_path).expect("Failed to rename temp file");
    NamedFile::open(temp_file).await.ok()
}

#[get("/calendar", data = "<ball_name>")]
async fn calendar(state: &State<MayballInfo>, ball_name: String) -> Option<NamedFile> {
    let MayballInfo { mayballs, springballs, winterballs } = state.inner();
    let ball_of_interest = mayballs
        .iter()
        .chain(springballs.iter())
        .chain(winterballs.iter())
        .find(|ball| ball.name == ball_name)
        .expect("The ball name should exist!");

    generate_ics(ball_of_interest).await
}

#[launch]
fn rocket() -> _ {
    let file_content = fs::read_to_string("static/2025.json").expect("Failed to read JSON file");
    let balls: Vec<Ball> = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    let app_state = MayballInfo::new(balls);
    rocket::build()
        .manage(app_state)
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}
