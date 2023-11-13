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
  regions of memory, so they satisfy the Copy trait.

  Library types like String have a control block in stack
  and data in heap. So they do not satisfy the Copy trait.

  A move type can be cloned, but that requires an explicit 
  call to clone(). Otherwise, assignment and pass by value
  result in move which transfers ownership of heap 
  resources and invalidates the moved instance.

  Any attempt to use a moved instance results in compile
  failure.
---------------------------------------------------------*/

/*---------------------------------------------------------
  Demonstrate initialization of Rust's types
*/
fn show_formatted<T:Debug>(t:&T, nm:&str) {
  show_op(std::any::type_name::<T>());
  show_type(t, nm);
}
fn create_initialize() {
  show_note("create and initialize");
  nl();

  show_op("initialize language-defined types");
  nl();
  /*-- integer --------------------------------*/
  let int = 42i8;
  show_formatted(&int, "int");
  /*-- bool -----------------------------------*/
  let b = true;
  show_formatted(&b, "b");
  /*-- float ----------------------------------*/
  let double:f32 = 3.15927;
  show_formatted(&double, "double");
  /*-- character ------------------------------*/
  let c = 'z';
  show_formatted(&c, "c");
  /*-- literal string -------------------------*/
  let ls = "literal string";
  show_formatted(&ls, "ls");
  let second = ls.chars().nth(1);
  show_type(&second, "second");
  /*-- string ---------------------------------*/
  let st = "an owned string".to_string();
  show_formatted(&st, "st");
  let fourth = st.chars().nth(3);
  show_type(&fourth, "fourth");
  /*-- array ----------------------------------*/
  let arr = [1, 2, 3];
  show_formatted(&arr, "arr");
  let first = arr[0];
  show_type(&first, "first");
  /*-- tuple ----------------------------------*/
  let tp: (i32, f64, char) = (1, 2.5, 'a');
  show_formatted(&tp, "tp");
  let second = tp.1;
  show_type(&second, "tp.1");
  /*-- reference: integer ---------------------*/
  let iref: &i8 = &int;
  show_formatted(&iref, "iref");
  /*-- reference: array -----------------------*/
  let aref: &[i32; 3] = &arr;
  show_formatted(&aref, "aref");
  let second = aref[1];
  show_type(&second, "second");
  /*-- reference: slice of literal string -----*/
  let lscs = &ls[1..5];
  show_formatted(&lscs, "lscs");
  let second = lscs.chars().nth(1);
  show_type(&second, "second");
  /*-- reference: slice of array --------------*/
  let sla = &arr[1..3];
  show_formatted(&sla, "sla");
  let second = sla[1];
  show_type(&second, "second");
  nl();

  show_op("initialize in specific locations");
  nl();
  /*-- static memory --------------------------*/
  static PI:f64 = 3.1415927;
  show_type(&PI, "PI in static memory");
  println!("  address: {:p}\n", &PI);
  /*-- stack memory ---------------------------*/
  let f:f64 = 3.1415927;
  show_type(&f, "f in stack memory");
  println!("  address: {:p}\n", &f);
  let s:&str = "a literal string";
  show_type(&s, "s in stack memory");
  println!("  address: {:p}\n", &s);
  /*-- heap memory ----------------------------*/
  let g = Box::new(3.1415927f64);
  show_type(&g, "g in heap memory");
  println!("  address: {:p}", &*g);
  nl();

  show_op("initialize std::lib types");
  nl();
  /*-- Vec ------------------------------------*/
  let vec = vec![1, 2, 3, 2, 1];
  show_type(&vec, "vec");
  /*-- String ---------------------------------*/
  let st = "an owned string".to_string();
  show_type(&st, "st");
  /*-- mut VecDeque ---------------------------*/
  let mut vecdeq = VecDeque::<f64>::new();
  vecdeq.push_front(1.0);
  vecdeq.push_front(1.5);
  vecdeq.push_front(2.0);
  vecdeq.push_front(1.5);
  vecdeq.push_front(1.0);
  show_type(&vecdeq, "vecdeq");
  /* */
  let mut map = HashMap::<&str,i32>::new();
  map.insert("zero", 0);
  map.insert("one", 1);
  map.insert("two", 2);
  show_type(&map, "map");
  nl();

  show_op("initialize user-defined type");
  nl();
  #[derive(Debug)]
  struct Demo { name: String }
  let udt = Demo { name: "demo".to_string() };
  show_type(&udt, "udt");
}

