use super::*;

pub type Package = Result<Res, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    InvalidBincode,
    BasicEventOrRespondentNotFound,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Req {
    pub id: i32,
    pub basic_event: String,
}

impl Bincoded for Req {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Res {}

impl Bincoded for Res {}
