#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/*-------------------------------------------------------------------
  rust_generics::main.rs
  - Demonstrates creation and use of generic Rust functions and structs
  - Rust uses struct instead of class to create objects
*/
mod analysis_generic;     // identify module source file
use analysis_generic::*;  // import public functions and types
mod points_generic;
use points_generic::*;
use std::{fmt::*, collections::HashMap};

/*---------------------------------------------------------
  Test: user-defined generic type
*/
#[derive(Debug, Clone)]  // compiler generated code
struct Test<T> 
  where T: Debug + Default + Clone
{
  datum: T,
}
impl<T> Test<T>
  where T: Debug + Default + Clone
{
  fn new(d:T) -> Self {
    Test::<T> {
      datum: d,
    }
  }
  fn default_new() -> Self {
    Test::<T> {
      datum: T::default(),
    }
  }
  /*  
    As show, value() is equivalent to making datum public.
    However value method supports code to modify the
    result.
  */
  fn value(&mut self) -> &mut T {
    &mut self.datum
  }
  fn show(&self) {
    println!("  Test {{ {:?} }}", self.datum);
  }
}

fn demo_generic_types() {
  
  show_note("demo_generic_types");
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
  let mut v = Vec::<i32>::new();
  v.push(1);
  v.push(2);
  v.push(3);
  v.push(2);
  v.push(1);
  println!("  Vec<i32> {:?}", v);
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
  println!("    {:?}\n", m);

  show_op("user-defined types");
  println!();
  let mut t = Test::<f64>::new(3.1415927);
  t.show();
  *t.value() = 42.0;
  t.show();
  let tc = t.clone();
  tc.show();
  println!();
  let mut p = PointN::<i32>::new(5)
         .init(vec![1, 2, 3, 2, 1]);
  p.show("p", 2, 12);
  *p.coors() = vec![1, 0, -1, 0, 1];
  p.show("p", 2, 12);
}
/*---------------------------------------------------------
  show_type(_t:&T, nm:&str)
  - Show input's call name and type
  - doesn't consume input
  - show_type is generic function with Debug bound.
    Using format "{:?}" requires Debug.
*/
pub fn show_type<T:Debug>(_t: &T, nm: &str) {
  let typename = std::any::type_name::<T>();
  println!("  {nm:?}, type: {typename:?}");
}
/*---------------------------------------------------------
  show_indexer<T:Debug>(nm:&str, s:&[T])
  - accepts any collection that implements Deref to slice
  - that includes array [T;N], slice T[N], Vec<T>, PointN<T>
*/
#[allow(clippy::needless_range_loop)]
fn demo_indexer<T: Debug + Display>(nm:&str, s:&[T]) 
{
  print!("  {}", nm);
  let max = s.len();
  print!{"  [ {:?}", s[0]};
  /* 1..max is range iterator */
  for i in 1..max {
      print!(", {:?}", s[i]);
  }
  println!(" ]");
  /*---------------------------
    clippy prefers no indexing:
    for item in s.iter().take(max) {
      print!("{item} ");
    }
  */
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
  let p = PointN::<f64>::new(3).init(vec![1.0, 2.0, -0.5]);
  demo_indexer("PointN<f64>", &p);
}
/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {

  show_label("demonstrate generic functions and types", 50);

  demo_generic_types();
  demo_generic_functions();
  // show_label(
  //   "std::library objects, string and vector<T>", 46
  // );
  // nl();

  // show_op("let mut s1 = \"a string\".to_string()");
  // let mut s1 = "a string".to_string();

  // show_op("show_type_scalar(&s1, \"&s1\")");
  // show_type_scalar(&s1, "&s1");
  // nl();

  // show_op("s1.push_str(\" and more\");");
  // s1.push_str(" and more");
  // show_type_scalar(&s1, "&s1");
  // nl();
  
  // show_op("let mut v:Vec<i32> = vec![1, 2, 3, 2, 1];");
  // let mut v: Vec<i32> = vec![1, 2, 3, 2, 1];
  // show_type_scalar(&v, "&v");
  // nl();
  
  // show_op("v.push(42);");
  // v.push(42);
  // show_type_scalar(&v, "&v");

  // show_label("instances of user-defined types", 35);
  // nl();

  // /*-----------------------------------------------------
  //   Type holding primitive arithmetic types, either
  //   i32 or f64.  Easy to extend to all integer and float
  //   types.
  // */
  // let v = vec![1.0, 2.5, -3.5, 4.0, 5.5];
  // let s = Stats::<f64>::new(v);
  // println!("s: {s:?}");
  // println!("s.max() = {:?}", s.max());
  // println!("s.min() = {:?}", s.min());
  // println!("s.sum() = {:?}", s.sum());
  // println!("s.avg() = {:?}", s.avg());
  // println!();
  
  // let v: Vec<i32> = vec![1, -2, 3, -4, 5];
  // let s: Stats<i32> = Stats::<i32>::new(v);
  // println!("s: {s:?}");
  // println!("s.max() = {:?}", s.max());
  // println!("s.min() = {:?}", s.min());
  // println!("s.sum() = {:?}", s.sum());
  // println!("s.avg() = {:?}", s.avg());
  // println!();

  // let mut p1 = PointN::<f64>::new(10);
  // p1.show("p1", 2, 7);

  // show_op("show_type(p1, \"p1\")");
  // show_type(&p1, "p1");

  // show_op("show_value_enum(p1.coors(), \"p1\", 2, 7)");
  // show_value_enum(p1.coors(), "p1.coors()", 2, 7);
  // nl();
  
  // show_op("p1.coors()[2] = 3.1415");
  // p1.coors()[2] = 3.1415;
  // p1.show("p1", 2, 7);

  // show_op("show_value_enum(p1.coors(), \"p1.coors()\", 2, 7)");
  // show_value_enum(p1.coors(), "p1.coors()", 2, 7);
  // nl();

  // let p2 = 
  //   PointN::<i32>::new(1).init(
  //     vec![0, 1, 2, 3, 4, 4, 3, 2, 1, 0]
  //   );
  // p2.show("p2", 2, 7);
  // nl();

  // show_op("show_type_scalar(p2, \"p2\")");
  // show_type_scalar(&p2, "p2");
  
  // println!("p2.len() = {:?}", p2.len());
  // nl();

  // show_op("p2.show(\"p2\", 2, 7)");
  // p2.show("p2", 2, 7);

  // show_label("Testing formats for collections", 35);
  // nl();

  // let v = 
  //   vec![
  //     0i32, -1, 2, -3, 4, -5, 6, -7, 8, -9, 
  //     10, -11, 12, -13, 14
  //   ];

  // show_op("show_type_scalar(&v, \"&v\")");
  // show_type_scalar(&v, "&v");
  // nl();

  // show_op("show_fold(&v, 2, 5)");
  // show_fold(&v, 2, 5);
  // nl();

  // show_op("let v = vec![0i32, ... ];");
  // show_op("fold(&v, 2, 12)");
  // let stmp = fold(&v, 2, 12);
  // println!("{stmp}");
  // nl();

  // show_op("let a = &[f64; 8];");
  // let a = 
  //   &[1.0, 2.5, -3.1, 4.3, 5.0, 6.0, 7.0, -8.0];
  // show_op("show_fold(a, 4, 5)");
  // show_fold(a, 4, 5);
  
  print!("\n\n  That's all Folks!\n\n");
}
