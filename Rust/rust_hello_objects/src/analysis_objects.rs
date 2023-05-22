/*-------------------------------------------------------------------
  analysis.rs
  - provides analysis and display functions for Objects demo.
  - a few of these require advanced generics code.
  - You don't need to know how these work to understand this
    demo. 
  - We will come back to these functions in a later Bit. 
-------------------------------------------------------------------*/

use std::fmt::*;

/*---------------------------------------------------------
  Show input's call name and type
  - doesn't consume input
  - show_type is generic function with Debug bound.
    Using format "{:?}" requires Debug.
*/
pub fn show_type<T:Debug>(_t: &T, nm: &str) {
    let typename = std::any::type_name::<T>();
    println!("call name: {nm:?}, type: {typename:?}");
}
/*---------------------------------------------------------
  Show facts about a type's elements, e.g., name, type,
  value, and size.
  - show_type is generic function with Debug bound.
    Using format "{:?} requires Debug."
  - works with small enumerable collections too because
    {:?} knows how to format them, but won't fold long
    sequences of elements. Use show_value_enum for that.
*/
pub fn show_type_scalar<T:Debug>(t: &T, nm: &str) {
    show_type(t, nm);
    println!(
        "value: {t:?}, size: {}", std::mem::size_of::<T>()
    );
}
/*--------------------------------------------------------- 
  build indent string with "left" spaces 
*/
pub fn offset(left: usize) -> String {
    let mut accum = String::new();
    for _i in 0..left {
        accum += " ";
    }
    accum
}
/*---------------------------------------------------------
  find index of last occurance of chr in s
  - returns option in case chr is not found
  https://stackoverflow.com/questions/50101842/how-to-find-the-last-occurrence-of-a-char-in-a-string
*/
fn find_last_utf8(s: &str, chr: char) -> Option<usize> {
  /*-- commented lines are equivalent to below --*/
    // if let Some(rev_pos) = 
    //   s.chars().rev().position(|c| c == chr) {
    //     Some(s.chars().count() - rev_pos - 1)
    // } else {
    //     None
    // }
    s.chars().rev().position(|c| c== chr)
     .map(|rev_pos| s.chars().count() - rev_pos - 1)
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