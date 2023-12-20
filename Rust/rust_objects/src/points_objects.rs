/*-------------------------------------------------------------------
  points.rs
  - defines type Point4D representing points in space-time
-------------------------------------------------------------------*/

use std::fmt::*;
use chrono::prelude::*;

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
    self
  }
  /*-- make coordinates accessible for reading and writing --*/
  pub fn coors(&mut self) -> &mut Vec<f64> {
    &mut self.coords
  }
}
