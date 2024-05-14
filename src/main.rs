mod lib;

fn main() {
    let settings = lib::load_settings();

    let mut sum_a: f64 = 0.0;
    let mut cur_a: f64 = 1.0;

    let mut counter = 2;
    let separator = "-".repeat(50);

    for i in 1..=settings.accuracy {
        if settings.show_calculating {
            println!("{separator}");
        }

        if counter < 2 {
            sum_a += 1.0 / cur_a;
            counter = 2;

            if settings.show_calculating {
                println!("current action: +");
            }
        } else {
            sum_a -= 1.0 / cur_a;
            counter = 1;

            if settings.show_calculating {
                println!("current action: -");
            }
        }
        cur_a += 2.0;

        if settings.show_calculating {
            println!("current denominator: {cur_a}");
            println!("current step: {i}");
            println!("current sum: {sum_a}");
        }
    }

    let pi = 4.0 * sum_a * -1.0;

    println!("\nPI Number: {:.1$}", pi, settings.numbers_after_dot);
    let _ = std::io::stdin().read_line(&mut String::from(""));
}
