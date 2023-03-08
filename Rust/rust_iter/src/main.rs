use std::fmt::*;

/*-----------------------------------------------
  fn looper prints out a comma separated list of
  the Collection c's items.
  - where clause requires C to implement 
    into_iterator(). 
  - 'a annotation says that c and its items last 
    at least as long as the function call.
*/
fn looper<C: Debug, I: Debug>(c:&C) 
    where for<'a> &'a C: IntoIterator<Item = &'a I>
{
    /* convert c into Vec to get len() method */
    let iter = c.into_iter();
    let v:Vec<_> = iter.collect();

    let mut iter = v.iter();
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
fn forlooper<C: Debug, I: Debug>(c:&C) 
    where for<'a> &'a C: IntoIterator<Item = &'a I>
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
/*-----------------------------------------------
  Simplest case - displays known collection
  with generic type
*/
fn whiler<T: Debug>(v:&Vec<T>) {
    let mut i = 0;
    while i < v.len() {
        print!("{:?} ", v[i]);
        i += 1;
    }
    println!();
}
fn main() {
    println!("----------------------------------");
    println!("--- Demonstrate Rust Iteration ---");
    println!("----------------------------------");
    let s = &[1usize, 2, 3, 4, 3, 2, 1];
    println!("slice s = {s:?}");
    println!("s[2] = {:?}", s[2usize]);
    /* works for slice */
    println!("looper displays slice:");
    looper(s);
    /* works for array */
    println!("looper displays array");
    let a = [1, 2, 3];
    looper(&a);
    /* works for Vec too */
    println!("forlooper displays vector:");
    let t = s.to_vec();
    forlooper(&t);

    println!("ranger displays range");
    ranger(&mut (0..10));
    println!("ranger displays results of iteration");
    ranger(&mut t.iter());
    println!("whiler displays Vec<T>");
    whiler(&t);
    println!("\nThat's all folks!\n");
}
