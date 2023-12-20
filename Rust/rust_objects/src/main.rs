#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/*---------------------------------------------------------
  rust_objects::main.rs
  - Demonstrates creation and use of Rust objects
  - Rust uses struct instead of class to create objects
*/
/*-----------------------------------------------
Note:
Find all Bits code, including this in
https://github.com/JimFawcett/Bits
You can clone the repo from this link.
-----------------------------------------------*/

mod analysis_objects;     // identify module source file
use analysis_objects::*;  // import public functs and types
mod points_objects;       // identify module source file
use points_objects::*;    // import public functs and types

fn demo_string_objects() {
/*-------------------------------------------------------------------
  demo standard String:
  - create and initialize instances
  - move construction
  - move assignment
  - selected operations
*/
  show_note("demo std vector", 40);
  let mut s:String = "a string".to_string();
  println!("  s: {:?}\n", s);
  
  show_op("let s1 = s : move construction");
  let mut s1 = s;  // move construction
  println!("s1: {:?}\n", s1);
  // s now invalid, been moved : uncomment to see error msg
  // println!("{:?}", s);
  
  show_op("s1 += \" and more\" : one of many operations");
  s1 += " and more";
  println!("s1: {:?}\n", s1);

  show_op("s = s1.clone : move assignment of clone");
  s = s1.clone();
  // clone copies s1's resources so s1 is still valid
  println!("s: {:?}", s);
  println!("s1: {:?}\n", s1);
  
  show_op("s1 = s1 + \" words\"");
  s1 += " words";
  show_op("s = s1 : move assignment");    
  s = s1;
  println!("s: {:?}\n", s);
  // s1 now invalid, been moved
  
  // String stored in heap
  show_op("let mut h_str = Box::new(\"...\")");
  let mut h_str = Box::new("heap string".to_string());
  println!("h_str: {:?}", h_str);
  
  *h_str += " and some more";
  println!("h_str: {:?}", h_str);
  
}

