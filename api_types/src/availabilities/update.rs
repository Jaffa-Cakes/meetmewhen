use super::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Req {
    pub id: i32,
    pub basic_event: String,
    pub name: String,
    pub availabilities: Availabilities,
}

impl Bincoded for Req {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Res {
    pub id: i32,
}

impl Bincoded for Res {}
