#![allow(unused_mut)]
/* rust_hello_data::main.rs */
/*
  Static Data Types:
    bool, char
    i8, i16, i32, i64, i128, isize
    u8, u16, u32, u64, u128, usize
    f32, f64
    (), reference
    array, tuple
    struct, enum, use
  Qualifiers:
    mutable, const

  Operations:
    Primitive types can all be copied.
    Most library and user-defined types can be moved,
    cloned, but only aggregates with all copyable members
    can be copied.
      
  Processing:
    All types are static, operations run as native code,
    and no garbage collection is needed. Resources are
    returned at end of declr scope.
*/
use std::fmt::Debug;
/*  
  First argument is passed by value, second passed
  by reference. Non-copy types supplied to the first
  argument will be moved.
*/
fn show<T: Debug>(t:T, n:&str) {
  let typename = std::any::type_name::<T>();
  println!("\n{}, {}",typename, n);
  println!("value: {:?}, size: {}", t, std::mem::size_of::<T>());
}

fn main() {
    print!(
      "\n Demonstrate Rust types\n\
       ------------------------"
    );
    /*
      Primitive data types: i32, f64, ... occupy contiguous
      regions of memory, so they satisfy the copy trait.

      Library types like String have a control block in stack
      and data in heap. So they do not satisfy the copy trait.
      They can be cloned, but that requires an explicit call
      to clone().
    */

    /*-- values live in stack frame --*/
    let mut t1 :i32 = 42;
    show(t1, stringify!(t1));
    let t1a:i64 = 42;
    show(t1a, stringify!(t1a));
    /* 
      Below is not type redefinition, it is shadowing, 
      e.g., a new variable that hides the older t1a.
    */
    let t1a:f64 = 42.0;
    show(t1a, stringify!(t1a));

    /*-- values live in heap when wrapped with box --*/
    let t2 = Box::new(3.14159f64);
    show(t2.clone(), stringify!(t2));
    show(*t2, stringify!(*t2));

    let s1 = std::mem::size_of::<Box<i8>>();
    let s2 = std::mem::size_of::<Box<i64>>();
    println!("\nsize_of::<Box<i8>>() = {}", s1);
    println!("size_of::<Box<i64>>() = {}", s2);

    /*-- 
      Control block lives on stack, char data live in heap.
      Passing by value or assignment transfers ownership
      from source to destination.  That is termed a move.
    --*/
    let t3:String = String::from("Hello Data");  // temp moved
    show(t3.clone(), stringify!(t3.clone()));    // moves t3s clone

    let t4 = &t3;  // reference to still valid t3
    show(t4, stringify!(t4));

    let t5 = [1, 2, 3, 4, 5];  // copy type
    show(t5, stringify!(t5));
    let mut txt = String::from("\nt5 was copied into show(...)");
    txt += ", see:\nt5 = ";
    println!("{}{:?} is still valid", txt, t5);

    println!("\n\nThat's all Folks!!\n\n");
}
