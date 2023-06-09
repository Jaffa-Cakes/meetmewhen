use super::*;

pub mod create;
pub mod get;

pub type Name = String;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum When {
    Date(Vec<time::Date>),
    Day(Vec<time::Weekday>),
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
