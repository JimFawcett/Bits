#![allow(unused_mut)]
#![allow(clippy::approx_constant)]

use std::fmt::Debug;
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
/*
  First argument is passed by value, second passed
  by reference. Non-copy types supplied to the first
  argument will be moved.
*/
fn main() {
    println!(
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
    println!("\n  -- initialize on stack from literals --");
    let mut t1: i32 = 42;
    show_type(t1, "t1");
    let t1a = 42i64;      // alternate declaration
    show_type(t1a, "t1a");
    /*
      Below is not type redefinition, it is shadowing,
      e.g., a new variable that hides the older t1a.
    */
    let mut t1a: f64 = 42.0;
    show_type(t1a, "t1a");

    /*-- create mutable reference --*/
    let t1b: &mut f64 = &mut t1a;  // requires t1a to be mutable
    *t1b = 84.0;
    show_type(t1b, "t1b");

    /*-- values live in heap when wrapped with box --*/
    println!("\n  -- initialize on heap --");
    let t2 = Box::new(3.14159f64); // 3.14159f64 is value specified to be f64
    show_type(*t2, "*t2");
    show_type(t2, "t2");    // moves t2, use t2.clone() to avoid move
    // print!("{}", &t2.to_string());  // error - t2 moved
    /*-- Box releases its heap storage when it goes out of scope --*/

    println!("\n  -- size_of measures size of handle to heap element --");
    let s1 = std::mem::size_of::<Box<i8>>();
    let s2 = std::mem::size_of::<Box<i64>>();
    println!("\nsize_of::<Box<i8>>() = {}", s1);
    println!("size_of::<Box<i64>>() = {}", s2);

    /*--
      String control block lives on stack, char data live in heap.
      Passing by value or assignment transfers ownership from
      source to destination.  That is termed a move.
    --*/
    println!("\n  -- use clone to avoid src move --");
    let t3: String = String::from("Hello Data"); // temp moved
    show_type(t3.clone(), "t3.clone()"); // moves t3s clone, not t3

    let t4 = &t3; // reference to still valid t3
    show_type(t4, "t4");

    println!("\n  -- array of copy type elements is copy --");
    let t5 = [1, 2, 3, 4, 5]; // copy type
    show_type(t5, "t5");
    let mut txt = String::from("\nt5 was copied into show_type(...)");
    txt += ", see:\nt5 = ";
    println!("{}{:?} is still valid", txt, t5);

    println!("\n\nThat's all Folks!!\n\n");
}
/*
  show_type is generic function with Debug bound.
  Using format "{:?} requires Debug."
*/
fn show_type<T: Debug>(t: T, nm: &str) {
  let typename = std::any::type_name::<T>();
  println!("\n{}, {}", nm, typename);
  println!("value: {:?}, size: {}", t, std::mem::size_of::<T>());
}