/*---------------------------------------------------------
  demo standard Vector
  - create and initialize instances
  - move construction
  - move assignment
  - selected operations
*/
fn demo_vector_objects() {
  show_note("demo std vector", 40);
  let mut v: Vec<i32> = vec![1, 2, 3, 2, 1];
  println!("  v: {:?}\n", v);
  
  show_op("let v1 = v : move construction");
  let mut v1 = v;  // move construction
  println!("  v1: {:?}\n", v1);
  // v now invalid, been moved : uncomment to see error msg
  // println!("{:?}", v);
  
  show_op("v1[1] = -2 : one of many operations");
  v1[1] = -2;
  println!("  v1: {:?}\n", v1);

  show_op("v = v1.clone : move assignment of clone");
  v = v1.clone();
  // clone copies v1's resources so v1 is still valid
  println!("  v: {:?}", v);
  println!("  v1: {:?}\n", v1);
  
  show_op("a = &v[1..3]");
  // create array from vector slice
  let a: &[i32] = &v[1..3];
  println!("  a: {:?}\n", a);
  
  show_op("v2 is collect from slice &v[1..3]");
  let v2 = (1..3).map(|i| v1[i]).collect::<Vec<i32>>();
  println!("  v2: {:?}\n", v2);

  show_op("v = v2 : move assignment");    
  v = v2;
  println!("  v: {:?}\n", v);
  // v2 now invalid, been moved
  
  // Vec stored in heap
  show_op("let mut h_vec = Box::new(\"...\")");
  let mut h_vec = Box::new(vec![1, 2, 3, 4, 5]);
  println!("  h_vec: {:?}\n", h_vec);
  
  show_op("(*h_vec)[2] += 2");
  (*h_vec)[2] += 2;
  println!("  h_vec: {:?}", h_vec);
  
  show_op("h_vec[2] += 2, uses auto deref");
  h_vec[2] += 2;
  println!("  h_vec: {:?}\n", h_vec);

}
/*---------------------------------------------------------
  demo user-defined type Point4D
  - create and initialize instances
  - copy construction
  - copy assignment
  - selected operations
*/
fn demo_user_defined_point4d() {

  show_note("instances of user-defined type Point4D", 35);
  println!();

  /*-- create instance of Point4D ---*/
  show_op("let mut p1 = Point4D::new()");
  let mut p1 = Point4D::new();
  p1.show("p1", 2);
  p1.set_x(42.0);
  p1.set_y(-3.0);
  p1.set_z(2.0);
  p1.show("p1", 2);
  /*-- show p1's type ---------------*/
  p1.show("p1", 2);
  println!();

  show_op("using Debug trait with println!");
  println!("p1: {:?}\n", p1);

  /*-- show one of its operations ---*/
  show_op("let p1a = p1.clone()");
  let p1a = p1.clone();
  p1a.show("p1.clone()", 2);   

  println!("\n  using PointN<f64>::coor_x function:");
  *(p1.coor_x()) = 84.0;
  p1.show("p1", 2);

  println!("\n  using PointN<f64>::coor_y function:");
  *p1.coor_y() = 84.0; 
  p1.show("p1", 2);

  println!("\n  using PointN<f64>::coor_z function:");
  let rz = p1.coor_z(); 
  *rz = 84.0;
  p1.show("p1", 2);

  println!("\n  updating time value:");
  /*-- delay 5 secs for update_time demo --*/
  println!("  delaying 5 seconds before time update");
  use std::time::Duration;
  let dur:Duration = Duration::from_secs(5);
  std::thread::sleep(dur);

  p1.update_time();
  p1.show("p1", 2);

  show_op("let p2 = p1 : copy construction");
  let mut p2 = p1;
  p2.show("p2", 2);
  p1.show("p1", 2);

  show_op("*p2.coor_x() = 42.0");
  *p2.coor_x() = 42.0;
  p2.show("p2", 2);

  show_op("p1 = p2 : copy assignment");
  p1 = p2;
  p1.show("p1", 2);
  p2.show("p2", 2);
  println!();

  show_note("Point4D objects in heap", 40);
  let mut h_point = Box::new(Point4D::new());
  // show_type(&h_point, "h_point");
  /*
    Point4D does not implement trait DerefMut, so code has to
    explicitly deref, as shown here.
  */
  *h_point.coor_x() = 2.0;
  *h_point.coor_y() = 1.0;
  *h_point.coor_z() = 0.0;
  (*h_point).show("h_point", 2);
  println!();

  show_op("let h_point1 = h_point.clone()");
  let h_point1 = h_point.clone();
  (*h_point1).show("h_point1", 2);
  println!();

}
/*---------------------------------------------------------
  demo user-defined type PointNPrototype
  - create and initialize instances
  - move construction
  - move assignment
  - selected operations
*/
fn demo_user_defined_pointnprototype() {

  show_note(
    "instances of user-defined type PointNPrototype", 
    35
  );
  println!();

  /*-- create instance of PointNPrototype ---*/
  show_op("create PointNPrototype, print using Debug trait");
  let mut q = PointNPrototype::new();
  q.init(&[1.0, 2.0, 3.0]);
  println!("  q: {:?}", q);

  show_op("let q1 = q: move construction");
  let mut q1 = q;
  println!("  q1: {:?}", q1);
  // code below will fail to compile, q was moved
  // println!("  q: {:?}", q);

  q1.coors()[1] = -2.0;
  show_op("q = q1: move assignment");
  q = q1;  // q owns resources given by q1 so valid
  println!("  q: {:?}", q);
  // q1 has been move so no longer valid

  show_op("let q2 = q.clone()");
  let q2 = q.clone();
  println!("  q2: {:?}\n", q2);

  show_note("PointNPrototype objects in heap", 40);
  let mut h_point_prototype = 
    Box::new(PointNPrototype::new());
  h_point_prototype.init(&[3.0, 2.5, 2.0]);
  println!("  h_point_prototype: {:?}", h_point_prototype);

  show_op("  h_point_prototype.coors()[0] = -3.0;");
  h_point_prototype.coors()[0] = -3.0;
  println!("  h_point_prototype: {:?}", h_point_prototype);

  show_op("let h_point_prototype1 = h_point_prototype.clone()");
  let h_point_prototype1 = 
    h_point_prototype.clone();
  println!("  h_point_prototype1: {:?}", h_point_prototype1);

}

/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {
    show_note("demonstrate object creation", 50);

    demo_string_objects();
    demo_vector_objects();
    demo_user_defined_point4d();
    demo_user_defined_pointnprototype();
    
    print!("\n\nThat's all Folks!\n\n");
}
