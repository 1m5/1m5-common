pub mod common {
    /// Maneuvering Condition
    #[derive(Debug)]
    pub enum ManCon {
        // 1DN Only w/ Random Configurable Delays: 10-100 Relays (~2sec-90days) / 20-200 Round-trip (~4sec-90days)
        NEO,
        // 1DN + I2P w/ Random Configurable Delays: 5 Relays (~1sec-6minutes) / 10 Round-trip (~2sec-1day)
        EXTREME,
        // I2P w/ Random Configurable Delays: 4 Relays (~800ms-6minutes) / 8 Round-trip (~1.8sec-12minutes)
        VERYHIGH,
        // I2P: 4 Relays (~800ms) / 8 Round-trip (~1.8sec)
        HIGH,
        // TOR: 3 Relays (~600ms) / 6 Round-trip (~1.4sec)
        MEDIUM,
        // VPN: 1 Relay (~200ms) / 2 Round-trip (~600ms)
        LOW,
        // HTTPS: 0 Relays
        NONE,
        // Unspecified
        UNKNOWN
    }
}