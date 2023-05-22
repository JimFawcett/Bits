/*-----------------------------------------------
  analysis_iter:
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
use std::fmt::*;
use std::cmp::*;

/*-----------------------------------------------
  Simplest case - displays Vector with
  generic Item type.
  - very close to C++ version
  - see comments for simple_indexer, below
*/
pub fn whiler<T: Debug>(v:&Vec<T>) {
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
pub fn simple_indexer<T:Debug>(s:&[T]) {
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
  Iterates over slice s without indexing
  - Works with any collection with contiguous
    fixed size elements,
    e.g., array, Vector, PointN, ...
*/
pub fn simple_looper<T:Debug>(s:&[T]) {
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
pub fn looper<C: Debug, I: Debug>(c:&C) 
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
pub fn for_looper<C: Debug, I: Debug>(c:&C) 
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

pub fn find_last_utf8(s:&str, chr: char) -> Option<usize> {
    s.chars().rev().position(|c| c == chr)
     .map(|rev_pos| s.chars().count() - rev_pos -1)
}
/*-----------------------------------------------
  Displays contents of iterator, often passed in
  as a range.
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
