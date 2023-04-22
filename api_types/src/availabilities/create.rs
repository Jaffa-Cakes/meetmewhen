use super::*;

pub type Package = Result<Res, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    BasicEventNotFound,
    InvalidRequest,
    InvalidBincode,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Req {
    pub basic_event: String,
    pub name: Name,
    pub availabilities: Availabilities,
}

impl Bincoded for Req {}

impl Validate for Req {
    fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        if !self.availabilities.is_valid() {
            return false;
        }

        true
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Res {
    pub id: i32,
}

impl Bincoded for Res {}
