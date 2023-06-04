/*-----------------------------------------------
  Bits::rust_iter::main.rs
  - demonstrates iteration over collections with
    Rust iterators
  - Most collections implement the Rust trait
    IntoIterator which consumes the collection
    to generate an iterator.
  - Demonstrates iteration over arrays, slices,
    Vecs, VecDeques, and custom PointN<T> type. 
-----------------------------------------------*/

#![allow(dead_code)]
#![allow(unused_variables)]

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

  It starts with a few quick iteration examples before
  applying the analysis functions.
---------------------------------------------------------*/
/*
- vec_indexer<T:Debug>(v:&Vec<T>)
- slice_indexer<T:Debug>(s:&[T])
- sub_range_indexer<T:Debug>(
    s:&[T], 
    mut lower:usize, mut upper:usize
  )
- slice_looper<T:Debug>(s:&[T])
- collection_looper<C: Debug, I: Debug>(c:&C)
    where C: IntoIterator<Item = I> + Clone
- for_looper<C: Debug, I: Debug>(c:&C) 
    where C: IntoIterator<Item = I> + Clone
- ranger<T>(iter: &mut T)
    where T: Iterator, T::Item: Debug
*/
use std::fmt::*;
use std::cmp::*;

/*-----------------------------------------------
  vec_indexer<T: Debug>(v:&Vec<T>)
-------------------------------------------------
Simplest case - displays Vector with generic
Item type.
- uses naive indexing
- works only for Vec's
*/
pub fn vec_indexer<T: Debug>(v:&Vec<T>) {
  let mut i = 0;
  while i < v.len() {
    print!("{:?} ", v[i]);
    i += 1;
  }
  println!();
}
/*-----------------------------------------------
  slice_indexer<T:Debug>(s:&[T])
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
pub fn slice_indexer<T:Debug>(s:&[T]) {
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
/*-----------------------------------------------
  sub_range_indexer<T:Debug>(
    s:&[T], mut lower:usize, mut upper:usize
  )
Iterates over a sub-range of the slice s
- works with any collection with contiguous
  fixed size elements.
*/
#[allow(clippy::needless_range_loop)]
pub fn sub_range_indexer<T:Debug>(
  s:&[T], mut lower:usize, mut upper:usize
) {
  lower = max(0, lower);
  upper = min(s.len(), upper);
  if lower <= upper {
    for i in lower..upper {
      print!("{:?} ", s[i]);
    }
  }
  println!();
}
/*-----------------------------------------------
  slice_looper<T:Debug>(s:&[T])
-------------------------------------------------
Iterates over slice s without indexing
- Works with any collection with contiguous
  fixed size elements,
  e.g., array, Vector, PointN, ...
- Uses slice iterator.
*/
pub fn slice_looper<T:Debug>(s:&[T]) {
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
pub fn collection_looper<C: Debug, I: Debug>(c:&C) 
where C: IntoIterator<Item = I> + Clone
{
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
/*----------------------------------------------- 
  for_looper<C: Debug, I: Debug>(c:&C) 
-------------------------------------------------
- prints comma separated list of Collection<I>'s 
  items.
- similar to collection_looper but erases last comma
  so no need for collection or clone
- uses idiomatic forloop with no indexing
*/
pub fn for_looper<C: Debug, I: Debug>(c:&C) 
  where C: IntoIterator<Item = I> + Clone
{
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
/*-----------------------------------------------
  ranger<T>(iter: &mut T)
-------------------------------------------------
- Displays contents of iterator, often passed in
  as range.
- another idiomatic iteration.
*/
pub fn ranger<T>(iter: &mut T)
  where T: Iterator, T::Item: Debug
{
  for item in iter {
      print!("{item:?} ")
  }
  println!();
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
pub fn demo_adapters<C, I>(c: C, i: I) -> Vec<I>
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

/*-- Begin demonstrations ---------------------*/

fn main() {
  analysis_iter::show_label("Demonstrate Rust Iteration",30);

  let s = &mut [1usize, 2, 3, 4, 3, 2, 1];
  println!("slice s = {s:?}");
  println!("s[2] = {:?}", s[2usize]);
  
  /*-- PointN<T>.into_iter() -----------------*/
  let mut p = PointN::<i32>::new(5);
  p[1] = 1;
  p[3] = -1;
  println!("\n{p:?}");
  println!("using PointN<i32>.iter");
  for item in p.iter() {
      print!("{item:?} ");
  }
  println!("\nusing PointN<i32>.into_iter");
  let iter = p.clone().into_iter(); // consumes clone
  analysis_iter::show_op("displaying iter type");
  analysis_iter::show_type(&iter, "iter");
  for item in iter {
      print!("{item} ");
  }
  println!();
  println!("using PointN<i32>.into_iter iter() with auto deref");
  let pc = p.clone();
  for item in pc {  // auto deref of pc into pc.iter()
      print!("{item} " ) // consumes pc
  }
  println!();
  println!("using PointN<i32>.iter()");
  for item in p.iter() { // does not consume p
      print!("{item} " )
  }
  println!();
  println!("using PointN<i32>.iter_mut()");
  for item in p.iter_mut() { // does not consume p
      *item *= 2;
      print!("{item} " )
  }
  println!("\n{p:?}");
  
  /*-- vec_indexer -------------------------------*/
  println!("\nvec_indexer displays Vec<T>");
  let v = vec![1, 6, 2, 5, 3, 4];
  vec_indexer(&v);  // can only display vecs
  println!();

  /*-- slice_indexer -----------------------*/
  println!("slice_indexer displays slice");
  slice_indexer(s);
  println!("slice_indexer displays vector");
  let v = vec![5, 4, 3, 2, 1, 0];
  slice_indexer(&v);
  println!("slice_indexer displays string bytes");
  let str1:&str = "a string";
  slice_indexer(str1.as_bytes());
  println!();
  
  /*-- sub_range_indexer --------------------*/
  println!("sub_range_indexer displays slice");
  sub_range_indexer(s, 2, 5);
  println!();
  
  /*-- slice_looper ------------------------*/
  println!("slice_looper displays slice");
  slice_looper(s);
  println!("slice_looper displays vector");
  slice_looper(&v);
  println!("slice_looper displays PointN");
  let mut point = PointN::<i32>::new(5);
  point[1] = 2;
  point[3] = -3;
  let ps = &point[0..];  // take slice
  slice_looper(ps);
  println!();

  /*-- collection_looper -------------------------------*/
  println!("collection_looper displays slice:");
  collection_looper(s);
  println!("collection_looper displays array");
  let a = [1, 2, 3];
  collection_looper(&a);
  println!("collection_looper displays VecDeque");
  let vecdeq = VecDeque::from([4, 3, 2, 0, -1]);
  collection_looper(&vecdeq);
  println!("collection_looper displays PointN");
  let pc = point.clone();
  collection_looper(&pc);
  println!("{pc:?}");
  println!();

  /*-- for_looper ---------------------------*/
  println!("for_looper displays slice:");
  for_looper(s);
  println!("for_looper displays vector:");
  let vec = s.to_vec();
  for_looper(&vec);
  println!("for_looper displays VecDeque");
  for_looper(&vecdeq);
  println!("for_looper displays PointN<T>");
  let pc = point.clone();
  for_looper(&pc);
  println!();
  /*------------------------------------------- 
    for_looper can't accept String or &str
    because they do not implement IntoIterator
    https://stackoverflow.com/questions/49962611/why-does-str-not-implement-intoiterator
  */

  /*-- ranger -------------------------------*/
  println!("ranger displays string:");
  let str = "a literal string".to_string();
  ranger(&mut str.chars());
  println!("ranger displays values in range");
  ranger(&mut (0..10));
  println!("ranger accepts Vector iterator");
  ranger(&mut vec.iter());
  println!("ranger accepts VecDeque iterator");
  ranger(&mut vecdeq.iter());
  println!("ranger accepts PointN<T> iterator");
  ranger(&mut point.iter());
  println!();

  /*-- demo_adapters ------------------------*/
  println!("demo_adapters<T, i32>(coll, 2) accepts array:");
  let a = [1, -1, 0, 2, 3, 4];
  println!("{:?} ", &a);
  let vo = demo_adapters(a, 2);
  println!("{:?} ", &vo);

  println!("demo_adapters<T, f64>(coll, 1.5) accepts PointN<f64>:");
  let mut pad = PointN::<f64>::new(5);
  pad[0] = 1.5;
  pad[1] = -2.0;
  pad[2] = 0.0;
  pad[3] = 1.1;
  pad[4] = 2.2;
  // this assignment works only in local module
  // pad.items = vec![1.5, -2.0, 0.0, 1.1, 2.2];
  println!("{:?} ", &pad);
  let vo = demo_adapters(&pad, 1.5);
  println!("{:?} ", &vo);

  println!("\nThat's all folks!\n");
}
