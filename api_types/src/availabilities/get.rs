use super::*;

pub type Package = Result<Res, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    InvalidBincode,
    BasicEventNotFound,
}

#[derive(Deserialize, Serialize)]
pub struct Req {
    pub basic_event: String,
}

impl Bincoded for Req {}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Res {
    pub respondents: Vec<Respondent>,
}

impl Bincoded for Res {}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Respondent {
    pub id: i32,
    pub name: String,
    pub availabilities: Availabilities,
}
