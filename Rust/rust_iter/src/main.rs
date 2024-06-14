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
    Vecs, VecDeques, and custom Point<T, N> type. 
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
/*-----------------------------------------------
Note:
Find all Bits code, including this in
https://github.com/JimFawcett/Bits
You can clone the repo from this link.
-----------------------------------------------*/
#![allow(dead_code)]
#![allow(unused_variables)]

/*-----------------------------------------------
  - Module analysis_iter provides functions
    for type analysis and display.
  - Module points_iter defines type Point<T, N>,
    a point in N-dimensional hyperspace.
*/
use std::collections::*;
mod analysis_iter;
use analysis_iter::*;
mod points_iter;
use points_iter::*;

use std::fmt::*;
use std::cmp::*;

/*---------------------------------------------------------
  Use of Rust iterators is encapsulated in a sequence of
  functions defined below and used in main.  This file 
  builds collections and applies various functions to
  illustrate how iterators are used.
---------------------------------------------------------*/

/*---------------------------------------------------------
  Demo Vec iteration with loop construct
  - illustrates how iteration works, using most basic
    syntax operating on Vec<T> instances.
*/
fn demo_loop_iteration() {
  show_label("basic loop iteration with Vec", 35);

  /* 1. v.iter(), iterates without consuming v */
  let v = vec![1, 2, 3, 2, 1];
  show_op("Vec iteration using loop and iter()");
  let mut itr = v.iter();
  loop {
    match itr.next() {
      Some(item) => print!("{item} "),
      None => break,
    }
  }
  // next statement is valid since v not consumed
  println!();
  println!("{v:?}");

  /* 2. v.iter_mut() iterates and mutates without consuming v */
  let mut v = vec![1, 2, 3, 2, 1];
  let mut mitr = v.iter_mut();
  show_op("mutable vec iteration with loop and mut_iter()");
  loop {
    match mitr.next() {
      Some(item) => { *item += 1; print!("{item} "); }
      None => break,
    }
  }
  println!();
  println!("{v:?}");

  /* 3. v.into_iter() consumes v while converting to iterator */
  let mut itr = v.into_iter();
  show_op("generate iterator with v.into_iter()");
  loop {
    match itr.next() {
      Some(item) => print!("{item} "),
      None => break,
    }
  }
  // into_iter() consumes v so the next statement is invalid
  // println!("{v:?}");  // v was moved
  println!();

}
/*---------------------------------------------------------
  Demo iteration with for-in loop construct
  - illustrates how for-in works, using idiomatic
    syntax operating on Vec<T> instances.
*/
fn demo_for_iteration() {
  show_label("basic for-in loop iteration using Vec", 45);

  /* 1. v.iter(), iterates without consuming v */
  let v = vec![1, 2, 3, 2, 1];
  show_op("Vec iteration using v.iter()");
  for item in v.iter() {
      print!("{item} ");
  }
  // next statement is valid since v not consumed
  println!();
  println!("{v:?}");

  /* 2. v.iter_mut() iterates and mutates without consuming v */
  let mut v = vec![1, 2, 3, 2, 1];
  show_op("mutable vec iteration using v.iter_mut()");
  println!("original:   {v:?}");
  for item in v.iter_mut() {
    *item += 1;
      print!("{item:?} ");
  }
  println!();
  println!("after iter: {v:?}");

  /* 3. v.into_iter() consumes v while converting to iterator */
  // let mut itr = v.into_iter();
  show_op("for-in uses v.into_iter()");
  for item in v.into_iter() {
    print!("{item:?} ");
  }
  // into_iter() consumes v so the next statement is invalid
  // println!("{v:?}");  // v was moved
  println!();
  /* 
     4. iteration with for-in consumes v 
        - same as 3. except that into_iter() is used implicitly
        - used in preference to 3
  */
  let v = vec![1, 2, 3, 4, 5];
  show_op("for-in uses v => into_iter()");
  for item in v {  // implicitly uses into_iter()
    print!("{item:?} ");
  }
  // into_iter() consumes v so the next statement is invalid
  // println!("{v:?}");  // v was moved
  println!();

  /*
     5. iteration over elements of v using &v
        - uses internal call to into_iter() implemented with
          Vec::iter() so v not moved
  */
  let v = vec![1, 2, -1, -2, 0];
  show_op("for-in uses &v => iter()");
  for item in &v {
    print!("{item:?} ");
  }
  println!();
  println!("{v:?}");  // v was not moved
  /*
     6. mutating iteration over elements of v using &mut v
        - generates into_iter() implemented with internal
          call to iter_mut(), so does not move v
  */
  let mut v = vec![1, 2, -1, -2, 0];
  show_op("for-in uses &mut v => iter_mut()");
  println!("original: {v:?}");
  for item in &mut v {
    *item += 1;
    print!("{item:?} ");
  }
  println!();
  println!("modified: {v:?}");  // v was not moved

  /*------------------------------------------------------- 
    Iteration forms 4, 5, and 6 are the preferred useage.
    Forms 1, 2, and 3 show how for-in loops work. 
  -------------------------------------------------------*/
}
/*---------------------------------------------------------
  Demo iteration over coordinate values in Point<T,N>
  - illustrates how to implement iteration for custom
    types, using definitions in points_iter.rs module.
*/
fn demo_point_iteration() {

  show_label("demo point iteration", 30);

  /* uses Point<T,N>::IntoIterator => into_iter() => move */
  let mut p = Point::<i32, 5>::new();
  p.init(&vec![3, 2, 1, 0, -1]);
  show_op("for-in uses p, generating iter from p.into_iter()");
  for item in p {
    print!("{item:?} ");
  }
  // into_iter() consumes v so the next statement is invalid
  // println!("{p:?}");  // v was moved
  println!();

  /* uses &Point<T,N>::IntoIterator => iter() => no move */
  let mut p = Point::<i32, 5>::new();
  p.init(&vec![3, 2, 1, 0, -1]);
  show_op("for-in uses &p, generating iter from &p.iter()");
  println!("original:   {p:?}");
  for item in &p {
    print!("{item:?} ");
  }
  println!();
  println!("after iter: {p:?}");  // v was not moved

  /* uses &mut Point<T,N>::IntoIterator => iter_mut() => no move */
  let mut p = Point::<i32, 5>::new();
  p.init(&vec![-3, -2, -1, 0, 1]);
  show_op("for-in uses &mut p, generating iter from &mut p.iter_mut()");
  println!("original:  {:?}", p);
  for item in &mut p {
    *item += 1;
    print!("{item:?} ");
  }
  println!();
  println!("modified:  {p:?}");  // v was not moved
  
}
/*---------------------------------------------------------
  Demonstrate iter() by displaying a comma seperated
  list (csl) of items in several common collections.
  - three different strategies used for making
    display comma-seperated
  - syntax used in this demo
    - iter().next() -> Option<Self::Item>
    - enum Option<T> { Some(T), None, }
*/
fn demo_iter() {
  show_label("demo_iter()", 20);

  /*-------------------------------------------------- 
    iterate over array
    csl strategy #1 extracts first item before iterating 
  --------------------------------------------------*/
  show_op("array iter using loop");
  let ar = [1, 2, 3, 4];
  let mut iter = ar.iter();  // extract first item
  if let Some(item) = iter.next() {
    print!("{item}");
  }
  loop {
    let item = iter.next();
    match item {  //display remaining items
      Some(item) => print!(", {}", item),
      None => break
    }
  }
  println!();
  // ar not consumed by ar.iter(), above
  // so statement below is valid:
  println!("using println!:\n{:?}", ar);

  /*-------------------------------------------------- 
    iterate over Vec
    csl strategy #2 uses first flag 
  --------------------------------------------------*/
  /*--- functionaly equivalent to loop, above ---*/
  show_op("Vec iter using for-in");
  let v = vec![1, 2, 3, 4];
  let mut first = true;   // set first flag
  for item in &v {
    if first {
      print!("{item}");
      first = false;            // reset first flag
    }
    else {
      print!(", {item}");
    }
  }
  println!();
  // statement below is valid, v not consumed since
  //   for-in used reference &v
  println!("using println!:\n{:?}", v);

  /*-------------------------------------------------- 
    iterate over HashMap
    csl strategy #3 uses enumerate() 
  --------------------------------------------------*/
  show_op("HashMap iter using for-in");
  let mut hm = HashMap::<&str, i32>::new();
  hm.insert("zero", 0);
  hm.insert("one", 1);
  hm.insert("two", 2);
  hm.insert("three", 3);
  /*------------------------------------------------------- 
    enumerate is an iterator adapter that returns another 
    iterator yielding (count, value) where value is 
    yielded by iter 
  */
  for (count, item) in hm.iter().enumerate() {
    if count == 0 {
      print!("{item:?}");
    }
    else {
      print!(", {item:?}");
    }
  }
  println!();
  println!("using println!:\n{:?}", hm);

  /*--------------------------------------------------
    iterate over Point<T, N>, 
    csl strategy same as above 
  --------------------------------------------------*/
  show_op("Point iter using for-in");
  let mut p = Point::<f64, 5>::new();
  p.init(&vec![1.0, 1.5, 2.0, 1.5, 1.0]);
  for item in p.iter().enumerate() {
    if item.0 == 0 {  // count == zero
      print!("{:?}", item.1);
    }
    else {  // count > 0
      print!(", {:?}", item.1);
    }
  }
  println!();
  print!("using println!:\n{p:?}");  // p not moved
  println!("\n");

  /*-------------------------------------------------- 
    Use formatting function that accepts any type 
    implementing IntoIterator trait.
    - function defined below 
  --------------------------------------------------*/
  show_op("using show_csl(&ar) for array");
  show_csl(&ar); // ar not consumed
  show_op("using show_csl(&v) for Vector");
  show_csl(&v);  // v not consumed
  show_op("using show_csl(&hm) for HashMap");
  show_csl(&hm); // hm not consumed
  show_op("using show_csl(&p) for Point");
  show_csl(&p);  // p not consumed
  println!();

  show_op("using show_csl(ar) for array - copies ar");
  show_csl(ar);  // ar is not consumed as it is a copy type
  show_op("using show_csl(v) for Vector - moves v");
  show_csl(v);   // v is consumed
  show_op("using show_csl(hm) for HashMap - moves hm");
  show_csl(hm);  // hm is consumed
  show_op("using show_csl(p) for Point - moves p");
  show_csl(p);   // p is consumed
}

