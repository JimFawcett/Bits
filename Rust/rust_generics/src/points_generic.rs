/*-------------------------------------------------------------------
  points.rs
  - defines types PointN<T>
-------------------------------------------------------------------*/

use std::default::*;
use std::fmt::*;

use crate::analysis_generic;    // identify source code
use analysis_generic::*;        // import public functions and types

/*---------------------------------------------------------
  - Declare PointN<T> struct, like a C++ template class
  - Request compiler implement traits Debug & Clone
*/
#[derive(Debug, Clone)]
pub struct Point<T, const N: usize>
where
  T: Debug + Default + Clone
{
  coor: Vec<T>,
}
impl<T, const N: usize> Point<T, N>
where
T: Debug + Default + Clone
{
  /*-- constructor --*/
  pub fn new() -> Point<T, N> {
    Point::<T, N> {
      coor: vec![T::default(); N],
    }
  }
  /*------------------------------------------------------- 
    Point<T, N>::init(&self, v:Vec<T>) fills coor with 
    first N values from v and sets any remainder to
    T::default(). 
  -------------------------------------------------------*/
  pub fn init(mut self, coord: &Vec<T>) -> Point<T, N> {
    for i in 0..N {
      if i < coord.len() {
        self.coor[i] = coord[i].clone();
      }
      else {
        self.coor[i] = T::default();
      }
    }
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
    println!("  {nm:?}: Point<T, N> {{");
    show_fold(&self.coor, left + 2, width);
    println!("  }}")
  }
}
/*-- implements const indexer -----------------*/
impl<T:Debug, const N:usize, Idx> std::ops::Index<Idx> for Point<T, N> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    type Output = Idx::Output;

    fn index(&self, index:Idx) -> &Self::Output {
        &self.coor[index]
    }
}
/*-- implements mutable indexer ---------------*/
impl<T, const N: usize, Idx> std::ops::IndexMut<Idx> for Point<T, N> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    fn index_mut(&mut self, index:Idx) -> &mut Self::Output {
        &mut self.coor[index]
    }
}
/* explicit conversion to slice &[T] */
impl<T, const N: usize> AsRef<[T]> for Point<T, N>
  where 
    T: ?Sized,
    T: Debug + Display + Default + Clone,
{
  fn as_ref(&self) -> &[T] {
    &self.coor
  }
}
/* implicit conversion to slice &[T] */
impl<T, const N: usize> std::ops::Deref for Point<T, N> 
  where T: Debug + Default + Clone,
{
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    &self.coor
  }
}
/*---------------------------------------------------------
  Demonstrate user-defined PointN<T> type
*/
pub fn demo_pointn() {

  show_label("demo indexing with PointN<132, 5>", 40);
  println!();

  show_op("PointN<i32, 5>");
  println!();
  let mut p = Point::<i32, 5>::new()
         .init(&vec![1, 2, 3, 2, 1]);
  p.show("p", 2, 12);
  println!();
  show_op("*p.coors() = vec![1, 0, -1, 0, 1]");
  *p.coors() = vec![1, 0, -1, 0, 1];
  p.show("p", 2, 12);

  println!("\n  using immutable indexer:");
  println!("  value of p[0] is {}\n", p[0]);
  println!("  using mutable indexer:");
  show_op("p[0] = 3");
  p[0] = 3;
  p.show("p", 2, 12);
  show_op("p[1] = 4");
  p[1] = 4;
  p.show("p", 2, 12);

}
