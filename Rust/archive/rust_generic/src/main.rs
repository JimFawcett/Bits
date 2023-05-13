use std::fmt::*;
use std::cmp::*;
use std::collections::*;

/*-----------------------------------------------
  Simplest case - displays Vector with
  generic type.
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
    e.g., array, Vector
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
  - 'a annotation says that c and its items last 
    at least as long as the function call.
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
   where for<'a> &'a C: IntoIterator<Item = &'a I>, 
   I: Default + Clone
{
    /* convert c into Vec to get len() method */
    let iter = c.into_iter();
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
    where for<'a> &'a C: IntoIterator<Item = &'a I>,
    I: Default + Clone
{
    /* build string of comma separated values */
    let mut accum = String::new();
    for item in c {
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
fn ranger<T: Debug>(iter: &mut T)
    where T: Iterator, T::Item: Debug
{
    for item in iter {
        print!("{item:?} ")
    }
    println!();
}
use core::default::*;
use std::option::*;
#[derive(Debug, Clone)]
struct Point<T> 
    where T:Debug + Default + Clone
{
    coor: Vec<T>,
    // iter: PointIter<'a, T>
}
impl<T> Point<T> 
    where T:Debug + Default + Clone
{
    fn new(n:usize) -> Point<T> {
        Point::<T> { 
            coor: vec![T::default(); n],
            //iter: PointIter{ pt:None, count:0 }
        }
    }
}
impl<T:Debug, Idx> std::ops::Index<Idx> for Point<T> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    type Output = Idx::Output;

    fn index(&self, index:Idx) -> &Self::Output {
        &self.coor[index]
    }
}
impl<'a, T, Idx> std::ops::IndexMut<Idx> for Point<T> 
    where
        T:Debug + Default + Clone, 
        Idx: std::slice::SliceIndex<[T]>
{
    fn index_mut(&mut self, index:Idx) -> &mut Self::Output {
        &mut self.coor[index]
    }
}
#[derive(Debug, Clone)]
struct PointIter<'a, T>
    where T: Debug + Default + Clone
{
    pt: Option<&'a Point<T>>,
    count: usize,
}
impl<'a, T:Debug> Point<T> 
    where T: Debug + Default + Clone
{
    fn iter(&self) -> PointIter<T> {
        PointIter{ pt: Some(&self), count:0 }
    }
}
impl<'a, T> Iterator for PointIter<'a, T> 
    where T: Debug + Default + Clone
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.pt.unwrap().coor.get(self.count) {
            None => None,
            Some(val) => {
                self.count += 1;
                Some(val)
            }
        }
    }
}
// impl<T> Iterator for Point<T>
//     where T: Debug + Default + Clone
// {
//     type Item = T;
//     fn next(&mut self) -> Option<T> {
//         let mut tmp = self.coor.iter();
//         tmp.next()
//         // let mut iter = self.coor.iter();
//         // let item = iter.next().cloned();
//         // print!("{item:?} ");
//         // item
//     }
// }
impl<T> IntoIterator for Point<T>
    where T:Debug + Default + Clone
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.coor.into_iter()
    }
}
fn test<T>(point_test:&Point<T>) 
    where T: Debug + Default + Clone
{
    let mut iter = point_test.iter();
    loop {
        let item = iter.next();
        match item {
            Some(val) => print!("{val:?} "),
            None => break,
        }
    }
    println!();
}
fn main() {
    println!("----------------------------------");
    println!("--- Demonstrate Rust Iteration ---");
    println!("----------------------------------");

    let mut point_test = Point::<i32>::new(6);
    point_test[0] = 1;
    point_test[4] = -1;
    let mut iter = point_test.iter();
    loop {
        let item = iter.next();
        match item {
            Some(val) => print!("{val} "),
            None => break,
        }
    }
    println!();
    test(&point_test);

    for item in point_test.iter() {
        print!("{item} ");
    }
    println!();

    let s = &[1usize, 2, 3, 4, 3, 2, 1];
    println!("slice s = {s:?}");
    println!("s[2] = {:?}", s[2usize]);
    
    println!("\nwhiler displays Vec<T>");
    let v = vec![1, 6, 2, 5, 3, 4];
    // let vc = v.clone();
    // let iter = vc.iter();
    // for item in iter {
    //     print!("{item} ");
    // }
    // println!();
    whiler(&v);
    println!();

    println!("simple_indexer displays slice");
    simple_indexer(s);
    println!("simple_indexer displays vector");
    let v = vec![5, 4, 3, 2, 1, 0];
    simple_indexer(&v);
    println!("simple_indexer displays string bytes");
    let str1:&str = "a string";
    simple_indexer(str1.as_bytes());
    println!();
    
    println!("sub_range_indexer displays slice");
    sub_range_indexer(s, 2, 5);
    println!();

    println!("simple_looper displays slice");
    simple_looper(s);
    println!("simple_looper displays vector");
    simple_looper(&v);
    println!("simple_looper displays point");
    let mut point = Point::<i32>::new(5);
    point[1] = 1;
    point[3] = -1;
    let ps = &point[0..];
    simple_looper(&ps);
    println!();

    println!("looper displays slice:");
    looper(s);
    println!("looper displays array");
    let a = [1, 2, 3];
    looper(&a);
    println!("looper displays VecDeque");
    let vecdeq = VecDeque::from([4, 3, 2, 0, -1]);
    looper(&vecdeq);
    println!("looper displays point");
    let pc = point.clone();
    let iter = pc.iter();
    for item in iter {
        print!("{item} ");
    }
    println!();
    let pc = point.clone();
    for item in pc.into_iter() {
        print!("{item} " )
    }
    println!();
    let pc = point.clone();
    // let mut iter = pc.into_iter();
    looper(&pc);
    //for_looper(&pc);
    println!();
   
    //looper(&ps);

    println!("for_looper displays slice:");
    for_looper(s);
    println!("for_looper displays vector:");
    let t = s.to_vec();
    for_looper(&t);
    println!("for_looper displays VecDeque");
    for_looper(&vecdeq);
    /*------------------------------------------- 
      for_looper can't accept String or &str
      because they do not implement IntoIterator
      https://stackoverflow.com/questions/49962611/why-does-str-not-implement-intoiterator
    */
    println!("ranger displays string:");
    let str = "a literal string".to_string();
    ranger(&mut str.chars());
    println!("ranger displays values in range");
    ranger(&mut (0..10));
    println!("ranger accepts Vector iterator");
    ranger(&mut t.iter());
    println!("ranger accepts VecDeque iterator");
    ranger(&mut vecdeq.iter());

    println!("\nThat's all folks!\n");
}
