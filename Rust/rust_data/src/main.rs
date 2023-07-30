#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/* rust_data::main.rs */
/*
  Static Data Types:
    bool, char
    i8, i16, i32, i64, i128, isize
    u8, u16, u32, u64, u128, usize
    f32, f64
    (), reference
    array, tuple, struct, enum
  Qualifiers:
    mutable, const

  Additional Library Types:
    std::String, std::Vec<T>, std::HashMap<K,V>,
    std::VecDeque<T>, std::BTreeSet<T>, std::BTreeMap<K,V>,
    std::LinkedList<T>, std::BinaryHeap<T>

  Operations:
    Primitive types can all be copied.
    Most library and user-defined types can be moved,
    cloned, but only aggregates with all copyable members
    can be copied.  Values can only be changed if declared
    mutable.

  Processing:
    All types are static, operations run as native code,
    and no garbage collection is needed. Resources are
    returned at end of declr scope.
*/
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
fn show_type<T: Debug>(t: &T, nm: &str) {
  let typename = std::any::type_name::<T>();
  print!("  {nm}, {typename}");
  println!(
    "\n  value: {:?}, size: {}", 
    t, std::mem::size_of::<T>()
  );
}
/*-- pass argument by value -----------------------------*/
fn pass_by_val<T:Debug>(t:T) {
  show_type(&t,"T");
  // println!("  value is {:?}", t);
}
/*-- pass argument by reference -------------------------*/
fn pass_by_ref<T:Debug>(t:&T) {
  show_type(&t,"T");
  // println!("  value is {:?}", t);
}
/*-- demonstrate variable has not been moved ------------*/
fn verify<T>(_t:T) {  // can only call this if t is valid
  /* t is valid */
}
/*-- demonstrate copy types -----------------------------*/
fn demo_copy<T:Debug + Copy>(t:T) {
  let tc = t;     // copy
  pass_by_val(tc);    // pass by value - copy for copy type
  pass_by_ref(&tc);   // pass by ref - no copy of tc
  verify(tc);
}
fn execute_demo_copy() {
  let i = 42i64;  // copy literal 42 into i
  println!("execute_demo_copy for i64");
  demo_copy(i);
  println!("  copied i: {i:?}");
  
  let arr = [3.14f64, 0.5, -0.75];  // copy type
  println!("execute_demo_copy for array of f64");
  demo_copy(arr);
  println!("  copied arr: {arr:?}");
  println!();
}
/*-- demonstrate move types -----------------------------*/
fn demo_move<T:Debug + Clone>(t:T) {
  let tc = t;             // move
  pass_by_val(tc.clone());   // pass by value - moves clone
  pass_by_ref(&tc);          // pass by ref - no move of tc
  verify(tc);
}

fn execute_demo_move() {
  println!("execute_demo_move for String");
  let s = "a string".to_owned();
  demo_move(s);
  // Try uncommenting line below
  // println!("  moved arr: {s:?}");  // s moved, can't print
  
  println!("execute_demo_move for Vec");
  let v = vec![1, 2, 3];
  demo_move(v);
  // println!("  moved v: {v:?}");  // v moved, can't print

  use std::collections::HashMap;
  println!("execute_demo_move for HashMap");
  let mut m = HashMap::<&str,i32>::new();
  m.insert("zero", 0);
  m.insert("one", 1);
  m.insert("two", 2);
  demo_move(m);
  // println!("  moved m: {m:?}");  // m moved, can't print
  println!();
}
/*-- demonstrate mutability -----------------------------*/
fn execute_demo_mutable() {
  println!("execute_demo_mutable");
  let mut i = 42i32;
  print!("  original value: {i:?}");
  i += 1;
  println!(", changed value:  {i:?}");
  
  let mut f = 3.14159f64;
  print!("  original value: {f:?}");
  f += 1.0;
  println!(", changed value:  {f:?}");
  
  let mut v = vec![1, 2, 3];
  print!("  original value: {v:?}");
  v[1] += 1;
  println!(", changed value:  {v:?}");

  use std::collections::HashMap;
  let mut m = HashMap::<&str,i32>::new();
  m.insert("zero", 0);
  m.insert("one", 1);
  m.insert("two", 2);
  println!("  original value: {m:?}");
  /* entry returns reference to value of supplied key */
  let mut value = m.entry("one").or_insert(1);
  *value += 1;
  println!("  changed value:  {m:?}")
}
fn main() {
    println!(
        "\n Demonstrate Rust types\n\
       ------------------------"
    );
    println!();
    
    execute_demo_copy();
    execute_demo_move();
    execute_demo_mutable();
    /*
      Primitive data types: i32, f64, ... occupy contiguous
      regions of memory, so they satisfy the copy trait.

      Library types like String have a control block in stack
      and data in heap. So they do not satisfy the copy trait.

      A move type can be cloned, but that requires an explicit 
      call to clone(). Otherwise, assignment and pass by value
      result in move which transfers ownership of heap 
      resources and invalidates the moved instance.

      Any attempt to use a moved instance results in compile
      failure.
    */

    println!("\n\nThat's all Folks!!\n\n");
}
/*
  show_type is generic function with Debug bound.
  Using format "{:?}" requires Debug.
  - mem::size_of::<T> measures size of T in stackframe.
    It does not measure size of resources on heap.
*/
// fn show_type<T: Debug>(t: &T, nm: &str) {
//   let typename = std::any::type_name::<*const T>();
//   print!("  {nm}, {typename}");
//   println!(
//     "  value: {:?}, size: {}", 
//     t, std::mem::size_of::<T>()
//   );
// }
// fn show_type_move<T: Debug>(t: T, nm: &str) {
//   let typename = std::any::type_name::<T>();
//   print!("  {nm}, {typename}");
//   println!(
//     "  value: {:?}, size: {}", 
//     t, std::mem::size_of::<T>()
//   );
// }

