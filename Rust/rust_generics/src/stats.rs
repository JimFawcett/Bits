/*-------------------------------------------------------------------
  stats.rs
  - defines type Stats containing Vec of generic Type
    bounded by Arithmetic trait 
  - Works as is only for i32 and f64, but easy to extend to
    all integers and floats
  - Can also be extended to complex numbers
-------------------------------------------------------------------*/

use std::ops::*;
use std::cmp::*;
use std::convert::{Into};
use std::fmt::Debug;

pub trait Arithmetic<T = Self>: Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + Default + Copy + Debug + Into<f64> {}

impl Arithmetic<f64> for f64 {}
impl Arithmetic<i32> for i32 {}

#[derive(Debug, Clone)]
pub struct Stats<T: Arithmetic + Debug> {
    items: Vec<T>
}

impl<T: Arithmetic> Stats<T> 
    where T: Arithmetic + Debug
{
    pub fn new(v:Vec<T>) -> Stats<T> {
        Stats {
            items: v,
        }
    }
    pub fn max(&self) -> T {
        let mut biggest = self.items[0];
        for item in &self.items {
            if biggest < *item {
                biggest = *item;
            }
        }
        biggest
    }
    pub fn min(&self) -> T {
        let mut smallest = self.items[0];
        for item in &self.items {
            if smallest > *item {
                smallest = *item;
            }
        }
        smallest
    }
    pub fn sum(&self) -> T {
        let mut sum = T::default();
        for item in &self.items {
            sum = sum + *item;
        }
        sum
    }
    pub fn avg(&self) -> f64 {
        let mut sum = T::default();
        for item in &self.items {
            sum = sum + *item;
        }
        /*-- cast usize to f64 --*/
        let den:f64 = self.items.len() as f64;
        /*-- can't cast non-primitive to primitive --*/
        let num:f64 = sum.into();
        num/den
    }
}