use super::*;

pub mod create;

pub type Name = String;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum When {
    Date(Vec<time::Date>),
    Day(Vec<Day>),
}

impl Bincoded for When {}

impl Validate for When {
    fn is_valid(&self) -> bool {
        match self {
            When::Date(dates) => {
                if dates.is_empty() {
                    return false;
                }

                if dates.len() > 14 {
                    return false;
                }

                for date in dates {
                    // Require unique dates
                    if dates.iter().filter(|&com_date| com_date == date).count() > 1 {
                        return false;
                    }
                }
            }
            When::Day(days) => {
                if days.is_empty() {
                    return false;
                }

                if days.len() > 7 {
                    return false;
                }

                for day in days {
                    // Require unique days
                    if days.iter().filter(|&com_day| com_day == day).count() > 1 {
                        return false;
                    }
                }
            }
        };

        true
    }
}

impl Validate for Name {
    fn is_valid(&self) -> bool {
        if self.len() < 3 || self.len() > 32 {
            return false;
        }

        // Require only alphanumeric characters and spaces
        if !self.chars().all(|c| c.is_alphanumeric() || c == ' ') {
            return false;
        }

        true
    }
}
