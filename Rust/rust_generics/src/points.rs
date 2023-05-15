/*-------------------------------------------------------------------
  points.rs
  - defines types PointN<T>
-------------------------------------------------------------------*/

use std::default::*;
use std::fmt::*;

use crate::analysis;    // identify source code
use analysis::*;        // import public functions and types

/*---------------------------------------------------------
  - Declare PointN<T> struct, like a C++ template class
  - Request compiler implement traits Debug & Clone
*/
#[derive(Debug, Clone)]
pub struct PointN<T>
where
    T: Debug,
    T: Default,
    T: Clone,
{
    coor: Vec<T>,
}
impl<T> PointN<T>
where
    T: Debug,
    T: Default,
    T: Clone,
{
    /*-- constructor --*/
    pub fn new(n: usize) -> PointN<T> {
        PointN::<T> {
            coor: vec![T::default(); n],
        }
    }
    /*-- moves new coordinate vector into coor --*/
    pub fn init(mut self, coord: Vec<T>) -> PointN<T> {
        self.coor = coord;
        self
    }
    pub fn len(&self) -> usize {
        self.coor.len()
    }
    /*-- acts as both get_coor() and set_coor(some_vector) --*/
    pub fn coors(&mut self) -> &mut Vec<T> {
        &mut self.coor
    }
    /*-- displays name, type, and coordinates --*/
    pub fn show(&self, nm:&str, left: usize, width:usize) {
        println!("{nm:?}: PointN<T> {{");
        show_fold(&self.coor, left, width);
        println!("}}")
    }
}
