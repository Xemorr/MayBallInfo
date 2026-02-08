use std::sync::Arc;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum Season {
    WINTER,
    SPRING,
    MAY,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Link {
    text: String,
    url: String,
}

#[derive(Deserialize, Clone)]
pub struct Ball {
    pub name: String,
    pub date: String,
    pub theme: String,
    pub price: String,
    pub season: Season,
    pub links: Vec<Link>,
}

impl Ball {
    pub fn score(&self) -> u32 {
        let mut score = self.links.len();
        if (self.date != "-") {
            score += 1;
        }
        if (self.price != "-") {
            score += 1;
        }
        if (self.theme != "-") {
            score += 1;
        }
        score as u32
    }
}

pub struct MayballInfo {
    pub mayballs: Arc<Vec<Ball>>,
    pub springballs: Arc<Vec<Ball>>,
    pub winterballs: Arc<Vec<Ball>>
}

impl MayballInfo {
    pub fn new(balls: Vec<Ball>) -> Self {
        let mayballs: Vec<Ball> = balls.iter().filter(|ball| ball.season == Season::MAY).cloned().collect();
        let springballs: Vec<Ball> = balls.iter().filter(|ball| ball.season == Season::SPRING).cloned().collect();
        let winterballs: Vec<Ball> = balls.iter().filter(|ball| ball.season == Season::WINTER).cloned().collect();
        MayballInfo {
            mayballs: Arc::new(mayballs),
            springballs: Arc::new(springballs),
            winterballs: Arc::new(winterballs),
        }
    }
}

impl Serialize for Ball {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_date = NaiveDate::parse_from_str(&self.date, "%Y/%m/%d").map(|date| {
            return format!(
                "{}, {} {} {}",
                date.format("%A"),
                ordinal(date.day()),
                date.format("%B"),
                date.format("%Y")
            );
        }).unwrap_or(self.date.clone());


        let mut state = serializer.serialize_struct("Ball", 6)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("date", &formatted_date)?;
        state.serialize_field("theme", &self.theme)?;
        state.serialize_field("price", &self.price)?;
        state.serialize_field("season", &self.season)?;
        state.serialize_field("links", &self.links)?;
        state.end()
    }
}

fn ordinal(n: u32) -> String {
    match n % 100 {
        11 | 12 | 13 => format!("{}th", n),
        _ => match n % 10 {
            1 => format!("{}st", n),
            2 => format!("{}nd", n),
            3 => format!("{}rd", n),
            _ => format!("{}th", n),
        },
    }
}