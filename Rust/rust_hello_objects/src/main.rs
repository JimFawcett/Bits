#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/*-------------------------------------------------------------------
  rust_hello_objects::main.rs
  - Demonstrates creation and use of Rust objects
  - Rust uses struct instead of class to create objects
*/
mod analysis;     // identify module source file
use analysis::*;  // import public functions and types
mod points;       // identify module source file
use points::*;    // import public functions and types

/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {
    print!("\n -- demonstrate object creation --\n");

    show_note("std::library objects, string and vector<T>");

    print!("\n--- let mut s1 = \"a string\".to_string() ---");
    let mut s1 = "a string".to_string();
    print!("\n--- show_type_scalar(&s1, \"&s1\") ---");
    show_type_scalar(&s1, "&s1");
    print!("\n--- s1.push_str(\" and more\"); ---");
    s1.push_str(" and more");
    show_type_scalar(&s1, "&s1");

    print!("\n--- let mut v:Vec<i32> = vec![1, 2, 3, 2, 1]; ---");
    let mut v: Vec<i32> = vec![1, 2, 3, 2, 1];
    show_type_enum(&v, "&v", 2, 10);
    print!("\n--- v.push(42); ---");
    v.push(42);
    show_type_scalar(&v, "&v");

    show_note("instances of user-defined types");

    print!("\n--- let mut p1 = Point1::new() ---\n");
    let mut p1 = Point1::new();
    p1.show();
    println!();
    p1.set_x(42);
    p1.set_y(-3);
    p1.set_z(2);
    p1.show();
    show_type_scalar(&p1, "p1");
    print!("\n--- let p1a = p1.clone() ---\n");
    let p1a = p1.clone();
    p1a.show();

    let mut p2 = Point2::<f64>::new(10);
    print!("\n\n--- show_type(p2a, \"p2a\") ---");
    show_type(&p2, "p2");
    show_op("show_value_enum(p2.coors(), \"p2\", 2, 7)");
    show_value_enum(p2.coors(), "p2.coors()", 2, 7);

    println!();
    show_op("p2.coors()[2] = 3.1415");
    p2.coors()[2] = 3.1415;
    show_op("show_value_enum(p2.coors(), \"p2.coors()\", 2, 7)");
    show_value_enum(p2.coors(), "p2.coors()", 2, 7);

    let p2a = Point2::<i32>::new(1).init(vec![0, 1, 2, 3, 4, 4, 3, 2, 1, 0]);
    print!("\n--- show_type_scalar(p2a, \"p2a\") ---");
    show_type_scalar(&p2a, "p2a");
    
    print!("p2a.len() = {:?}", p2a.len());
    print!("\n\n--- p2a.show(2, 8) ---\n");
    p2a.show(2, 8);
    println!();

    show_note("Testing formats for collections");
    
    let v = 
      vec![
        0i32, -1, 2, -3, 4, -5, 6, -7, 8, -9, 
        10, -11, 12, -13, 14
      ];

    print!("\n--- show_type_scalar(&v, \"&v\") ---");
    show_type_scalar(&v, "&v");
    print!("\n--- show_fold(&v, 5, 2) ---");
    show_fold(&v, 2, 5);

    println!();
    show_op("let v = vec![0i32, ... ];");
    print!("\n--- show_fold(&v, 6, 2) ---");
    let stmp = fold(&v, 2, 12);
    println!("{stmp}");
    println!();

    show_op("let a = &[f64; 8];");
    let a = 
      &[1.0, 2.5, -3.1, 4.3, 5.0, 6.0, 7.0, -8.0];
    show_op("show_fold_enum(a, 4, 5)");
    show_fold(a, 4, 5);
    
    print!("\n\n  That's all Folks!\n\n");
}
