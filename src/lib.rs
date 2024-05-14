use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct _settings {
    pub accuracy: u64,
    pub numbers_after_dot: usize,
    pub show_calculating: bool,
}

pub fn load_settings() -> _settings {
    let mut settingsFile = File::open("settings.json").unwrap_or_else(|_e| {
        let mut f = File::create("settings.json").unwrap();
        let defaultSettings = _settings {
            accuracy: 5000,
            numbers_after_dot: 5,
            show_calculating: false,
        };

        let json_string = serde_json::to_string(&defaultSettings).unwrap();
        let _ = f.write_all(json_string.as_bytes());

        File::open("settings.json").unwrap()
    });
    let mut settings_reader = String::new();
    let _ = settingsFile.read_to_string(&mut settings_reader);

    serde_json::from_str(settings_reader.as_str()).unwrap()
}
