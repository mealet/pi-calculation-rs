use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
struct Settings {
    accuracity: u64,
    numbers_after_dot: usize,
    show_calculating: bool,
}

fn main() {
    let mut accuracity = 5000;
    let mut numbers_after_dot = 5;
    let mut show_calculating = false;

    let mut settingsFile = match File::open("settings.json") {
        Ok(t) => t,
        Err(e) => {
            let mut f = File::create("settings.json").unwrap();
            let mut defaultSettings = Settings {
                accuracity: 5000,
                numbers_after_dot: 5,
                show_calculating: false,
            };

            let mut json_string = serde_json::to_string(&defaultSettings).unwrap();
            f.write_all(json_string.as_bytes());

            File::open("settings.json").unwrap()
        }
    };
    let mut settings_reader = String::new();
    settingsFile.read_to_string(&mut settings_reader);
    let mut settings: Settings = serde_json::from_str(settings_reader.as_str()).unwrap();

    accuracity = settings.accuracity;
    numbers_after_dot = settings.numbers_after_dot;
    show_calculating = settings.show_calculating;

    let mut sum_a: f64 = 0.0;
    let mut cur_a: f64 = 1.0;

    let mut counter = 2;
    let separator = "-".repeat(50);

    for i in 1..=accuracity {
        if show_calculating {
            println!("{separator}");
        }

        if counter < 2 {
            sum_a += (1.0 / cur_a);
            counter = 2;

            if show_calculating {
                println!("current action: +");
            }
        } else {
            sum_a -= (1.0 / cur_a);
            counter = 1;

            if show_calculating {
                println!("current action: -");
            }
        }
        cur_a += 2.0;

        if show_calculating {
            println!("current denominator: {cur_a}");
            println!("current step: {i}");
            println!("current sum: {sum_a}");
        }
    }

    let pi = 4.0 * sum_a * -1.0;

    println!("\nPI Number: {:.1$}", pi, numbers_after_dot);
    std::io::stdin().read_line(&mut String::from(""));
}
