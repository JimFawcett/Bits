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
  show_note("demo std::String", 40);
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
  Demo object instances in action
*/
fn main() {
    show_note("demonstrate object creation and use", 40);

    demo_string_objects();
    demo_vector_objects();
    demo_user_defined_point4d();
    demo_user_defined_pointnprototype();
    
    print!("\nThat's all Folks!\n\n");
}
