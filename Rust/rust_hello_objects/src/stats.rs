/*-------------------------------------------------------------------
  stats.rs
  - defines type Stats containing Vec of generic Type
    bounded by Arithmetic trait 
-------------------------------------------------------------------*/

//use std::ops::{Add, Sub, Mul, Div, Neg, AddAsign, };
use std::ops::*;
use std::cmp::*;
// use num_traits::cast::AsPrimitive;
use std::fmt::Debug;
use std::convert::TryInto;
// enum Number {
//     Integral(i32),
//     Float(f64),
// }
// impl AsPrimitive<f64> for f64 {}
// impl AsPrimitive<i32> for i32 {}


pub trait Arithmetic<T = Self>: Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + Default + Copy + Debug + std::convert::Into<f64> + std::convert::TryInto<f64>
    {}
    // + AsPrimitive<T>: 'static + Copy {}

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
        /*- will panic if value of len() is too large -*/
        let den = self.items.len() as f64;
        // let test = self.items.len().try_into();
        // let den:f64 = test.unwrap();
        let num = sum.into();
        num/den
    }
}