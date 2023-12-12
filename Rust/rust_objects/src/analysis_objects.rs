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
  show string wrapped with long dotted lines above and below 
*/
pub fn show_note(note: &str, n:usize) {
  let mut line = String::new();
  for _i in 0..n {
    line.push('-');
  }
  print!("\n{line}\n");
  print!("  {note}");
  print!("\n{line}\n");
}
/*---------------------------------------------------------
  label with default length 
  - Rust does not support default argument values
*/
pub fn show_label(note:&str) {
  show_note(note, 50);
}
/*---------------------------------------------------------
  show string wrapped in short lines
*/
pub fn show_op(opt: &str) {
  println!("--- {opt} ---");
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
  fold an enumerable's elements into rows of w elements
  - indent by left spaces
  - does not consume t since passed as reference
  - returns string
  https://users.rust-lang.org/t/generic-code-over-iterators/10907/3
*/
pub fn fold<T, I:Debug>(
  t: &T, left: usize, width: usize
) -> String
    where for<'a> &'a T: IntoIterator<Item = &'a I>, T:Debug
{
  let mut accum = String::new();
  accum += &offset(left);

  for (i, item) in t.into_iter().enumerate() {
    accum += &format!("{item:?}, ");
    if ((i + 1) % width) == 0 && i != 0 {
        accum += "\n";
        accum += &offset(left);
    }
  }
  /*-- Alternate direct implementation --*/
  //let mut i = 0usize;
  // for item in t {
  //   accum += &format!("{item:?}, ");
  //   if ((i + 1) % width) == 0 && i != 0 {
  //       accum += "\n";
  //       accum += &offset(left);
  //   }
  //   i += 1;
  // }

  /*-- strip off trailing comma --*/
  let opt = find_last_utf8(&accum, ',');
  if let Some(index) = opt {
    accum.truncate(index);
  }
  accum
}
/*---------------------------------------------------------
  show enumerables's elements as folded rows
  - width is number of elements in each row
  - left is indent from terminal left
*/
pub fn show_fold<T:Debug, I:Debug>(t:&T, left:usize, width:usize) 
  where for<'a> &'a T: IntoIterator<Item = &'a I>
{
  println!("{}",fold(t, left, width));
}
/*---------------------------------------------------------
  print newline
*/
pub fn nl() {
  println!();
}