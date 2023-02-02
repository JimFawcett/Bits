#![allow(dead_code)]

/*-------------------------------------------------------------------
  rust_hello_objects::main.rs
  - Demonstrates creation and use of Rust objects
  - Rust uses struct instead of class to create objects
*/
mod analysis;     // defines scope for analysis and display functions
use analysis::*;  // imports all public contents
mod points;       // defines scope for Point1 and Point2<T>
use points::*;    // imports all public contents

/*-- Demo object instances in action --*/
fn main() {
    print!(
      "\n -- demonstrate object creation --\n"
    );
    show_note("std::library objects");

    print!("\n--- let mut s1 = {}.to_string() ---", "\"a string\"");
    let mut s1 = "a string".to_string();
    print!("\n--- {} ---", "show_type(&s1, \"&s1\")");
    show_type(&s1, "&s1");
    print!("\n--- {} ---", "s1.push_str(\" and more\");");
    s1.push_str(" and more");
    show_type(&s1, "&s1");

    print!("\n--- let mut v:Vec<i32> = vec![1, 2, 3, 2, 1]; ---");
    let mut v:Vec<i32> = vec![1, 2, 3, 2, 1];
    show_type(&v, "&v");
    print!("\n--- v.push(42); ---");
    v.push(42);
    show_type(&v, "&v");    

    show_note("instances of user-defined types");
    
    print!("\n--- let mut p1 = Point1::new() ---\n");
    let mut p1 = Point1::new();
    p1.show();
    println!();
    p1.set_x(42);
    p1.set_y(-3);
    p1.set_z(2);
    p1.show();
    show_type(p1.clone(), "p1");
    print!("\n--- let p1a = p1.clone() ---\n");
    let p1a = p1.clone();
    p1a.show();

    let mut p2 = Point2::<f64>::new(10);
    print!("\n\n--- show_type(p2a.clone(), \"p2a\") ---");
    show_type(p2.clone(), "p2");
    p2.coors()[2] = 3.1415;
    show_type(p2.clone(), "p2");

    let p2a = 
      Point2::<i32>::new(1).init(vec![0,1,2,3,4,4,3,2,1,0]);
    print!("\n--- show_type(p2a.clone(), \"p2a\") ---");
    show_type(p2a.clone(), "p2a");
    print!("\n{}", "\"--- p2a.len() ---\"");
    print!("\nsize of p2a = {:?}", p2a.len());
    print!("\n\n--- p2a.show(8,2) ---\n");
    p2a.show(8,2);
    println!();

    let v = vec![
      0i32, -1, 2, -3, 4, -5, 6, -7, 
      8, -9, 10, -11, 12, -13, 14
    ];
    
    print!("{}", "\n--- show_type(&v, \"&v\") ---");
    show_type(&v, "&v");
    print!("\n--- show_fold(&v, 5, 2) ---");
    show_fold(&v, 5, 2);

    print!("\n\n  That's all Folks!\n\n");
}
