#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/* rust_data::bits_data_analysis.rs */
/*-----------------------------------------------
Note:
Find all Bits code, including this in
https://github.com/JimFawcett/Bits
You can clone the repo from this link.
-----------------------------------------------*/

use std::fmt::Debug;

/*-- show_type --------------------------------------------
  Shows compiler recognized type and data value
*/
pub fn show_type<T: Debug>(t: &T, nm: &str) {
  let typename = std::any::type_name::<T>();
  print!("  {nm}, {typename}");
  println!(
    "\n  value: {:?}, size: {}",  // smart formatting {:?}
    t, std::mem::size_of::<T>()   // handles both scalars
  );                              // and collections
}
/*---------------------------------------------------------
  show string wrapped with long dotted lines above and below 
*/
pub fn show_label(note: &str, n:usize) {
  let mut line = String::new();
  for _i in 0..n {
    line.push('-');
  }
  print!("\n{line}\n");
  print!("  {note}");
  print!("\n{line}\n");
}
pub fn show_label_def(note:&str) {
  show_label(note, 50);
}
/*---------------------------------------------------------
  show string wrapped with dotted lines above and below 
*/
pub fn show_note(note: &str) {
  print!("\n-------------------------\n");
  print!(" {note}");
  print!("\n-------------------------\n");
}
/*---------------------------------------------------------
  show string wrapped in short lines
*/
pub fn show_op(opt: &str) {
  println!("--- {opt} ---");
}
/*---------------------------------------------------------
  print newline
*/
pub fn nl() {
  println!();
}