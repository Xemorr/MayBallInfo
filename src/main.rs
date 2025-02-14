mod types;

#[macro_use] extern crate rocket;

use std::fs;
use icalendar::{Calendar, Property};
use rocket::fs::{relative, FileServer};
use rocket::response::content;
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};
use crate::types::{Ball, Season};

#[get("/")]
fn index() -> Template {
    let file_content = fs::read_to_string("static/2025.json").expect("Failed to read JSON file");
    let balls: Vec<Ball> = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    let mayballs: Vec<&Ball> = balls.iter().filter(|ball| ball.season == Season::MAY).collect();
    let springballs: Vec<&Ball> = balls.iter().filter(|ball| ball.season == Season::SPRING).collect();
    let winterballs: Vec<&Ball> = balls.iter().filter(|ball| ball.season == Season::WINTER).collect();
    Template::render("index", context! { springballs: springballs, winterballs: winterballs, mayballs: mayballs })
}

fn transform_date(input: &str) -> Result<String, chrono::ParseError> {
    // Parse the input date string into a NaiveDate
    let naive_date = NaiveDate::parse_from_str(input, "%Y/%m/%d")?;

    // Create a NaiveDateTime from the NaiveDate
    let naive_datetime = NaiveDateTime::new(
        naive_date,
        chrono::NaiveTime::from_hms(11, 0, 0), // Set the time to 11:00:00
    );

    // Format the NaiveDateTime into the desired output format
    let formatted_date = naive_datetime.format("%Y%m%dT%H%M%SZ").to_string();

    Ok(formatted_date)
}

fn generate_ics(ball: &Ball) -> String {
    let mut calendar = Calendar::new();

    let mut ical_event = icalendar::Event::new();
    ical_event.push(Property::new("SUMMARY", ball.name.clone()));
    ical_event.push(Property::new("DTSTART", ball.date));
    ical_event.push(Property::new("DTEND", "20231001T110000Z"));
    calendar.push(ical_event);

    calendar.to_string()
}

#[get("/calendar", data = "<ball>")]
fn calendar(ball: String) -> content::RawHtml<String> {

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}
