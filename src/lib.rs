/// Common Module

/// Maneuvering Condition
#[derive(Debug)]
pub enum ManCon {
    Unknown,
    None,
    Low,
    Medium,
    High,
    VeryHigh,
    Extreme,
    Neo
}

impl ManCon {
    pub fn string(&self) -> &str {
        match *self {
            ManCon::Unknown => "Unknown",
            ManCon::None => "None",
            ManCon::Low => "Low",
            ManCon::Medium => "Medium",
            ManCon::High => "High",
            ManCon::VeryHigh => "VeryHigh",
            ManCon::Extreme => "Extreme",
            ManCon::Neo => "Neo"
        }
    }
    pub fn from(mancon: &str) -> ManCon {
        match mancon {
            "None" => ManCon::None,
            "Low" => ManCon::Low,
            "Medium" => ManCon::Medium,
            "High" => ManCon::High,
            "VeryHigh" => ManCon::VeryHigh,
            "Extreme" => ManCon::Extreme,
            "Neo" => ManCon::Neo,
            _ => ManCon::Unknown
        }
    }
}