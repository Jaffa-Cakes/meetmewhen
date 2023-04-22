use super::*;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub type Name = String;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Availabilities(pub BTreeMap<u16, (BasicWhen, Vec<u16>)>);

impl Bincoded for Availabilities {}

impl Validate for Availabilities {
    fn is_valid(&self) -> bool {
        for (date, (_, slots)) in self.0.iter() {
            // Require unique days/dates
            if self
                .0
                .iter()
                .filter(|(&com_date, _)| com_date == *date)
                .count()
                > 1
            {
                return false;
            }

            // Require unique slots
            for slot in slots.iter() {
                if slots.iter().filter(|&&com_slot| com_slot == *slot).count() > 1 {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum BasicWhen {
    Date(time::Date),
    Day(time::Weekday),
}

impl Validate for Name {
    fn is_valid(&self) -> bool {
        if self.len() < 1 || self.len() > 32 {
            return false;
        }

        // Require only alphanumeric characters and spaces
        if !self.chars().all(|c| c.is_alphanumeric() || c == ' ') {
            return false;
        }

        true
    }
}
