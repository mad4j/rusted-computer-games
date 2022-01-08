/// REMARKABLE PROGRAM BY DAVID AHL
/// original code:
///     https://www.atariarchives.org/basicgames/showpage.php?page=146
fn main() {
    println!(
        r#"
              SINE WAVE
        RUSTED COMPUTER GAMES

        "#
    );

    for t in 0..160 {
        let n = 26f32 + 25f32 * (0.25 * t as f32).sin();
        println!(
            "{}{}",
            " ".repeat(n as usize),
            [ "CREATIVE", "COMPUTING" ][t % 2]
        );
    }
}
