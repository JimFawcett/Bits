/*-------------------------------------------------------------------
  analysis.rs
  - provides analysis and display functions for Objects demo.
-------------------------------------------------------------------*/

use std::fmt::*;

pub fn show_note(s:&str, n:usize) {
  let border = (0..n).map(|_| "-").collect::<String>();
  println!("{}", &border);
  println!("  {}", s);
  println!("{}", &border);
}

pub fn show_op(s:&str) {
  println!("--- {:?} ---", s);
}

/*---------------------------------------------------------
  Show input's call name and type
  - doesn't consume input
  - show_type is generic function with Debug bound.
    Using format "{:?}" requires Debug.
  - used in show_type_scalar
*/
pub fn show_type<T:Debug>(_t: &T, nm: &str) {
    let typename = std::any::type_name::<T>();
    println!("{nm:?}, type: {typename:?}");
}
