use colour::Colour;

fn main() {
    println!("{}", "Hi my name is Nikolas Nikshiqi".truecolour(192));
    println!(
        "{}",
        "Hi my name is Nikolas Nikshiqi".truecolour_rgb(0, 170, 170)
    );

    println!(
        "{}",
        "Hi my name is Nikolas Nikshiqi"
            .bold()
            .bg_blue()
            .truecolour(192)
    );
    println!(
        "{}",
        "Hi my name is Nikolas Nikshiqi"
            .italic()
            .bg_truecolour_rgb(0, 170, 170)
            .bright_red()
    );
}
