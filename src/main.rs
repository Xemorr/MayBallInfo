mod types;

#[macro_use] extern crate rocket;

use std::env::temp_dir;
use std::fs;
use std::ops::Add;
use chrono_tz::Europe::London;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use icalendar::{Calendar, Component};
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::State;
use rocket_dyn_templates::{Template, context};
use rand::seq::SliceRandom;
use rand::thread_rng;
use uuid::Uuid;
use anyhow::Result;
use crate::types::{Ball, MayballInfo};

#[get("/")]
fn index(state: &State<MayballInfo>) -> Template {
    Template::render("index", context! { springballs: &*state.springballs, winterballs: &*state.winterballs, mayballs: &*state.mayballs })
}

async fn generate_ics(ball: &Ball) -> Result<NamedFile> {
    let mut calendar = Calendar::new();
    let date = NaiveDate::parse_from_str(&ball.date, "%Y/%m/%d")?;

    let local_start = London
        .from_local_datetime(&NaiveDateTime::new(date, NaiveTime::from_hms_opt(19, 0, 0).unwrap()))
        .single()
        .unwrap();

    let local_end = local_start.add(Duration::hours(11));

    let mut ical_event = icalendar::Event::new();
    ical_event.add_property("SUMMARY", ball.name.clone());
    ical_event.add_property("DTSTART", local_start.format("%Y%m%dT%H%M%SZ").to_string());
    ical_event.add_property("DTEND", local_end.format("%Y%m%dT%H%M%SZ").to_string());
    ical_event.add_property("DTSTAMP", Utc::now().format("%Y%m%dT%H%M%SZ").to_string());
    ical_event.add_property("UID", format!("{}@mayball.com", Uuid::new_v4()));
    calendar.push(ical_event);

    let file_path = temp_dir().join(format!("{}.ics", ball.name));
    fs::write(&file_path, calendar.to_string())?;

    Ok(NamedFile::open(file_path).await?)
}

#[get("/calendar?<ball_name>")]
async fn calendar(state: &State<MayballInfo>, ball_name: String) -> Option<NamedFile> {
    println!("{}", ball_name);
    let MayballInfo { mayballs, springballs, winterballs } = state.inner();
    let result = mayballs
        .iter()
        .chain(springballs.iter())
        .chain(winterballs.iter())
        .find(|ball| ball.name == ball_name)
        .map(generate_ics)
        .unwrap()
        .await;
    match result {
        Ok(value) => Option::from(value),
        Err(error) => {
            None
        }
    }
}

#[launch]
fn rocket() -> _ {
    let file_content = fs::read_to_string("static/2026.json").expect("Failed to read JSON file");
    let mut balls: Vec<Ball> = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    balls.shuffle(&mut thread_rng());
    balls.sort_by(|ball1, ball2| {
        ball1.score().cmp(&ball2.score())
    });
    balls.reverse();
    let app_state = MayballInfo::new(balls);
    rocket::build()
        .manage(app_state)
        .attach(Template::fairing())
        .mount("/", routes![index, calendar])
        .mount("/", FileServer::from(relative!("static")))
}
