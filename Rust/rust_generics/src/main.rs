#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/*-------------------------------------------------------------------
  rust_generics::main.rs
  ----------------------
  Generics allow you to implement onec collections and algorithms
  that work for many element types.
  This code:
  - Demonstrates creation and use of generic Rust types and functions
  - Rust uses struct instead of class to create instances
*/
/*-----------------------------------------------
Note:
Find all Bits code, including this in
https://github.com/JimFawcett/Bits
You can clone the repo from this link.
-----------------------------------------------*/

mod analysis_generic;     // identify module source file
use analysis_generic::*;  // import public functions and types
mod hello_generic;
use hello_generic::*;
mod stats;
use stats::*;
mod points_generic;
use points_generic::*;

use std::collections::HashMap;

/*---------------------------------------------------------
  We will demonstrate a generic type PointN<T>, defined
  in the module points_generic.rs.  PointN<T> implements:
  - methods supporting immutable and mutable indexing.
  - methods to convert a PointN<T> instance into a slice
    &T[m..n].
  We will also demonstrate a generic type Stats<T>,
  defined in the module stats.rs
---------------------------------------------------------*/

fn demo_std_generic_types() {

  show_label("demo standard generic types", 32);
  println!();

  show_op("arrays: [T; N]");
  println!();
  let arri = [1, 2, 3, 2, 1];
  println!("  {:?}", arri);
  let arrf = [1.0, 1.5, 2.0, 1.5, 1.0];
  println!("  {:?}\n", arrf);

  show_op("slices: &T[m..n]");
  println!();
  let slicei = &arri[1..5]; // [2, 3, 4, 5]
  println!("  {:?}", slicei);
  let slicef = &arrf[2..4]; // [2.0, 1.5]
  println!("  {:?}\n", slicef);

  show_op("vectors: Vec<T>");
  println!();

  /*-- vector of copy type --*/
  let mut v = Vec::<i32>::new();
  let s = &vec![1, 2, 3, 2, 1];
  v.extend_from_slice(s);
  /*
    This works because the elements are i32, i.e., copy.
    To load a vector with non-copy types use:
    v.extend(s.iter().cloned());
  */
  println!("  Vec<i32> {:?}", v);

  /*--- vector of tuple (i32, &str) --*/
  let mut v2 = Vec::<(i32,&str)>::new();
  v2.push((1, "one"));
  v2.push((2, "two"));
  println!("  Vec<(i32, &str)> {:?}\n", v2);

  show_op("maps: HashMap<K,V>");
  println!();
  let mut m = HashMap::<&str, i32>::new();
  m.insert("zero", 0);
  m.insert("one", 1);
  m.insert("two", 2);
  m.insert("three", 3);
  println!("  maps: HashMap<&str, i32>");
  println!("    {:?}", m);
}

fn demo_user_defined_generic_types() {
  
  demo_HelloGeneric();
  demo_stats();
  demo_pointn();
}

fn demo_generic_functions() {
  show_note("demo_generic_functions");
  println!();

  show_op("show_type<T:Debug>(_t, \"name\")");
  println!();

  let v = vec![1, 2, 3];
  show_type(&v, "v");
  let m = HashMap::<&str, i32>::new();
  show_type(&m, "m");
  println!();

  show_op("demo_indexer");
  println!();
  demo_indexer("[i32; 3]", &[1, 2, 3]);
  demo_indexer("Vec<i32>", &v);
  let p = Point::<f64, 3>::new().init(&vec![1.0, 2.0, -0.5]);
  demo_indexer("Point<f64, 3>", &p);
}
/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {

  show_label("generic functions and types", 35);

  demo_std_generic_types();
  demo_user_defined_generic_types();
  demo_generic_functions();
  
  print!("\n\n  That's all Folks!\n\n");
}
