/*-- PointN<T> -----------------------------------
  PointN<T> declares a PointN type holding a
  Vec<T> of coordinate values.
  It implements:
  - new(n)  constructor
  - iter()  returns iterator over items
  - trait IntoIterator for PointN<T>
  - trait IntoIterator for &PointN<T>
  - immutable and mutable indexing
  Note:
  ---------------------------------------------
  This is a nice example of building a custom
  collection type. It implements methods and
  traits necessary to make a collection behave
  like standard library collections.
  ---------------------------------------------
*/
use std::fmt::*;

#[derive(Debug, Clone)]
pub struct Point<T, const N: usize> 
    where T:Debug + Default + Clone
{
    pub items: Vec<T>
}
impl<T, const N:usize> Point<T, N> 
    where T:Debug + Default + Clone
{
    /*-- constructor --*/
    pub fn new() -> Point<T, N> {
        Point::<T, N> { 
            items: vec![T::default(); N],
        }
    }
    pub fn init(&mut self, v:&Vec<T>) {
      for i in 0..v.len() {
        self.items[i] = v[i].clone();
      }
      for i in v.len()..N {
        self.items[i] = T::default();
      }
    }
    /*-- non-destructive non-mutating iterator */
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }
}
/*-- implements const indexer -----------------*/
impl<T, const N:usize, Idx> std::ops::Index<Idx> for Point<T, N> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    type Output = Idx::Output;

    fn index(&self, index:Idx) -> &Self::Output {
        &self.items[index]
    }
}
/*-- implements mutable indexer ---------------*/
impl<T, const N:usize, Idx> std::ops::IndexMut<Idx> for Point<T, N> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    fn index_mut(&mut self, index:Idx) -> &mut Self::Output {
        &mut self.items[index]
    }
}
/*-- IntoIterator trait for PointN<T> ----------*/
impl<T, const N:usize> IntoIterator for Point<T, N>
    where T:Debug + Default + Clone
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
/*-- IntoIterator trait for &PointN<T> ---------*/
impl<T, const N:usize> IntoIterator for &Point<T, N>
    where T:Debug + Default + Clone
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.clone().into_iter()
    }
}
// The definition below is ambiguous
//
// /*-- Iterator trait for PointN<T> --------------*/
// impl<T> Iterator for PointN<T>
//     where T:Debug + Default + Clone
// {
//     type Item = T;
//     fn next(self) -> Option<Self::Item> {
//         self.items.iter().next();
//     }
// }
