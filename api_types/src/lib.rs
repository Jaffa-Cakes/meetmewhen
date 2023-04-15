use serde::{Deserialize, Serialize};

pub mod basic_event;

pub trait Validate {
    fn is_valid(&self) -> bool;
}

pub trait Bincoded {
    fn to_bincode(&self) -> Vec<u8>
    where
        Self: Serialize,
    {
        bincode::serialize(self).expect("bincode serialization failed")
    }

    fn from_bincode(bytes: &[u8]) -> Result<Self, ()>
    where
        Self: for<'a> Deserialize<'a>,
    {
        match bincode::deserialize(bytes) {
            Ok(value) => Ok(value),
            Err(_) => Err(()),
        }
    }
}

pub mod prelude {
    pub use super::{Bincoded, Validate};
}
