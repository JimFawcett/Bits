#![allow(unused_mut)]
#![allow(clippy::approx_constant)]

/* rust_hello_data::main.rs */
/*
  Static Data Types:
    bool, char
    i8, i16, i32, i64, i128, isize
    u8, u16, u32, u64, u128, usize
    f32, f64
    (), reference
    array, tuple, struct, enum
  Qualifiers:
    mutable, const

  Library Types:
    std::String, std::Vec<T>, std::HashMap<K,V>,
    std::VecDeque<T>, std::BTreeSet<T>, std::BTreeMap<K,V>,
    std::LinkedList<T>, std::BinaryHeap<T>

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

      A move type can be cloned, but that requires an explicit 
      call to clone(). Otherwise, assignment and pass by value
      result in move which transfers ownership of heap 
      resources and invalidates the moved instance.

      Any attempt to use a moved instance results in compile
      failure.
    */

    /*-- thesee values live in stack frame --*/
    println!("\n  -- initialize on stack from literals --");
    let t1: i32 = 42;
    show_type(t1, "t1");
    let t1a = 42i64;    // alternate declaration style
    show_type(t1a, "t1a");
    /*
      Below is not type redefinition, it is shadowing,
      e.g., a new variable that hides the older t1a.
    */
    let mut t1a: f64 = 42.0;  // will change value later
    show_type(t1a, "t1a");

    /*-- create mutable reference --*/
    let t1b: &mut f64 = &mut t1a;  // requires t1a to be mutable
    *t1b = 84.0;
    show_type(t1b, "t1b");

    /*-- values live in heap when wrapped with box --*/
    println!("\n  -- initialize on heap --");
    // 3.14159f64 below is a value specified to be f64
    let t2 = Box::new(3.14159f64); 
    show_type(*t2, "*t2");
    show_type(t2, "t2");    // moves t2, use t2.clone() to avoid move
    // print!("{}", &t2.to_string());  // error - t2 moved
    /*-- Box releases its heap storage when it goes out of scope --*/

    println!("\n  -- size_of measures size of handle to heap element --");
    let s1 = std::mem::size_of::<Box<i8>>();
    let s2 = std::mem::size_of::<Box<i64>>();
    println!("\nsize_of::<Box<i8>>() = {}", s1);
    println!("size_of::<Box<i64>>() = {}", s2);

    print!("\n  -- String is collection of utf8 chars --\n");
    /*--
      Each utf8 character may take from 1 to 4 bytes. The first
      byte in a char sequence has bits that describe the size of
      the character. utf8 can represent many more characters than
      ASCII, so Rust strings can contain SanSkrit, Kangi, Arabic,
      and emojis, as well as English.

      String control block lives on stack, char data live in heap.
      Passing by value or assignment transfers ownership from
      source to destination.  That is a move.
    --*/
    let t3: String = String::from("a string"); // temp moved
    show_type(&t3, "t3");

    println!("\n  -- use clone to avoid src move --");
    show_type(t3.clone(), "t3.clone()"); // moves t3s clone, not t3

    let t4 = &t3; // reference to still valid t3
    show_type(t4, "t4");

    println!("\n  -- array of copy type elements is copy --");
    let t5 = [1, 2, 3, 4, 5]; // copy type
    show_type(t5, "t5");
    let mut txt = String::from("\nt5 was copied into show_type(...)");
    txt += ", see:\nt5 = ";
    println!("{}{:?} is still valid", txt, t5);

    println!("\n  -- vec is like an extendable array --");
    let mut v1 = Vec::<i32>::new();
    v1.push(1);
    v1.push(-1);
    v1.insert(1, 4);
    show_type(v1, "\"v1\"");  // v1 moved

    let v2 = vec![42, 84, 126];
    show_type(&v2, "&v2");    // v2 not moved, passed reference
                                    // size of &v2 is size of a pointer
    println!("\n\nThat's all Folks!!\n\n");
}
/*
  show_type is generic function with Debug bound.
  Using format "{:?}" requires Debug.
  - mem::size_of::<T> measures size of T in stackframe.
    It does not measure size of resources on heap.
*/
fn show_type<T: Debug>(t: T, nm: &str) {
  let typename = std::any::type_name::<T>();
  println!("\n{}, {}", nm, typename);
  println!(
    "value: {:?}, size: {}", 
    t, std::mem::size_of::<T>()
  );
}

