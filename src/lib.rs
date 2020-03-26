/// Common Module
use std::convert::{TryFrom};

/// Maneuvering Condition
#[derive(Debug)]
pub enum ManCon {
    Unknown   = 0,
    None      = 1,
    Low       = 2,
    Medium    = 3,
    High      = 4,
    VeryHigh  = 5,
    Extreme   = 6,
    Neo       = 7,
}

impl From<ManCon> for u8 {
    fn from(original: ManCon) -> u8 {
        match original {
            ManCon::Unknown  => 0,
            ManCon::None   => 1,
            ManCon::Low   => 2,
            ManCon::Medium => 3,
            ManCon::High   => 4,
            ManCon::VeryHigh => 5,
            ManCon::Extreme => 6,
            ManCon::Neo => 7,
        }
    }
}

impl TryFrom<u8> for ManCon {
    type Error = ();
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(ManCon::Unknown),
            1 => Ok(ManCon::None),
            2 => Ok(ManCon::Low),
            3 => Ok(ManCon::Medium),
            4 => Ok(ManCon::High),
            5 => Ok(ManCon::VeryHigh),
            6 => Ok(ManCon::Extreme),
            7 => Ok(ManCon::Neo),
            _ => Err(())
        }
    }
}
