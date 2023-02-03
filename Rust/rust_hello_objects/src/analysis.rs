/*-------------------------------------------------------------------
  analysis.rs
  - provides analysis and display functions for the demo.
*/

use std::fmt::*;

/*
Show facts about a type's elements, e.g., name, type,
value, and size.
show_type is generic function with Debug bound.
Using format "{:?} requires Debug."
*/
pub fn show_type<T: std::fmt::Debug>(t: T, nm: &str) {
    let typename = std::any::type_name::<T>();
    println!("\ncall name: {:?}, type: {:?}", nm, typename);
    println!("value: {:?}, size: {}", t, std::mem::size_of::<T>());
}
/* indent print out by "left" spaces */
pub fn print_offset(left: usize) {
    for _i in 0..left {
        print!(" ");
    }
}
/* fold a vector slice into rows of w elements */
pub fn show_fold<T: Debug>(d: &Vec<T>, w: usize, left: usize) {
    println!();
    print_offset(left);
    for _i in 1..=d.len() {
        if _i < d.len() {
            print!("{:?}, ", d[_i - 1]);
        } else {
            print!("{:?}", d[_i - 1]);
            println!();
            break;
        }
        if _i % w == 0 && _i != 0 && _i != w - 1 {
            println!();
            print_offset(left);
        }
    }
}
/* show string wrapped with dotted lines above and below */
pub fn show_note(note: &str) {
    print!("\n-------------------------\n");
    print!("{}", note);
    print!("\n-------------------------\n");
}