use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Deserialize, PartialEq, Eq, Serialize)]
pub enum Season {
    WINTER,
    SPRING,
    MAY,
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    text: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Ball {
    pub name: String,
    pub date: String,
    pub theme: String,
    pub price: String,
    pub season: Season,
    pub links: Vec<Link>,
}

impl Serialize for Ball {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_date = self.date.format("%B %e %Y").to_string();
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