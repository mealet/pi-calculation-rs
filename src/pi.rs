pub fn calculate_pi(accuracy: u64) -> f32 {
    let mut sum_a: f32 = 0.0;
    let mut cur_a: f32 = 1.0;

    for i in 0..=accuracy {
        let numerator = if i % 2 == 0 { -1.0 } else { 1.0 };

        sum_a += numerator / cur_a;
        cur_a += 2.0;
    }

    (sum_a * 4.0).abs()
}
