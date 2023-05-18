/*-------------------------------------------------------------------
  points.rs
  - defines type Point4D representing points in space-time
-------------------------------------------------------------------*/

// use std::default::*;
use std::fmt::*;
use chrono::{DateTime, Local};

// use crate::analysis;    // identify source code
// use analysis::*;        // import public functions and types

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
/*-- implement function new --*/
impl Point4D {
    pub fn new() -> Point4D {
        Point4D { x: 0.0, y: 0.0, z: 0.0, t: Local::now() }
    }
    /*
      For simple types like this it would be reasonable to
      make x, y, z public and remove the getter and setter
      functions.
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
    pub fn show(&self, nm:&str) {
        print!("{nm}: Point4D {{\n  ");
        print!("{0}, ", self.x);
        print!("{0}, ", self.y);
        println!("{0},", self.z);
        println!("  {}", Local::now().format("%a %b %e %Y, %T"));
        println!("}}");
    }
}
