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
    let rest_m = now.elapsed().as_secs() / (60 * 5);
    let rest_s = (now.elapsed().as_secs() / 5) % 60;
    println!("Resting for {}m {}s", rest_m, rest_s);
    sleep(Duration::new(2, 0));

    let mut termdown_rest = Command::new("termdown")
        .arg(format!("{}m {}s", rest_m, rest_s))
        .spawn()
        .expect(
        "A problem occured while running termdown. Termdown must be installed to use this program.",
    );
    termdown_rest.wait().expect("An error occured");
}
