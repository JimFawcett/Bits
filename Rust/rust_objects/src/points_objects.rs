/*-------------------------------------------------------------------
  points.rs
  - defines type Point4D representing points in space-time
  - defines type PointNPrototype representing system states
-------------------------------------------------------------------*/

use std::fmt::*;
use chrono::prelude::*;
use crate::analysis_objects::*;

/*---------------------------------------------------------
  Declare Point4D struct
  - similar in function to a C++ class
  - Request compiler to implement traits: Debug, Copy & Clone
  - Can be Copy since all members are copy
  - So, construction and assignment are copy operations
  - Demos two types of member access, only one needed
*/
#[derive(Debug, Copy, Clone)] 
pub struct Point4D {
    x: f64,
    y: f64,
    z: f64,
    t: DateTime<Local>,
}
/*-- implement constructor function new --*/
impl Point4D {
    pub fn new() -> Point4D {
        Point4D { x: 0.0, y: 0.0, z: 0.0, t: Local::now() }
    }
    /*------------------------------------------------
      These methods can be used to set or retrieve 
      values of Point4D instance coordinates, as they 
      return a mutable reference. Note similarity with
      C++ code.
    */
    pub fn coor_x(&mut self) -> &mut f64 {
      &mut self.x
    }
    pub fn coor_y(&mut self) -> &mut f64 {
      &mut self.y
    }
    pub fn coor_z(&mut self) -> &mut f64 {
      &mut self.z
    }
    pub fn time(&self) -> String {
      self.t.format("%a %b %e %Y, %T").to_string()
    }
    pub fn update_time(&mut self) {
      self.t = Utc::now().with_timezone(&Local);
    }
    /*------------------------------------------------
      An alternate design for mutating coordinates.
      This set has twice as many methods, but may 
      make access and mutation more obvious when
      reading source code.
    */
      pub fn get_x(&self) -> &f64 {
          &self.x
      }
      pub fn get_y(&self) -> &f64 {
          &self.y
      }
      pub fn get_z(&self) -> &f64 {
          &self.z
      }
      pub fn set_x(&mut self, x: f64) {
          self.x = x
      }
      pub fn set_y(&mut self, y: f64) {
          self.y = y
      }
      pub fn set_z(&mut self, z: f64) {
          self.z = z
      }
    /*
      For simple types like this it would be reasonable to
      make x, y, z public and remove the getter and setter
      functions.
    */
    /*-----------------------------------------------------
      show method displays instance values
      - uses helper function indent  
    */
    pub fn indent(ch:char, n:usize) -> String {
      (0..n).map(|_i| ch).collect::<String>()
      // equivalent to:
      //   let ind = (0..n).map(|_i| ch).collect::<String>();
      //   ind
      // - (0..n) is an iterator yielding values 0 through n-1
      // - map sets each iterator item to same char ch
      // - collect appends each char to temporary String
      // ind defines return value, the collected String
    }
    pub fn show(&self, nm:&str, ind:usize) {
        let indstr = Point4D::indent(' ', ind);
        print!("{indstr}{nm} : Point4D {{\n  ");
        print!("{indstr}{0}, ", self.x);
        print!("{indstr}{0}, ", self.y);
        println!("{indstr}{0},", self.z);
        println!("{indstr}  {}", Local::now().format("%a %b %e %Y, %T"));
        println!("{indstr}}}");
    }
}
/*---------------------------------------------------------
  demo user-defined type Point4D
  - create and initialize instances
  - copy construction
  - copy assignment
  - selected operations
*/
pub fn demo_user_defined_point4d() {

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
  println!();

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
/*
  PointNPrototype
  - Request compiler implement traits Debug & Clone
  - Can't be Copy since members are not copy
  - So, construction and assignment are move operations
  - This demo will be expanded into useful type in
    next Bit: Rust_GenericBit
*/
#[derive(Debug, Clone)]   // can't make Copy type
pub struct PointNPrototype {
  coords: Vec<f64>        // not copy type
}
impl PointNPrototype {
  pub fn new() -> PointNPrototype {
    PointNPrototype {
      coords: vec![]
    }
  }
  /*-- initialize from elements of array slice --*/
  pub fn init(&mut self, arr: &[f64]) -> &mut PointNPrototype {
    self.coords = (0..arr.len()).map(|i| arr[i]).collect::<Vec<f64>>();
    // arr is an array slice, e.g., a view into an array
    // (0..arr.len()) is an iterator returning items from the slice
    // map(|i| arr[i]) returns arr[i] for each iterator item i
    // collect::&lt;Vec<f64&gt;&gt;() collects Vec of those arr values.
    self // returns mutable reference to modified PointNPrototype instance
  }
  /*-- make coordinates accessible for reading and writing --*/
  pub fn coors(&mut self) -> &mut Vec<f64> {
    &mut self.coords
  }
}
/*---------------------------------------------------------
  demo user-defined type PointNPrototype
  - create and initialize instances
  - move construction
  - move assignment
  - selected operations
*/
pub fn demo_user_defined_pointnprototype() {

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
