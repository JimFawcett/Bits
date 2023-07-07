/*-----------------------------------------------
  Bits::rust_iter::main.rs
  - demonstrates iteration over collections with
    Rust iterators
  - Most collections implement the Rust trait
    IntoIterator which consumes the collection
    to generate an iterator.
  - Many also supply functions iter() and mut_iter()
    which return iterators without consuming originial
    collection.
  - Demonstrates iteration over arrays, slices,
    Vecs, VecDeques, and custom PointN<T> type. 
-----------------------------------------------*/
/*-----------------------------------------------
Iteration traits:
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>; // returns value
  // plus many provided adapter methods
}
//https://doc.rust-lang.org/std/iter/trait.Iterator.html#

pub trait IntoIterator {
  type Item;
  type IntoIter: Iterator<Item = Self::Item>;
  fn into_iter(self) -> Self::IntoIter;  // returns iterator
}
//https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

Standard methods for creating iterators:
- iter()      : iterates over &T
- iter_mut()  : iterates over &mut T
- into_iter() : iterates over T, consumes collection
//https://doc.rust-lang.org/std/iter/

Closure traits (used with modifying iterators):
pub trait FnOnce<Args>  //consumes args
  where Args: Tuple,
{
  type Output;
  extern "rust-call" fn call_once(self, args: Args) -> Self::Output
}
pub trait FnMut<Args> : FnOnce<Args>
  where Args: Tuple,
{
  extern "rust-call" fn call_mut(
    &mut self, args: Args
  ) -> Self::Output;
}
pub trait Fn<args> : FnMut<Args>
  where Args: Tuple,
{
  extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
//https://doc.rust-lang.org/stable/std/ops/trait.Fn.html#

-----------------------------------------------*/
#![allow(dead_code)]
#![allow(unused_variables)]

/*-----------------------------------------------
  - Don't need use for analysis_iter as it only 
    contains functions.
  - Do need for points_iter as it defines a type
    to be instantiated.
*/
use std::collections::*;
mod analysis_iter;
// use analysis_iter::*;
mod points_iter;
use points_iter::*;

/*---------------------------------------------------------
  Use of Rust iterators is encapsulated in a sequence of
  functions defined below and used in main.  This file 
  builds collections and applies the various functions
  to illustrate how iterators are used.

  It starts with a few quick indexing and iteration 
  examples before applying the analysis functions.
---------------------------------------------------------*/

use std::fmt::*;
use std::cmp::*;

