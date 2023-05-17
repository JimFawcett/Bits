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

use std::fmt::*;
use std::cmp::*;
use std::collections::*;
mod points_iter;
use points_iter::*;

/*-----------------------------------------------
  Test functions with increasing functionality 
  and increasing generality of inputs:
  -----------------------------------------------
  - whiler<T:Debug>(v:&Vec<T>)
  - simple_indexer<T:Debug>(s:&[T])
  - sub_range_indexer<T:Debug>(
      s:&[T], 
      mut lower:usize, mut upper:usize
    )
  - simple_looper<T:Debug>(s:&[T])
  - looper<C: Debug, I: Debug>(c:&C)
      where C: IntoIterator<Item = I> + Clone
  - for_looper<C: Debug, I: Debug>(c:&C) 
      where C: IntoIterator<Item = I> + Clone
  - ranger<T>(iter: &mut T)
      where T: Iterator, T::Item: Debug
*/

/*-----------------------------------------------
  Simplest case - displays Vector with
  generic Item type.
  - very close to C++ version
  - see comments for simple_indexer, below
*/
fn whiler<T: Debug>(v:&Vec<T>) {
    let mut i = 0;
    while i < v.len() {
        print!("{:?} ", v[i]);
        i += 1;
    }
    println!();
}
/*-----------------------------------------------
  Illustrates indexing
  - Not idiomatic, but safe and correct.
  - Works with any collection with contiguous
    fixed size elements.
  - Won't work with String or str inputs; they
    are collections of utf8 chars which vary
    in size from 1 to 4 bytes.
  - It is rare for idomatic Rust code to use
    indexing. 
  - &[T] is slice type
*/
#[allow(clippy::needless_range_loop)]
fn simple_indexer<T:Debug>(s:&[T]) {
    let max = s.len();
    /* 0..max is range iterator */
    for i in 0..max {
        print!("{:?} ", s[i]);
    }
    println!();
    /*---------------------------
      clippy prefers:
      for item in s.iter().take(max) {
        print!("{item} ");
      }
    */
}
/*-----------------------------------------------
  Iterates over a sub-range of the slice s
  - works with any collection with contiguous
    fixed size elements.
*/
#[allow(clippy::needless_range_loop)]
fn sub_range_indexer<T:Debug>(
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
  Iterates over slice s without indexing
  - Works with any collection with contiguous
    fixed size elements,
    e.g., array, Vector, PointN, ...
*/
fn simple_looper<T:Debug>(s:&[T]) {
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
  fn looper prints out a comma separated list of
  Collection<I>'s items.
  - where clause requires C to implement 
    IntoIterator trait. 
  - C is type of collection, I is type of C's items
  - Accepts any collection type that implements
    IntoIterator trait, e.g., array, slice, Vector, ...
  - looper can't accept String or &str
    because String does not implement IntoIterator
  - That's because String provides two iterators,
    chars() to iterate multibyte chars and bytes()
    to iterate over bytes.
    https://stackoverflow.com/questions/49962611/why-does-str-not-implement-intoiterator
*/
fn looper<C: Debug, I: Debug>(c:&C) 
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
  similar to looper but uses for loop and erases
  last comma
*/
fn for_looper<C: Debug, I: Debug>(c:&C) 
    where C: IntoIterator<Item = I> + Clone
{
    /* build string of comma separated values */
    let mut accum = String::new();
    let cc = c.clone();

    for item in cc {
        accum += &format!("{item:?}, ");
    }
    /* remove last comma */
    let opt = find_last_utf8(&accum, ',');
    if let Some(index) = opt {
        accum.truncate(index);
    }
    println!("{accum}");
}

fn find_last_utf8(s:&str, chr: char) -> Option<usize> {
    s.chars().rev().position(|c| c == chr)
     .map(|rev_pos| s.chars().count() - rev_pos -1)
}
/*-----------------------------------------------
  Displays contents of iterator, often passed in
  as a range.
*/
fn ranger<T>(iter: &mut T)
    where T: Iterator, T::Item: Debug
{
    for item in iter {
        print!("{item:?} ")
    }
    println!();
}
/*-----------------------------------------------
  Iterator adapters:
  - demo_adapters iterates over collection C,
    removes non-positive items, adds second 
    argument i and collects into vector.
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

/*-- Begin demonstrations ---------------------*/
fn main() {
    println!("------------------------------------");
    println!(" --- Demonstrate Rust Iteration ---");
    println!("------------------------------------");
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
    
    /*-- whiler -------------------------------*/
    println!("\nwhiler displays Vec<T>");
    let v = vec![1, 6, 2, 5, 3, 4];
    whiler(&v);  // can only display vecs
    println!();

    /*-- simple_indexer -----------------------*/
    println!("simple_indexer displays slice");
    simple_indexer(s);
    println!("simple_indexer displays vector");
    let v = vec![5, 4, 3, 2, 1, 0];
    simple_indexer(&v);
    println!("simple_indexer displays string bytes");
    let str1:&str = "a string";
    simple_indexer(str1.as_bytes());
    println!();
    
    /*-- sub_range_indexer --------------------*/
    println!("sub_range_indexer displays slice");
    sub_range_indexer(s, 2, 5);
    println!();
    
    /*-- simple_looper ------------------------*/
    println!("simple_looper displays slice");
    simple_looper(s);
    println!("simple_looper displays vector");
    simple_looper(&v);
    println!("simple_looper displays PointN");
    let mut point = PointN::<i32>::new(5);
    point[1] = 2;
    point[3] = -3;
    let ps = &point[0..];  // take slice
    simple_looper(ps);
    println!();

    /*-- looper -------------------------------*/
    println!("looper displays slice:");
    looper(s);
    println!("looper displays array");
    let a = [1, 2, 3];
    looper(&a);
    println!("looper displays VecDeque");
    let vecdeq = VecDeque::from([4, 3, 2, 0, -1]);
    looper(&vecdeq);
    println!("looper displays PointN");
    let pc = point.clone();
    looper(&pc);
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
