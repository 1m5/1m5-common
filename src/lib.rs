/// Common Module

/// Maneuvering Condition
#[derive(Debug)]
pub enum ManCon {
    /// Unspecified
    Unknown     = 0,
    /// HTTPS: 0 Relays
    None        = 1,
    /// VPN: 1 Relay (~200ms) / 2 Round-trip (~600ms)
    Low         = 2,
    /// TOR: 3 Relays (~600ms) / 6 Round-trip (~1.4sec)
    Medium      = 3,
    /// I2P: 4 Relays (~800ms) / 8 Round-trip (~1.8sec)
    High        = 4,
    /// I2P w/ Random Configurable Delays: 4 Relays (~800ms-6minutes) / 8 Round-trip (~1.8sec-12minutes)
    VeryHigh    = 5,
    /// 1DN + I2P w/ Random Configurable Delays: 5 Relays (~1sec-6minutes) / 10 Round-trip (~2sec-1day)
    Extreme     = 6,
    /// 1DN Only w/ Random Configurable Delays: 5-10 Relays (~2sec-90days) / 10-20 Round-trip (~4sec-90days)
    Neo         = 7
}