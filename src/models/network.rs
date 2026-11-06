pub struct AccessPoint {
    pub ssid: String,
    pub bssid: Option<String>,
    pub signal_dbm: Option<i32>,
    pub frequency_mhz: Option<u32>,
    pub auth: Vec<Authentication>,
}

pub struct SavedNetwork {
    pub name: String,
    pub auth: Authentication,
    pub is_autoconnect: bool,
    pub is_hidden: bool,
    // TODO: priority
}

pub enum Authentication {
    Open,
    Psk,
    //TODO: Eap,
    Sae,
}

pub enum AuthConfig {
    Open,
    Psk { passphrase: String },
    Sae { passphrase: String },
    //TODO: Eap(EapConfig),
}