/*---------------------------------------------------------*/
fn demo_array_into_iterator_loop<T:Debug>(arr:[T; 5]) {
  let mut iter = arr.into_iter();
  loop {
    let item = iter.next();
    match item {
      Some(item) => print!("{:?} ", item),
      None => break
    }
  }
}
fn execute_demo_array_into_iterator_loop() {
  print!("execute demo_array_into_iterator_loop");
  let arr = [1, 2, 3, 2, 1];
  print!("\n  ");
  demo_array_into_iterator_loop(arr);
  print!("\n  {:?}", arr);  // this compiles because arr is copy
  println!();
}
/*---------------------------------------------------------*/
fn demo_array_into_iterator_for<T:Debug>(arr:[T; 5]) {
  for item in arr {
    print!("{:?} ", item);
  }
}
fn execute_demo_array_into_iterator_for() {
  print!("execute demo_array_into_iterator_for");
  let arr = [1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  ");
  demo_array_into_iterator_for(arr);
  print!("\n  {:?}", arr);  // this compiles because arr is copy
  println!();
}
/*---------------------------------------------------------*/
fn demo_vec_into_iterator_for<T:Debug>(vec:Vec<T>) {
  for item in vec {
    print!("{:?} ", item);
  }
}
fn execute_demo_vec_into_iterator_for() {
  print!("execute demo_vec_into_iterator_for");
  let v = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  ");
  demo_vec_into_iterator_for(v);
  //print!("\n  {:?}", v);  // won't compile because v moved
  println!();
}
/*---------------------------------------------------------*/
fn demo_array_iter_for<T:Debug>(arr:[T; 5]) {
  for item in arr.iter() {
    print!("{:?} ", item);
  }
}
fn execute_demo_array_iter_for() {
  print!("execute demo_array_iter_for");
  let arr = [1.0f64, 2.0, 3.0, 4.0, 5.0];
  let iter = arr.iter();
  print!("\n  ");
  demo_array_iter_for::<f64>(arr);
  print!("\n  {:?}", arr); 
  println!();
}
/*---------------------------------------------------------*/
fn demo_vec_iter_for<T:Debug>(v:&Vec<T>) {
  for item in v.iter() {
    print!("{:?} ", item);
  }
}
fn execute_demo_vec_iter_for() {
  print!("execute demo_vec_iter_for");
  let v = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  ");
  demo_vec_iter_for(&v);
  print!("\n  {:?}", v);  // compiles because iter() doesn't move
  println!();
}
/*---------------------------------------------------------*/
fn demo_vec_iter_mut_for<T:Debug, F>(v:&mut Vec<T>, mut f:F) 
  where F: FnMut(&mut T) -> ()
{
  for item in v.iter_mut() {
    f(item);
  }
}
fn execute_demo_vec_iter_mut_for() {
  print!("execute demo_vec_iter_mut_for");
  let mut v = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
  let plus_one = |item: &mut f64| *item += 1.0;
  print!("  ");
  demo_vec_iter_mut_for(&mut v, plus_one);
  print!("\n  {:?}", v);  // compiles because iter() doesn't move
  println!();
}
/*---------------------------------------------------------*/
fn demo_collection_iter_mut_for<T, F>(c:&mut dyn Iterator<Item = T>, mut f:F) 
  where 
    F: FnMut(&mut T) -> ()
{
  for mut item in c {
    f(&mut item);
  }
}
fn execute_demo_collection_iter_mut_for() {
  
  print!("execute demo_collection_iter_mut_for with array");
  let plus_one = |item: &mut &mut f64| { **item += 1.0; };
  let mut a = [1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  input:  {:?}", a);
  demo_collection_iter_mut_for(&mut a.iter_mut(), plus_one);
  print!("\n  output: {:?}", a);
  println!();

  print!("execute demo_collection_iter_mut_for with Vec");
  let mut v = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  input:  {:?}", v);
  demo_collection_iter_mut_for(&mut v.iter_mut(), plus_one);
  print!("\n  output: {:?}", v);
  println!();
  
  print!("execute demo_collection_iter_mut_for with Map");
  let mut m = HashMap::<String, i32>::new();
  m.insert("zero".to_owned(), 0);
  m.insert("one".to_owned(), 1);
  m.insert("two".to_owned(), 2);
  m.insert("three".to_owned(), 3);
  let plus_one = |item: &mut (&String, &mut i32)| { *item.1 += 1; };
  print!("\n  input:\n    {:?}", m);
  demo_collection_iter_mut_for(&mut m.iter_mut(), plus_one);
  print!("\n  output:\n    {:?}", m);
  println!();
}
/*---------------------------------------------------------*/
/*-----------------------------------------------
  demo_vec_indexer<T: Debug>(v:&Vec<T>)
-------------------------------------------------
Simplest case - displays Vector with generic
Item type.
- uses naive indexing (no iterator here)
- works only for Vec's
*/
fn demo_vec_indexer<T: Debug>(v:&Vec<T>) {
  let mut i = 0;
  print!("  ");
  while i < v.len() {
    print!("{:?} ", v[i]);
    i += 1;
  }
  println!();
}
fn execute_demo_vec_indexer() {
  println!("execute demo_vec_indexer");
  let v = vec![1, 2, 3, 2, 1];
  demo_vec_indexer(&v);
}
/*-----------------------------------------------
  Demo for loop that consumes vector
  - implicit use of into_iterator
  - consumes v
*/
fn demo_for<T:Debug>(v:Vec<T>) {
  print!("  ");
  for item in v {  // for is using into_iterator
    print!("{:?} ", item);
  }
  // next line won't compile - v was moved, e.g., consumed
  // println!("{:?}", v);
  println!();
}
fn execute_demo_for() {
  println!("execute demo_for");
  let v = vec![1, 2, 3, 2, 1];
  demo_for(v);
}
/*-----------------------------------------------
  Demo for loop that does not consume vector
  - uses vector method iter() which returns
    iterator
*/
fn demo_for_ref<T:Debug>(rv:&Vec<T>) {
  print!("  ");
  for item in rv.iter() {
    print!("{:?} ", item);
  }
  // next line compiles - rv was not moved
  println!("{:?}", rv);
}
fn execute_demo_for_ref() {
  println!("execute demo_for_ref");
  let v = vec![1, 2, 3, 2, 1];
  demo_for_ref(&v);
}
/*-----------------------------------------------
  demo_slice_indexer<T:Debug>(s:&[T])
-------------------------------------------------
Illustrates indexing in slices
- Not idiomatic, but safe and correct.
- Works with any collection with contiguous
  fixed size elements.
- Won't work with String or str inputs; they
  are collections of utf8 chars which vary
  in size from 1 to 4 bytes.
- It is rare for idomatic Rust code to use
  indexing. 
- &[T] is slice type
- Demo's in main show how to use for various
  types.
*/
#[allow(clippy::needless_range_loop)]
fn demo_slice_indexer<T:Debug>(s:&[T]) {
  print!("\n  ");
  let max = s.len();
  /* 0..max is range iterator */
  for i in 0..max {
      print!("{:?} ", s[i]);
  }
  println!();
  /*---------------------------
    clippy prefers no indexing:
    for item in s.iter().take(max) {
      print!("{item} ");
    }
  */
}
fn execute_demo_slice_indexer() {
  print!("execute demo_slice_indexer");
  let sl = [1, 2, 3, 4, 3, 2, 1];
  demo_slice_indexer(&sl);
}
/*-----------------------------------------------
  sub_range_indexer<T:Debug>(
    s:&[T], mut lower:usize, mut upper:usize
  )
Iterates over a sub-range of the slice s
- works with any collection with contiguous
  fixed size elements.
*/
#[allow(clippy::needless_range_loop)]
fn demo_sub_range_indexer<T:Debug>(
  s:&[T], mut lower:usize, mut upper:usize
) {
  print!("\n  ");
  lower = max(0, lower);
  upper = min(s.len(), upper);
  if lower <= upper {
    for i in lower..upper {
      print!("{:?} ", s[i]);
    }
  }
  println!();
}
fn execute_demo_sub_range_indexer() {
  print!("execute demo_sub_range_indexer");
  let s = [1, 2, 3, 4, 3, 2, 1];
  demo_sub_range_indexer(&s, 2, 4);
}
/*-----------------------------------------------
  slice_looper<T:Debug>(s:&[T])
-------------------------------------------------
Iterates over slice s without indexing
- Works with any collection with contiguous
  fixed size elements,
  e.g., array, Vector, PointN, ...
- Uses slice iterator with matching.
*/
fn demo_slice_looper<T:Debug>(s:&[T]) {
  print!("\n  ");
  let mut iter = s.iter();
  loop {
      let item = iter.next();
      match item {
          Some(val) => print!("{val:?} "),
          None => break
      }
  }
  println!();
}
fn execute_demo_slice_looper() {
  print!("execute demo_slice_looper");
  let s = [1, 2, 3, 4, 3, 2, 1];
  demo_slice_looper(&s);
}
/*-----------------------------------------------
  collection_looper<C:Debug, I:Debug>(c:&C)
-------------------------------------------------
- prints comma separated list of Collection<I>'s 
  items.
- where clause requires C to implement 
  IntoIterator trait. 
- C is type of collection, I is type of C's items
- Accepts any collection type that implements
  IntoIterator trait, e.g., array, slice, Vector, ...
- collection_looper can't accept String or &str
  because String does not implement IntoIterator
- That's because String provides two iterators,
  chars() to iterate multibyte chars and bytes()
  to iterate over bytes.
- Not very efficient - uses three order N operations,
  clone, collect, and loop.
  https://stackoverflow.com/questions/49962611/why-does-str-not-implement-intoiterator
*/
fn demo_collection_looper<C: Debug, I: Debug>(c:&C) 
where C: IntoIterator<Item = I> + Clone
{
  print!("\n  ");
  let cc = c.clone();
  let iter = cc.into_iter();
  /* convert c into Vec to get len() method */
  let v:Vec<_> = iter.collect();
  let mut iter = v.iter();  // shadowing
  let mut count = 0;
  loop {
      let item = iter.next();
      match item {
          Some(val) => print!("{val:?}"),
          None => { println!(); break; }
      }
      if count < v.len() - 1 {
          print!(", ");
      }
      count += 1;
  }
}
fn execute_demo_collection_looper() {
  print!("execute demo_collection_looper with slice");
  let s = [1, 2, 3, 4, 3, 2, 1];
  demo_collection_looper(&s);
  print!("execute demo_collection_looper with vector");
  let v = vec![1, 2, 3, 4, 3, 2, 1];
  demo_collection_looper(&v);
  print!("execute demo_collection_looper with PointN");
  let mut p = PointN::<i32>::new(0usize);
  p.push(1);
  p.push(2);
  p.push(3);
  p.push(4);
  p.push(3);
  p.push(2);
  p.push(1);
  demo_collection_looper(&p);
}
/*----------------------------------------------- 
  for_looper<C: Debug, I: Debug>(c:&C) 
-------------------------------------------------
- prints comma separated list of Collection<I>'s 
  items.
- similar to demo_collection_looper but erases last comma
  so no need for collect() or clone()
- uses idiomatic forloop with no indexing
*/
fn demo_for_looper<C: Debug, I: Debug>(c:&C) 
  where C: IntoIterator<Item = I> + Clone
{
  print!("\n  ");
  /* build string of comma separated values */
  let mut accum = String::new();
  let cc = c.clone();
  for item in cc {  // converts cc into iterator
      accum += &format!("{item:?}, ");
  }
  /* remove last comma */
  let opt = find_last_utf8(&accum, ',');
  if let Some(index) = opt {
      accum.truncate(index);
  }
  println!("{accum}");
}
/*-- find last char in str --*/
pub fn find_last_utf8(s:&str, chr: char) -> Option<usize> {
  s.chars().rev().position(|c| c == chr)
   .map(|rev_pos| s.chars().count() - rev_pos -1)
}
fn execute_demo_for_looper() {
  print!("execute demo_for_looper with slice");
  let s = [1, 2, 3, 4, 3, 2, 1];
  demo_for_looper(&s);
  print!("execute demo_for_looper with vector");
  let v = vec![1, 2, 3, 4, 3, 2, 1];
  demo_for_looper(&v);
  print!("execute demo_for_looper with PointN");
  let mut p = PointN::<i32>::new(0usize);
  p.push(1);
  p.push(2);
  p.push(3);
  p.push(4);
  p.push(3);
  p.push(2);
  p.push(1);
  demo_for_looper(&p);
}
/*-----------------------------------------------
  ranger<T>(iter: &mut T)
-------------------------------------------------
- Displays contents of iterator, often passed in
  as range.
- another idiomatic iteration.
*/
fn demo_ranger<T>(iter: &mut T)
  where T: Iterator, T::Item: Debug
{
  print!("\n  ");
  for item in iter {
      print!("{item:?} ")
  }
  println!();
}
fn execute_demo_ranger() {
  print!("execute demo_ranger with string iter");
  let st = "a string";
  demo_ranger(&mut st.chars());
  print!("execute demo_ranger with slice iter");
  let s = [1, 2, 3, 4, 3, 2, 1];
  demo_ranger(&mut s.iter());
  print!("execute demo_ranger with vector iter");
  let v = vec![1, 2, 3, 4, 3, 2, 1];
  demo_ranger(&mut v.iter());
  print!("execute demo_ranger with VecDeq iter");
  let mut vd = VecDeque::<i32>::new();
  vd.push_back(1);
  vd.push_back(2);
  vd.push_back(3);
  vd.push_back(4);
  vd.push_back(3);
  vd.push_back(2);
  vd.push_back(1);
  demo_ranger(&mut vd.iter());
  print!("execute demo_ranger with PointN iter");
  let mut p = PointN::<i32>::new(0usize);
  p.push(1);
  p.push(2);
  p.push(3);
  p.push(4);
  p.push(3);
  p.push(2);
  p.push(1);
  demo_ranger(&mut p.iter());
}
/*-----------------------------------------------
  demo_adapters<C, I>(c: C, i: I) -> Vec<I>
-------------------------------------------------
- iterates over collection C,
  removes non-positive items, adds second 
  argument i and collects into vector.
-------------------------------------------------
- adapters accept an iterator and return a
  revised iterator, as discussed below.
- adapter filter builds iterator over elements
  that satisfy a predicate defined by closure
- map builds iterator that modifies elements
  according to a closure.
- Adapter collect runs iterator and collects 
  into Vec<I>.
*/
fn demo_adapters<C, I>(c: C, i: I) -> Vec<I>
where
  C: IntoIterator<Item = I> + Debug + Clone,
  I: std::ops::Add<Output = I> + std::ops::Mul<Output = I>
      + PartialOrd + PartialEq + Debug + Default + Copy,
{
  let def = I::default();  // expect value is zero
  c.into_iter()
      .filter(|item| item > &def)
      .map(|item| item + i)
      .collect()
}
fn execute_demo_adapters() {
  println!("execute demo_adapters<T, i32>(coll, 2) with array:");
  let a = [1, -1, 0, 2, 3, 4];
  println!("{:?} ", &a);
  let vo = demo_adapters(a, 2);
  println!("{:?} ", &vo);

  println!("execute demo_adapters<T, f64>(coll, 1.5) with PointN<f64>:");
  let mut pad = PointN::<f64>::new(5);
  pad[0] = 1.5;
  pad[1] = -2.0;
  pad[2] = 0.0;
  pad[3] = 1.1;
  pad[4] = 2.2;
  // this assignment works only in local module
  // pad.items = vec![1.5, -2.0, 0.0, 1.1, 2.2];
  println!("{:?} ", pad);
  let vo = demo_adapters(pad, 1.5);
  println!("{:?} ", &vo);
}
/*-----------------------------------------------
fn demo_collection_looper<C: Debug, I: Debug>(c:&C) 
where C: IntoIterator<Item = I> + Clone
*/
fn collection_with_operation<I>(iter:&mut I, f:impl Fn(I::Item) -> ())
  where I: Iterator, I::Item: Debug
{
  print!("\n  ");
  for item in iter {
    f(item);
  }
}
fn execute_collection_with_operation() {
  /*-- plus_one on Vec --------------*/
  print!("execute collection_with_operation plus_one on Vec");
  let v = vec![1, 2, 3, 2, 1];
  let plus_one = |item:&i32| {
    print!("{} ", item + 1);
  };
  collection_with_operation(&mut v.iter(), plus_one);
  println!();
  print!("no side effects - original:");
  demo_slice_looper(&v);
  /*-- plus_one on PointN -----------*/
  print!("execute collection_with_operation plus_one on PointN");
  let mut p = PointN::<f64>::new(0);
  //let p.items = vec![1.0, 1.5, 2.0, 1.5, 1.0];
  p.push(1.0);
  p.push(1.5);
  p.push(2.0);
  p.push(1.5);
  p.push(1.0);
  let plus_one = |item:&f64| {
    let val = item + 1.0;
    print!("{val:?} ");
  };
  collection_with_operation(&mut p.iter(), plus_one);
  println!();
  print!("no side effects - same as original:");
  demo_slice_looper(&p.items);
  /*-- square on PointN -------------*/
  print!("execute collection_with_operation square on PointN");
  let mut p = PointN::<f64>::new(0);
  //let p.items = vec![1.0, 1.5, 2.0, 1.5, 1.0];
  p.push(1.0);
  p.push(1.5);
  p.push(2.0);
  p.push(1.5);
  p.push(1.0);
  let square = |item:&f64| {
    let val = item * item;
    print!("{val:?} ");
  };
  collection_with_operation(&mut p.iter(), square);
  println!();
  print!("no side effects - same as original:");
  demo_slice_looper(&p.items);
}
/*-----------------------------------------------
fn demo_collection_looper<C: Debug, I: Debug>(c:&C) 
where C: IntoIterator<Item = I> + Clone
*/
fn demo_collection_with_mod_operation<T, F>(iter:&mut dyn Iterator<Item=T>, mut f: F)
  where F: FnMut(&mut T)
{
  for mut item in iter {
    f(&mut item);
  }
}
fn execute_demo_collection_with_mod_operation() {
  /*-- plus_one_mod on slice --*/
  print!("execute demo_collection_with_mod_operation on slice");
  let mut sl = [1i32, 2, 3, 2, 1];
  let plus_one_mod = |item: &mut &mut i32| { **item += 1; print!("{:?} ", item); };
  print!("\n  ");
  demo_collection_with_mod_operation(&mut sl.iter_mut(), plus_one_mod);
  println!();
}

// fn collection_with_mod_operation<I, F>(iter:&mut I, mut f:F)
//   where I: Iterator + Debug, F: FnMut(&mut I::Item) -> ()
// {
//   print!("\n  ");
//   for mut item in iter {
//     f(&mut item);
//   }
// }
// fn execute_collection_with_mod_operation() {
//   /*-- plus_one_mod on Vec ----------*/
//   print!("execute collection_with_operation plus_one_mod on Vec");
//   let mut v = vec![1, 2, 3, 2, 1];
//   let mut plus_one_mod = |item:&mut &mut i32| {
//     **item = **item + 1;
//     print!("{} ", item);
//   };
//   collection_with_mod_operation(&mut v.iter(), plus_one_mod);
//   println!();
//   print!("no side effects - original:");
//   demo_slice_looper(&v);
//   /*-- plus_one on PointN -----------*/
//   print!("execute collection_with_operation plus_one on PointN");
//   let mut p = PointN::<f64>::new(0);
//   //let p.items = vec![1.0, 1.5, 2.0, 1.5, 1.0];
//   p.push(1.0);
//   p.push(1.5);
//   p.push(2.0);
//   p.push(1.5);
//   p.push(1.0);
//   let plus_one_mod = |item:&f64| {
//     let val = *item + 1.0;
//     print!("{:?} ", item);
//   };
//   collection_with_operation(&mut p.iter(), plus_one_mod);
//   println!();
//   print!("no side effects - same as original:");
// }
//   demo_slice_looper(&p.items);
//   /*-- square on PointN -------------*/
//   print!("execute collection_with_operation square on PointN");
//   let mut p = PointN::<f64>::new(0);
//   //let p.items = vec![1.0, 1.5, 2.0, 1.5, 1.0];
//   p.push(1.0);
//   p.push(1.5);
//   p.push(2.0);
//   p.push(1.5);
//   p.push(1.0);
//   let square = |item:&f64| {
//     let val = item * item;
//     print!("{val:?} ");
//   };
//   collection_with_operation(&mut p.iter(), square);
//   println!();
//   print!("no side effects - same as original:");
  // demo_slice_looper(&p.items);
  // /*-- square_mod on PointN ---------*/
  // print!("execute collection_with_operation square_mod on PointN");
  // let mut p = PointN::<f64>::new(0);
  // //let p.items = vec![1.0, 1.5, 2.0, 1.5, 1.0];
  // p.push(1.0);
  // p.push(1.5);
  // p.push(2.0);
  // p.push(1.5);
  // p.push(1.0);
  // let square_mod = |item:&mut f64| {
  //   let item = (*item) * (*item);
  //   print!("{item:?} ");
  // };
  // let q = p.clone();
  // collection_with_mod_operation(&mut p.iter(), square_mod);
  // println!();
  // print!("side effects - original was:");
  // demo_slice_looper(&q.items);
// }
// fn collection_with_operation<C>(iter:C, f:impl Fn(C::Item))
//   where 
//     C: Iterator,
//     C::Item: std::ops::Add<Output = C::Item> + std::ops::Mul<Output = C::Item> +
//       PartialOrd + PartialEq + Debug + Copy
// {
//   // print!("  ");
//   for mut item in iter {
//       f(item)
//   }
//   println!();
// }
// fn execute_collection_with_operation() {
//   /*-- plus_one on Vec --------------*/
//   print!("execute collection_with_operation using plus_one() on Vec");
//   let v = vec![1, 2, 3, 2, 1];
//   let plus_one = |item:&i32|
//   { 
//     print!("{} ", item + 1i32); 
//   };
//   print!("\n  ");
//   let mut iter = (&v).iter();
//   collection_with_operation(iter, plus_one);
//   print!("no side effects - original is:");
//   demo_slice_looper(&v);
//   /*-- plus_one on PointN -----------*/
//   print!("execute collection_with_operation using plus_one on PointN");
//   let mut p = PointN::<f64>::new(5);
//   p[0] = 1.0;
//   p[1] = 1.5;
//   p[2] = 2.0;
//   p[3] = 1.5;
//   p[4] = 1.0;
//   let iter = p.iter();
//   let plus_one = |item:&f64|
//   { 
//     print!("{:?} ", item + 1.0); 
//   };
//   print!("\n  ");
//   let mut iter = p.iter();
//   collection_with_operation(&mut iter, plus_one);
//   print!("no side effects - original is:");
//   demo_slice_looper(&p.items);
//  /*-- square on PointN -------------*/
//   print!("execute collection_with_operations using square on PointN");
//   print!("\n  ");
//   let mut iter = p.iter();
//   let square = |item:&f64| {
//     print!("{:?} ", item*item);
//   };
//   collection_with_operation(&mut iter, square);
//   print!("no side effects - original is:");
//   demo_slice_looper(&p.items);
//  /*-- square_mod on PointN ---------*/
//  print!("execute collection_with_operations using square_mod on PointN");
//  print!("\n  ");
//  let mut iter = p.iter();
//  let square_mod = |item:&mut f64| {
//    *item = (*item) * (*item);
//    print!("{:?} ", item);
//  };
//  collection_with_operation(&mut iter, square_mod);
//  print!("side effects - original is:");
//  demo_slice_looper(&p.items);
// }
/*-- Begin demonstrations ---------------------*/

fn main() {
  analysis_iter::show_label("Demonstrate Rust Iteration",30);

  execute_demo_array_into_iterator_loop();
  execute_demo_array_into_iterator_for();
  execute_demo_array_iter_for();
  execute_demo_vec_into_iterator_for();
  execute_demo_vec_iter_for();
  execute_demo_vec_iter_mut_for();
  execute_demo_collection_iter_mut_for();
  // execute_demo_vec_indexer();
  // execute_demo_for();
  // execute_demo_for_ref();
  // execute_demo_slice_indexer();
  // execute_demo_sub_range_indexer();
  // execute_demo_slice_looper();
  // execute_demo_collection_looper();
  // execute_demo_for_looper();
  // execute_demo_ranger();
  // execute_collection_with_operation();
  execute_demo_collection_with_mod_operation();
  execute_demo_adapters();

  println!("\nThat's all folks!\n");
}
