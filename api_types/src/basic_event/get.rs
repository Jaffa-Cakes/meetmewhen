use super::*;

#[derive(Deserialize, Serialize)]
pub struct Req {
    pub id: String,
}

impl Bincoded for Req {}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Res {
    pub id: String,
    pub name: Name,
    pub when: When,
    pub no_earlier: time::Time,
    pub no_later: time::Time,
    pub timezone: time::UtcOffset,
    pub created: time::OffsetDateTime,
}

impl Bincoded for Res {}