/* generalize csl strategy #3 */
fn show_csl<C>(c:C)  // consumes c
  where C: IntoIterator, C::Item: Debug 
{
  let iter = c.into_iter();
  for (count, val) in iter.enumerate() {
    if count == 0 {
      print!("{:?}", val);
    }
    else {
      print!(", {:?}", val);
    }
  }
  println!();
}
/*---------------------------------------------------------
  Demonstrate iterator methods
*/
fn demo_methods() {
  show_label("iterator methods", 25);

  show_op("original vector");
  let mut v = vec![1, 2, 3, 2, 1];
  println!("{v:?}");
  show_op("modified using for_each()");
  /* inplace modification of elements of v */
  v.iter_mut().for_each(|item| *item *= *item);
  println!("{v:?}");

  show_op("collect squared items from vec into array");
  let sq:[i32; 5] = 
    /* return iterator over squared items from v */
    v.iter().map(|&item| item * item)
      /* collect invokes iterator to load modified elements into sq */
      .collect::<Vec<i32>>().try_into()
      /* display message if collection panics, e.g., fails and terminates */
      .expect("incorrect length");
  println!("{sq:?}");

  show_op("filter out elements larger than 20");
  let filtered: Vec<i32> = 
    /* create iterator over filtered elements */
    sq.iter().filter(|&&item| item <= 20)
    /* copy filtered element and collect into Vec */
    .cloned().collect();
  println!("{filtered:?}");
}
/*---------------------------------------------------------
  Related operations:
  - All functions below were learning experiments imple-
    mented while I was preparing iteration code demos.
  - A few use techniques not discussed above. Most use
    code very similar to the demos above.
  - Of special note are the generic functions and their
    often not so obvious signatures.
---------------------------------------------------------*/
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
/*---------------------------------------------------------
  demo_collection_into_iterator
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
  https://stackoverflow.com/questions/49962611/why-does-str-not-implement-intoiterator
*/
fn demo_collection_into_iterator<C: Debug, I: Debug>(c:C) 
  where C: IntoIterator<Item = I> + Clone
{
  /* generate comma separated list */
  let mut iter = c.into_iter();
  if let Some(item) = iter.next() {
    print!("{:?}", item);  // print first with no leading comma
  }
  for item in iter {
    print!(", {:?}", item);
  }
}
fn execute_demo_collection_into_iterator() {
  /*-- Vec -------------*/
  print!("execute demo_collection_into_iterator for array");
  let arr = [1i32, 2, 3, 2, 1];
  print!("\n  ");
  demo_collection_into_iterator(arr);
  println!();
  /*-- Vec -------------*/
  print!("execute demo_collection_into_iterator for Vec");
  let v = vec![1i32, 2, 3, 2, 1];
  print!("\n  ");
  demo_collection_into_iterator(v);
  println!();
  /*-- PointN ----------*/
  print!("execute demo_collection_into_iterator for Point");
  let mut p = Point::<f64, 5>::new();
  p[0] = 1.0;
  p[1] = 1.5;
  p[2] = 2.0;
  p[3] = 1.5;
  p[4] = 1.0;
  print!("\n  ");
  demo_collection_into_iterator(p);
  println!();
}
/*---------------------------------------------------------*/
fn demo_array_iter_for<T:Debug>(arr:&[T; 5]) {
  for item in arr.iter() {
    print!("{:?} ", item);
  }
}
fn execute_demo_array_iter_for() {
  print!("execute demo_array_iter_for");
  let arr = [1.0f64, 2.0, 3.0, 4.0, 5.0];
  let iter = arr.iter();
  print!("\n  ");
  demo_array_iter_for::<f64>(&arr);  // ::<f64> optional here
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
  println!();
}
/*---------------------------------------------------------*/
fn demo_collection_iter_for<T, F>(c:impl Iterator<Item = T>, f:F) 
  where 
    F: Fn(&T) -> ()
{
  for item in c {
    f(&item);
  }
}
// /*---------------------------------------------------------*/
// fn demo_collection_iter_for<T, F>(c:&mut dyn Iterator<Item = T>, f:F) 
//   where 
//     F: Fn(&T) -> ()
// {
//   for item in c {
//     f(&item);
//   }
// }
fn execute_demo_collection_iter_for() {
  
  print!("execute demo_collection_iter_for with array");
  let plus_one = |item: &&f64| { print!("{:?} ", item); };
  let a = [1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  ");
  demo_collection_iter_for(&mut a.iter(), plus_one);
  println!();

  print!("execute demo_collection_iter_for with slice");
  let sl = &a[1..3];
  print!("\n  ");
  demo_collection_iter_for(&mut sl.iter(), plus_one);
  println!();

  print!("execute demo_collection_iter_for with Vec");
  let v = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
  print!("\n  ");
  demo_collection_iter_for(&mut v.iter(), plus_one);
  println!();
  
  print!("execute demo_collection_iter_for with Map");
  let mut m = HashMap::<String, i32>::new();
  m.insert("zero".to_owned(), 0);
  m.insert("one".to_owned(), 1);
  m.insert("two".to_owned(), 2);
  m.insert("three".to_owned(), 3);
  let plus_one2 = |item: &(&String, &mut i32)| { print!("{:?} ", item); };
  print!("\n  ");
  demo_collection_iter_for(&mut m.iter_mut(), plus_one2);
  println!();

  print!("execute demo_collection_iter_mut_for with Point");
  let mut p = Point::<f64, 7>::new();
  p[0] = 1.0;
  p[1] = 1.5;
  p[2] = 2.0;
  p[3] = 2.5;
  p[4] = 3.0;
  p[5] = 3.5;
  p[6] = 4.0;
  print!("\n  ");
  demo_collection_iter_for(&mut p.iter(), plus_one);
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

  print!("execute demo_collection_iter_mut_for with slice");
  let sl = &mut a[1..3];
  let plus_one_mod = |item: &mut &mut f64| { **item += 1.0; };
  print!("\n  input:  {:?}", sl);
  demo_collection_iter_mut_for(&mut sl.iter_mut(), plus_one_mod);
  print!("\n  output: {:?}", sl);
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
  let plus_one2 = |item: &mut (&String, &mut i32)| { *item.1 += 1; };
  print!("\n  input:\n    {:?}", m);
  demo_collection_iter_mut_for(&mut m.iter_mut(), plus_one2);
  print!("\n  output:\n    {:?}", m);
  println!();

  print!("execute demo_collection_iter_mut_for with Point");
  let mut p = Point::<f64, 7>::new();
  p[0] = 1.0;
  p[1] = 1.5;
  p[2] = 2.0;
  p[3] = 2.5;
  p[4] = 3.0;
  p[5] = 3.5;
  p[6] = 4.0;
  print!("\n  input:\n    {:?}", p);
  demo_collection_iter_mut_for(&mut p.iter_mut(), plus_one);
  print!("\n  output:\n    {:?}", p);
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
  into destination collection, e.g., return type.
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

  println!("execute demo_adapters<T, f64>(coll, 1.5) with Point<f64, 5>:");
  let mut pad = Point::<f64, 5>::new();
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
  The functions below have some related details
  that are worth saving, but can be skipped for
  first study.
*/
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
  print!("execute demo_for_looper with Point");
  let mut p = Point::<i32, 7>::new();
  p.init(&vec![1, 2, 3, 4, 3, 2, 1]);
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
  print!("execute demo_ranger with Point iter");
  let mut p = Point::<i32, 7>::new();
  p.init(&vec![1, 2, 3, 4, 3, 2, 1]);
  demo_ranger(&mut p.iter());
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
  print!("execute collection_with_operation plus_one on Point");
  let mut p = Point::<f64, 5>::new();
  p.init(&vec![1.0, 1.5, 2.0, 1.5, 1.0]);
  let plus_one = |item:&f64| {
    let val = item + 1.0;
    print!("{val:?} ");
  };
  collection_with_operation(&mut p.iter(), plus_one);
  println!();
  print!("no side effects - same as original:");
  demo_slice_looper(&p.items);
  /*-- square on PointN -------------*/
  print!("execute collection_with_operation square on Point");
  let mut p = Point::<f64, 5>::new();
  p.init(&vec![1.0, 1.5, 2.0, 1.5, 1.0]);
  let square = |item:&f64| {
    let val = item * item;
    print!("{val:?} ");
  };
  collection_with_operation(&mut p.iter(), square);
  println!();
  print!("no side effects - same as original:");
  demo_slice_looper(&p.items);
}

fn test() {
  println!();
  analysis_iter::show_note("Testing related operations");
  execute_demo_vec_indexer();
  execute_demo_sub_range_indexer();
  execute_demo_slice_looper();
  execute_demo_for_looper();
  execute_demo_ranger();
  execute_demo_collection_iter_for();
  execute_demo_collection_iter_mut_for();
  execute_demo_collection_into_iterator();
  execute_collection_with_operation();
}
/*-- Begin demonstrations ---------------------*/

fn main() {
  analysis_iter::show_label("Demonstrate Rust Iteration",30);
  demo_loop_iteration();
  demo_for_iteration();
  demo_point_iteration();
  demo_iter();
  demo_methods();

  const DOTEST:bool = true;
  if DOTEST {
    test();
  }

  // analysis_iter::show_op("into_iterator()");
  // println!();
  // execute_demo_array_into_iterator_loop();
  // execute_demo_array_into_iterator_for();
  // execute_demo_vec_into_iterator_for();
  // execute_demo_collection_into_iterator();
  // println!();

  // analysis_iter::show_op("iter()");
  // println!();
  // execute_demo_array_iter_for();
  // execute_demo_vec_iter_for();
  // execute_demo_collection_iter_for();
  // println!();

  // analysis_iter::show_op("iter_mut()");
  // println!();
  // execute_demo_vec_iter_mut_for();
  // execute_demo_collection_iter_mut_for();
  // println!();

  // analysis_iter::show_op("iteration adapters");
  // println!();
  // execute_demo_adapters();

  println!("\nThat's all folks!\n");

}
