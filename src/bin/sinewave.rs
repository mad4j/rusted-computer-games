/// REMARKABLE PROGRAM BY DAVID AHL
fn main() {
    println!(
        r#"
        SINEWAVE
        CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY
    "#
    );

    let mut f = true;
    let mut t = 0f32;
    while t < 40f32 {
        let n = (26f32 + 25f32 * t.sin()) as usize;
        println!(
            "{}{}",
            " ".repeat(n),
            if f { "CREATIVE" } else { "COMPUTING" }
        );
        t = t + 0.25;
        f = !f;
    }
}
