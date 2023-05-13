use chrono::{Local};
fn main() {
    let now = Local::now();
    println!("now is:\n  {now}");
    

    let df = now.format("  %a %b %e %Y, %T");
    println!("{}", df);
}
