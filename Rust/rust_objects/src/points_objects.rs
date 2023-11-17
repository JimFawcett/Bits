/*-------------------------------------------------------------------
  points.rs
  - defines type Point4D representing points in space-time
-------------------------------------------------------------------*/

use std::fmt::*;
use chrono::prelude::*;

/*---------------------------------------------------------
  - Declare Point4D struct, like a C++ class
  - Request compiler implement traits Debug & Clone
*/
#[derive(Debug, Clone)]
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
      A different strategy is used in the Generic Bit to
      access an arbitrary number of coordinate values.
    */
    /*
      For simple types like this it would be reasonable to
      make x, y, z public and remove the getter and setter
      functions.
    */
    pub fn show(&self, nm:&str) {
        print!("{nm}: Point4D {{\n  ");
        print!("{0}, ", self.x);
        print!("{0}, ", self.y);
        println!("{0},", self.z);
        println!("  {}", Local::now().format("%a %b %e %Y, %T"));
        println!("}}");
    }
}