/*---------------------------------------------------------
  pass argument by value
  - t is destination of move operation on caller's 
    argument
  - t mutated in function to demonstrate that there
    are no side effects from calling this function
*/
fn pass_by_val<T:Debug + Default>(mut t:T) { 
  show_type(&t,"T"); 
  t = T::default();
  println!("  value mutated internally to {:?}", t);
  // no side effect since t was copied or moved
}
/*---------------------------------------------------------
  pass argument by reference
  - t is reference to caller's argument
  - t mutated in function to demonstrate that there
    are side effects from calling this function
*/
fn pass_by_ref<T:Debug + Default>(t:&mut T) {
  show_type(&t,"T");
  *t = T::default();
  println!("  value mutated internally to {:?}", t);
  // has side effects since t is ref to caller's value
}
/*-- demonstrate variable has not been moved ------------*/
fn verify<T>(_t:T) {  // can only call this if t is valid
  /* t is valid */
}

/*-- demonstrate copy types -------------------------------
  Primitives and arrays of primitives are Copy types so
  construction, assignment, and pass-by-value result in
  copying entire source to destination.

  Most non-primitive types are not Copy, but provide a
  clone() function to make copies explicitly.
*/
fn demo_copy() {
  show_note("demonstrate copy and pass-by-value");
  nl();

  show_op("direct integer copy");
  let i_src = 42i64;  // copy literal 42 into i
  let i_dst = i_src;
  println!("  source: {i_src:?}");
  println!("  destin: {i_dst:?}");
  nl();
  
  show_op("direct copy of array of f64");
  let arr_src = [3.14f64, 0.5, -0.75];  // copy type
  let arr_dst = arr_src;
  println!("  source: {arr_src:?}");
  println!("  destin: {arr_dst:?}");
  nl();

  show_op("direct copy str");
  let lstrs = "literal string";  // copy type
  let lstrd = lstrs;
  println!("  source: {lstrs:?}, address: {:p}", lstrs);
  println!("  destin: {lstrd:?}, address: {:p}", lstrd);
  nl();
  println!("  Note: a literal string is a fixed value in memory.
  All access occurs through a reference, so copies just copy
  the reference. Both variables point to the same address." 
  );
  nl();

  show_op("string clone");
  let s_src = String::from("a string");
  let s_src_addr = &s_src;
  let s_src_bufaddr = std::ptr::addr_of!(s_src.as_bytes()[0]);
  println!("  s_src: {:?}, address: {:p}", s_src, s_src_addr);
  println!("  s_src_bufaddr: {:p}", s_src_bufaddr);
  let s_cln = s_src.clone();
  let s_cln_addr = &s_cln;
  let s_cln_bufaddr = std::ptr::addr_of!(s_cln.as_bytes()[0]);
  println!("  s_cln: {:?}, address: {:p}", s_cln, s_cln_addr);
  println!("  s_cln_bufaddr: {:p}", s_cln_bufaddr);
  nl();
  println!("  Note: s_src and s_cln have different addresses
  and their buffers have different addresses.
  So they are unique entities.");
  
  // let mut s_clnmut = s_src.clone();
  // let s_clnmut_addr = &s_clnmut;
  // let s_clnmut_bufaddr = std::ptr::addr_of!(s_clnmut.as_bytes()[0]);
  // println!("  s_clnmut: {:?}, address: {:p}", s_clnmut, s_clnmut_addr);
  // println!("  s_clnmut_buffaddr: {:p}", s_clnmut_bufaddr);
  // // let mut s_cln = s_src.clone();
  // s_clnmut.push_str(" plus more");
  // let s_clnmut_addr = &s_clnmut;
  // let s_clnmut_bufaddr = std::ptr::addr_of!(s_clnmut.as_bytes()[0]);
  // println!("  s_clnmut: {:?}, address: {:p}", s_clnmut, s_clnmut_addr);
  // println!("  s_clnmut_buffaddr: {:p}", s_clnmut_bufaddr);
  nl();

  show_op("direct vector clone");
  let v_src = vec![1, 2, 3, 2, 1];
  let v_dst = v_src.clone();
  // let v_dst = v_src;  moves v_src resources to v_dst
  // does not copy
  //dbg!(&v_dst);
  println!("  source: {v_src:?}");
  println!("  destin: {v_dst:?}");
  nl();
  
  show_op("pass-by-value copies integer");
  pass_by_val(i_src);
  nl();

  show_op("pass-by-value moves vector");
  pass_by_val(v_src);
  // println!("{:?}", v_src); fails to compile, v_scr moved

}

/*-- demonstrate references -------------------------------
  Illustrates  side affects of passing by reference
  - function pass_by_ref(&t) mutates t to illustrate
    that caller sees this change.
*/
fn demo_pass_by_ref() {
  show_label("demonstrate pass-by-reference", 50);

  show_label("these demos have side effects", 35);
  nl();

  show_op("pass integer reference");
  let mut i = 42i64;
  println!("  before pass_by_ref: {i:?}");
  pass_by_ref(&mut i); 
  println!("  after pass_by_ref: {i:?}");
  nl();

  show_op("pass reference to vector");
  let mut v = vec![1, 2, 3, 4, 5];
  println!("  before pass_by_ref: {v:?}");
  pass_by_ref(&mut v);
  println!("  after pass_by_ref: {v:?}");
  nl();

  show_op("pass reference to HashMap");
  let mut m = HashMap::<&str, i32>::new();
  m.insert("zero", 0);
  m.insert("one", 1);
  m.insert("two", 2);
  println!("  before pass_by_ref: {m:?}");
  pass_by_ref(&mut m);
  println!("  after pass_by_ref: {m:?}");
}

/*-- demonstrate move types -------------------------------
  Construction, assignment, and pass-by-value all result
  in moving the source resources to the destination.

  That is fast, usually only a few bytes are copied.

  This demo shows that move source and destination are 
  unique, but source and destination share same heap buffer.
*/
fn get_address<'a, T:Debug + AsRef<[u8]>>(t:&T) -> *const T {
  std::ptr::addr_of!(*t)
}
fn show_addresses<'a, T:Debug + AsRef<[u8]>>(t:&T, nm:&str) {
  println!("  {}: {:?}", nm, t);  // show value of t
  let addr_t = std::ptr::addr_of!(*t);
  println!("  static address of {}: {:?}", nm, addr_t);
  let byte_ref = t.as_ref();
  let addr_res = std::ptr::addr_of!(byte_ref[0]);
  println!("  heap address of {}'s buffer: {:?}", nm, addr_res);
}
fn demo_move() {
  show_note("demonstrate move");
  nl();

  show_op("demo_move for String");
  nl();
  show_op("let s = String::from(\"a string\")");
  let s = String::from("a string");
  let addrvec = &s;
  let addrzero = std::ptr::addr_of!(s.as_bytes()[0]);
  println!("address of s: {:p}", addrvec);
  println!("address of first byte of s&apos;s char buffer: {:p}\n", addrzero);

  show_op("let t:String = s; // move");
  let t = s;  // move
  let addrvec = &t;
  let addrzero = std::ptr::addr_of!(t.as_bytes()[0]);
  println!("address of t: {:p}", addrvec);
  println!("address of first byte of t&apos;s char buffer: {:p}\n", addrzero);

  println!("Note: s and t are unique objects");
  println!("that share same buffer");
  println!("but now, s is invalid");

  // show_op("demo_move for String");
  // let s = String::from("a string");
  // show_addresses(&s, "s");
  // let addrzero = std::ptr::addr_of!(s.as_bytes()[0]);
  // println!("  check - address of first byte of chars: {:?}", addrzero);

  // let t = s;  // move
  // show_op("let t:String = s; // move");
  // show_addresses(&t, "t");
  // nl();

  // println!("  Note: s and t are unique objects");
  // println!("  that share same buffer");
  // println!("  but now, s is invalid");
  nl();

  show_op("demo_move for Vec");
  nl();
  let u = vec![1, 2, 3, 2, 1];
  show_addresses(&u, "u");
  let addrzero = std::ptr::addr_of!(u[0]);
  println!("  check - address of u[0]: {:?}", addrzero);

  show_op("let v:Vec<u8> = u; // move");
  let v = u;  // move
  show_addresses(&v, "v");
}

/*---------------------------------------------------------
  Demonstrate mutability
*/
fn demo_mutable() {
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
/*---------------------------------------------------------
  Demonstrate references
  - illustrates that mutability cannot be shared using
    references
*/
fn demo_ref() {
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
  nl();

  show_op("attempt to mutate Vec while immutable ref exists");
  let mut v3 = vec![1, 2, 3];
  println!("  v3 capacity: {:?}", v3.capacity());
  let rv3 = &v3[1]; // ok to declare
  // uncomment next line to see build failure
  //v3.push(4);  // v3 will reallocate if capacity is 3
  println!("  r2v2: {rv3:?}"); // not ok to use
}
/*-- execute demonstrations -----------------------------*/
fn main() {
    show_note(
      "Demonstrate Rust types"
    );
    
    create_initialize();
    demo_copy();
    demo_pass_by_ref();
    demo_move();
    demo_mutable();
    demo_ref();

    println!("\n\nThat's all Folks!!\n\n");
}
