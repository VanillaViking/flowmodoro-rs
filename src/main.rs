use std::{
    process::Command,
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let now = Instant::now();
    let mut termdown = Command::new("termdown").spawn().expect(
        "A problem occured while running termdown. Termdown must be installed to use this program.",
    );

    termdown.wait().expect("An error occured");
    let rest = now.elapsed().as_secs() / (60 * 5);
    println!("Resting for {} minutes...", rest);
    sleep(Duration::new(2, 0));

    let mut termdown_rest = Command::new("termdown")
        .arg(format!("{}m", rest))
        .spawn()
        .expect(
        "A problem occured while running termdown. Termdown must be installed to use this program.",
    );
    termdown_rest.wait().expect("An error occured");
}
