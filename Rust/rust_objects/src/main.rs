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

fn demo_standard_objects() {

  show_note(
    "std::library objects: string and vector<T>", 46
  );
  nl();

  /*-- create instance of String ----*/
  show_op("let mut s1 = \"a string\".to_string()");
  let mut s1 = "a string".to_string();
  println!("  s1:String : {:?}", s1);
  nl();
  /*-- show type of s1 --------------*/
  show_op("show_type_scalar(&s1, \"&s1\")");
  show_type_scalar(&s1, "&s1");
  nl();
  /*-- one of many operations -------*/
  show_op("s1.push_str(\" and more\");");
  s1.push_str(" and more");
  println!("  s1:String : {:?}", s1);
  nl();
  /*-- move construction --*/
  show_op("let s2 = s1 : move construction");
  let s2 = s1;
  println!("  s2:String : {:?}", s2);
  println!("  s1 now invalid\n");
  // println!("  s1:String : {:?}\n", s1);
  // uncomment line above to see error message

  /*-- move assignment --*/
  show_op("s1 = s2 : move assignment");
  let s1 = s2;
  println!("  s1:String : {:?}", s1);
  println!("  s2 now invalid\n");
  // println!("  s2:String : {:?}\n", s2);
  // uncomment line above to see error message

  //https://jimfawcett.github.io/Resources/RustBites/RustBites_Strings.html
  
  /*-- create instance of Vec<i32> --*/
  show_op("let mut v:Vec<i32> = vec![1, 2, 3, 2, 1];");
  let mut v: Vec<i32> = vec![1, 2, 3, 2, 1];
  println!("  v:Vec<i32> : {:?}", v);
  nl();
  /*-- show type of v ---------------*/
  show_op("show_type_scalar(&v, \"&v\")");
  show_type_scalar(&v, "&v");
  nl();
  /*-- show one of many operations --*/
  show_op("v.push(42);");
  v.push(42);
  println!("  v:Vec<i32> : {:?}", v);
  //https://jimfawcett.github.io/Resources/RustBites/RustBites_DataStr.html#vec
}

fn demo_user_defined_objects() {

  show_note("instances of user-defined type", 35);
  nl();

  /*-- create instance of Point4D ---*/
  show_op("let mut p1 = Point4D::new()");
  let mut p1 = Point4D::new();
  p1.show("p1");
  p1.set_x(42.0);
  p1.set_y(-3.0);
  p1.set_z(2.0);
  p1.show("p1");
  /*-- show p1's type ---------------*/
  show_type_scalar(&p1, "p1");
  nl();

  /*-- show one of its operations ---*/
  show_op("let p1a = p1.clone()");
  let p1a = p1.clone();
  p1a.show("p1.clone()");   

  println!("\nusing PointN<f64>::coor_x function:");
  let rx = p1.coor_x(); 
  *rx = 84.0;
  p1.show("p1");

  println!("\nusing PointN<f64>::coor_y function:");
  let ry = p1.coor_y(); 
  *ry = 84.0;
  p1.show("p1");

  println!("\nusing PointN<f64>::coor_z function:");
  let rz = p1.coor_z(); 
  *rz = 84.0;
  p1.show("p1");

  println!("\nupdating time value:");
  /*-- delay 5 secs for update_time demo --*/
  println!("delaying 5 seconds before time update");
  use std::time::Duration;
  let dur:Duration = Duration::from_secs(5);
  std::thread::sleep(dur);

  p1.update_time();
  p1.show("p1");

  show_op("let p2 = p1 : move construction");
  let mut p2 = p1;
  p2.show("p2");

  show_op("*p2.coor_x() = 42.0");
  *p2.coor_x() = 42.0;
  p2.show("p2");

  show_op("p1 = p2 : move assignment");
  p1 = p2;
  p1.show("p1");
}

fn demo_heap_operations() {

  show_note("std library objects in heap", 40);
  let mut h_str = Box::new(vec![1, 2, 3, 2, 1]);
  show_type(&h_str, "h_str");
  println!("h_str: {:?}", h_str);

  show_op("h_str[1] = -1");
  /*-------------------------------------------------------
    This works because Box implements Deref and DerefMut
    traits which automatically dereference smart Box pointers.  
    That essentially delegates the inner interface back 
    to the Box pointer.
  */
  h_str[1] = -1;
  /*
    equivalent to (*h_str)[1] = -1;
    because of DerefMut
  */
  println!("h_str: {:?}", h_str);

  show_note("user-defined objects in heap", 40);
  let mut h_point = Box::new(Point4D::new());
  show_type(&h_point, "h_point");
  /*
    Point4D does not implement trait DerefMut, so code has to
    explicitly deref, as shown here.
  */
  *h_point.coor_x() = 2.0;
  *h_point.coor_y() = 1.0;
  *h_point.coor_z() = 0.0;
  (*h_point).show("h_point");
}

fn demo_formats() {

  show_note("Testing formats for collections", 35);
  nl();

  let v = 
    vec![
      0i32, -1, 2, -3, 4, -5, 6, -7, 8, -9, 
      10, -11, 12, -13, 14
    ];

  show_op("show_type_scalar(&v, \"&v\")");
  show_type_scalar(&v, "&v");
  nl();

  show_op("show_fold(&v, 2, 5)");
  show_fold(&v, 2, 5);
  nl();

  show_op("let v = vec![0i32, ... ];");
  show_op("fold(&v, 2, 12)");
  let stmp = fold(&v, 2, 12);
  println!("{stmp}");
  nl();

  show_op("let a = &[f64; 8];");
  let a = 
    &[1.0, 2.5, -3.1, 4.3, 5.0, 6.0, 7.0, -8.0];
  show_op("show_fold(a, 4, 5)");
  show_fold(a, 4, 5);

}
/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {
    show_note("demonstrate object creation", 50);

    demo_standard_objects();
    demo_user_defined_objects();
    demo_heap_operations();
    
    const TEST:bool = false;
    if TEST {
      demo_formats();
    }
    print!("\n\nThat's all Folks!\n\n");
}
