/*-------------------------------------------------------------------
  analysis_generics.rs
  - provides analysis and display functions for Objects demo.
  - a few of these require advanced generics code.
  - You don't need to know how these work to understand this
    demo. 
  - We will come back to these functions in rust_generics. 
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
  Show enumerable input's values
  - 'a is an annotation saying that T's lifetime
    is as long as the function's lifetime.
  - I is the type of T's elements, that is coll:T<I>.
  - T can be any iterable type and both T and I must 
    satisfy Debug trait.
  - Does not consume input t since passed by reference.
*/
pub fn show_value_enum<T:Debug, I:Debug>(
  t: &T, nm: &str, left:usize, width:usize
) 
  where for<'a> &'a T: IntoIterator<Item = &'a I>
{
  println!("{nm:?} {{");
  // print!("{nm:?} {{\n");
  //show_fold(&t, left, width);
  show_fold(t, left, width);
  print!("}}");
  println!("\nsize: {}", std::mem::size_of::<T>());
}
/*---------------------------------------------------------
  show_indexer<T:Debug>(nm:&str, s:&[T])
  - accepts any collection that implements Deref to slice
  - that includes array [T;N], slice T[N], Vec<T>, PointN<T>
*/
#[allow(clippy::needless_range_loop)]
pub fn demo_indexer<T: Debug + Display>(nm:&str, s:&[T]) 
{
  print!("  {}", nm);
  let max = s.len();
  print!{"  [ {:?}", s[0]};
  /* 1..max is range iterator */
  for i in 1..max {
      print!(", {:?}", s[i]);
  }
  println!(" ]");
  /*---------------------------------------------
    The code above is not idiomatic Rust.
    Rust style prefers using iterators over indexing
    like this:
    for item in s.iter() {
      print!("{item} ");
    }
  */
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
// pub fn show_type_scalar<T:Debug>(t: &T, nm: &str) {
//   show_type(t, nm);
//   println!(
//     "value: {t:?}, size: {}", std::mem::size_of::<T>()
//   );
// }
/*---------------------------------------------------------
Show facts about an enumerable type's elements, e.g., 
name, type, values, and size.
- show_type is generic function with Debug bound.
  Using format "{:?} requires Debug."
*/
// pub fn show_type_enum<T:Debug, I:Debug>(t: &T, nm: &str, left:usize, width:usize) 
//   where for<'a> &'a T: IntoIterator<Item = &'a I>
// {
//   show_type(t, nm);
//   show_value_enum(t, nm, left, width);
// }
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
/*-- alternate implementation --*/
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
  
  for (i, item) in t.into_iter().enumerate() {
    accum += &format!("{item:?}, ");
    if ((i + 1) % width) == 0 && i != 0 {
        accum += "\n";
        accum += &offset(left);
    }
  }
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
  show string wrapped with long dotted lines above and below 
*/
pub fn show_label(note: &str, n:usize) {
  let line = 
    std::iter::repeat('-').take(n).collect::<String>();
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