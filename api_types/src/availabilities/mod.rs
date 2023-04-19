use super::*;

pub mod create;
pub mod get;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Availabilities(pub BTreeMap<u16, (BasicWhen, Vec<u16>)>);

impl Bincoded for Availabilities {}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum BasicWhen {
    Date(time::Date),
    Day(time::Weekday),
}
