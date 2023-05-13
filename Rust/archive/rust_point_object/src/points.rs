/*-------------------------------------------------------------------
  points.rs
  - defines types Point1 and Point2<T>
-------------------------------------------------------------------*/

use std::default::*;
use std::fmt::*;

#[derive(Debug, Clone)]
pub struct Point<T> {
    items: Vec<T>,
}

impl<T> Point<T> 
    where T: Debug + Clone + Default
{
    pub fn new(n:usize) -> Point<T> {
        Point::<T> {
            items:vec![T::default(); n]
        }
    }
}

impl<T> IntoIterator for Point<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// impl<T> Iterator for Point<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.items.pop()
//     }
// }