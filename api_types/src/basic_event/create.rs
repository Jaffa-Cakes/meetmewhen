use super::*;

pub type Package = Result<Res, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    InvalidBincode,
    InvalidRequest,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Req {
    pub name: Name,
    pub when: When,
    pub no_earlier: time::Time,
    pub no_later: time::Time,
    pub timezone: time::UtcOffset,
}

impl Bincoded for Req {}

impl Validate for Req {
    fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        if !self.when.is_valid() {
            return false;
        }

        if self.no_earlier >= self.no_later {
            return false;
        }

        true
    }
}

#[derive(Deserialize, Serialize)]
pub struct Res {
    pub id: String,
}

impl Bincoded for Res {}
