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
    show_label("demonstrate object creation", 50);

    show_label(
      "std::library objects, string and vector<T>", 46
    );
    nl();

    show_op("let mut s1 = \"a string\".to_string()");
    let mut s1 = "a string".to_string();

    show_op("show_type_scalar(&s1, \"&s1\")");
    show_type_scalar(&s1, "&s1");
    nl();

    show_op("s1.push_str(\" and more\");");
    s1.push_str(" and more");
    show_type_scalar(&s1, "&s1");
    nl();
    
    show_op("let mut v:Vec<i32> = vec![1, 2, 3, 2, 1];");
    let mut v: Vec<i32> = vec![1, 2, 3, 2, 1];
    show_type_scalar(&v, "&v");
    nl();
    
    show_op("v.push(42);");
    v.push(42);
    show_type_scalar(&v, "&v");

    show_label("instances of user-defined type", 35);
    nl();

    show_op("let mut p1 = Point4D::new()");
    let mut p1 = Point4D::new();
    p1.show("p1");
    p1.set_x(42.0);
    p1.set_y(-3.0);
    p1.set_z(2.0);
    p1.show("p1");
    show_type_scalar(&p1, "p1");
    nl();

    show_op("let p1a = p1.clone()");
    let p1a = p1.clone();
    p1a.show("p1a.clone()");    
    nl();

    show_label("Testing formats for collections", 35);
    nl();

    let v = 
      vec![
        0i32, -1, 2, -3, 4, -5, 6, -7, 8, -9, 
        10, -11, 12, -13, 14
      ];

    show_op("show_type_scalar(&v, \"&v\")");
    show_type_scalar(&v, "&v");
    nl();

    show_op("show_fold(&v, 2, 5)");
    show_fold(&v, 2, 5);
    nl();

    show_op("let v = vec![0i32, ... ];");
    show_op("fold(&v, 2, 12)");
    let stmp = fold(&v, 2, 12);
    println!("{stmp}");
    nl();

    show_op("let a = &[f64; 8];");
    let a = 
      &[1.0, 2.5, -3.1, 4.3, 5.0, 6.0, 7.0, -8.0];
    show_op("show_fold(a, 4, 5)");
    show_fold(a, 4, 5);
    
    print!("\n\n  That's all Folks!\n\n");
}
