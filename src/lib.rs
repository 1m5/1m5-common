/// Common Module

/// Maneuvering Condition
#[derive(Debug)]
pub enum ManCon {
    Unknown     = 0,
    None        = 1,
    Low         = 2,
    Medium      = 3,
    High        = 4,
    VeryHigh    = 5,
    Extreme     = 6,
    Neo         = 7
}

impl From<u8> for ManCon {
    fn from(num: u8) -> Self {
        match num {
            1 => ManCon::None,
            2 => ManCon::Low,
            3 => ManCon::Medium,
            4 => ManCon::High,
            5 => ManCon::VeryHigh,
            6 => ManCon::Extreme,
            7 => ManCon::Neo,
            _ => ManCon::Unknown
        }
    }
}