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
pub struct PointN<T> 
    where T:Debug + Default + Clone
{
    items: Vec<T>
}
impl<T> PointN<T> 
    where T:Debug + Default + Clone
{
    pub fn new(n:usize) -> PointN<T> {
        PointN::<T> { 
            items: vec![T::default(); n],
        }
    }
    pub fn is_empty(&self) -> bool {
      self.items.is_empty()
    }
    pub fn len(&self) -> usize {
      self.items.len()
    }
    pub fn push(&mut self, item:T) {
      self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
      self.items.pop()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }
}
/*-- implements const indexer -----------------*/
impl<T:Debug, Idx> std::ops::Index<Idx> for PointN<T> 
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
impl<T, Idx> std::ops::IndexMut<Idx> for PointN<T> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    fn index_mut(&mut self, index:Idx) -> &mut Self::Output {
        &mut self.items[index]
    }
}
/*-- IntoIterator trait for &PointN<T> ---------*/
impl<'a, T> IntoIterator for &'a PointN<T>
    where T:Debug + Default + Clone
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let ccln = self.items.clone();
        ccln.into_iter()
    }
}
/*-- IntoIterator trait for PointN<T> ----------*/
impl<T> IntoIterator for PointN<T>
    where T:Debug + Default + Clone
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
