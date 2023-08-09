/*---------------------------------------------------------
 rust_data::src::main.rs 
 - demonstrate move, copy, and borrow for
   primitives and selected std::library types
---------------------------------------------------------*/
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::approx_constant)]
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

mod bits_data_analysis;
use bits_data_analysis::*;
use std::collections::*;

use std::fmt::Debug;

/*---------------------------------------------------------
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
---------------------------------------------------------*/

/*-- demonstrate initialization of Rust's types ---------*/
fn create_initialize() {
  show_note("create and initialize");
  nl();

  show_op("initialize primitives");
  let int = 42i64;
  show_type(&int, "int");
  let double:f64 = 3.15927;
  show_type(&double, "double");
  let arr = [1, 2, 3];
  show_type(&arr, "arr");
  nl();

  show_op("initialize std::lib types");
  let vec = vec![1, 2, 3, 2, 1];
  show_type(&vec, "vec");
  let mut map = HashMap::<&str,i32>::new();
  map.insert("zero", 0);
  map.insert("one", 1);
  map.insert("two", 2);
  show_type(&map, "map");
  nl();

  show_op("initialize user-defined type");
  #[derive(Debug)]
  struct Demo { name: String }
  let udt = Demo { name: "demo".to_string() };
  show_type(&udt, "udt");
}

/*---------------------------------------------------------
  pass argument by value
  - t is destination of move operation on caller's 
    argument
*/
fn pass_by_val<T:Debug>(t:T) { 
  show_type(&t,"T"); 
}
/*-- pass argument by reference -------------------------*/
fn pass_by_ref<T:Debug>(t:&T) {  // t is ref to caller's arg
  show_type(&t,"T");
}
/*-- demonstrate variable has not been moved ------------*/
fn verify<T>(_t:T) {  // can only call this if t is valid
  /* t is valid */
}
/*-- demonstrate copy types -----------------------------*/
fn demo_copy<T:Debug + Copy>(t:T) {
  let tc = t;      // copy
  pass_by_val(tc);    // pass by value - copy for copy type
  pass_by_ref(&tc);   // pass by ref - no copy of tc
  verify(tc);
}
fn execute_demo_copy() {
  show_note("demonstrate copy");
  nl();

  let i = 42i64;  // copy literal 42 into i
  show_op("demo_copy for i64");
  demo_copy(i);
  println!("  copied i: {i:?}\n");
  
  let arr = [3.14f64, 0.5, -0.75];  // copy type
  show_op("demo_copy for array of f64");
  demo_copy(arr);
  println!("  copied array arr: {arr:?}");
}
/*-- demonstrate move types -----------------------------*/
fn demo_move<T:Debug + Clone>(t:T) {  // t is dest of move
  let tc = t;              // t moved to tc
  pass_by_val(tc.clone());    // pass by value - moves clone
  pass_by_ref(&tc);           // pass by ref - no move of tc
  verify(tc);                 // tc valid
}
fn execute_demo_move() {
  show_note("demonstrate move");
  nl();

  show_op("demo_move for String");
  let s = "a string".to_owned();
  demo_move(s);
  println!("  moved String s");
  // Try uncommenting line below
  // println!("  moved s: {s:?}");  // s moved, can't print
  println!();
  
  show_op("demo_move for Vec");
  let v = vec![1, 2, 3];
  demo_move(v);
  println!("  moved Vec v");
  // println!("  moved v: {v:?}");  // v moved, can't print
  println!();

  show_op("demo_move for HashMap");
  let mut m = HashMap::<&str,i32>::new();
  m.insert("zero", 0);
  m.insert("one", 1);
  m.insert("two", 2);
  demo_move(m);
  println!("  moved HashMap m");
  // println!("  moved m: {m:?}");  // m moved, can't print
}
/*-- demonstrate mutability -----------------------------*/
fn execute_demo_mutable() {
  show_note("demonstrate mutability");
  nl();

  show_op("demo_mutable for primitives");
  let mut i = 42i32;
  print!("  original value: {i:?}");
  i += 1;
  println!(", changed value:  {i:?}");
  
  let mut f = 3.14159f64;
  print!("  original value: {f:?}");
  f += 1.0;
  println!(", changed value:  {f:?}");
  nl();

  show_op("demo_mutable for std::lib types");
  let mut v = vec![1, 2, 3];
  print!("  original value: {v:?}");
  v[1] += 1;
  println!(", changed value:  {v:?}");

  let mut m = HashMap::<&str,i32>::new();
  m.insert("zero", 0);
  m.insert("one", 1);
  m.insert("two", 2);
  println!("  original value: {m:?}");
  /* entry returns reference to value of supplied key */
  let mut value = m.entry("one").or_insert(1);
  *value += 1;
  println!("  changed value:  {m:?}\n");
}
/*-- demonstrate references -----------------------------*/
fn execute_demo_ref() {
  show_note("demonstrate references");
  nl();

  show_op("demo_ref immutable");
  let v1 = vec![1, 2, 3, 2, 1];
  /* can use any number of immutable referenes */
  let r1v1 = &v1;
  println!("  r1v1: {r1v1:?}");
  let r2v1 = &v1;
  println!("  r2v1: {r2v1:?}");
  nl();

  show_op("demo_ref mutable");
  /* can't mutate data through shared references */
  let mut v2 = vec![1, 2, 3, 4, 5];
  /* immutable borrow */
  let r1v2 = &v2;
  println!("  r1v2: {r1v2:?}");
  /* mutable borrow ok if we don't use immutable borrow later */
  let mr2v2 = &mut v2;
  mr2v2.push(42);
  println!("  mr2v2: {mr2v2:?}");
  /*
    Use of immutable borrow in println! illegal with active 
    mutable borrow. Try uncommenting line below.
  */
  // println!("  r1v2: {r1v2:?}");
}
/*-- execute demonstrations -----------------------------*/
fn main() {
    show_note(
      "Demonstrate Rust types"
    );
    
    create_initialize();
    execute_demo_copy();
    execute_demo_move();
    execute_demo_mutable();
    execute_demo_ref();

    println!("\n\nThat's all Folks!!\n\n");
}
