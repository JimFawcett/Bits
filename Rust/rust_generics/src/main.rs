#![allow(dead_code)]
#![allow(clippy::approx_constant)]

/*-------------------------------------------------------------------
  rust_hello_objects::main.rs
  - Demonstrates creation and use of Rust objects
  - Rust uses struct instead of class to create objects
*/
mod analysis;     // identify module source file
use analysis::*;  // import public functions and types
mod points;
use points::*;
mod stats;
use stats::*;

/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {
    show_label("demonstrate generic functions and types", 50);

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

    show_label("instances of user-defined types", 35);
    nl();

    /*-----------------------------------------------------
      Type holding primitive arithmetic types, either
      i32 or f64.  Easy to extend to all integer and float
      types.
    */
    let v = vec![1.0, 2.5, -3.5, 4.0, 5.5];
    let s = Stats::<f64>::new(v);
    println!("s: {s:?}");
    println!("s.max() = {:?}", s.max());
    println!("s.min() = {:?}", s.min());
    println!("s.sum() = {:?}", s.sum());
    println!("s.avg() = {:?}", s.avg());
    println!();
    
    let v: Vec<i32> = vec![1, -2, 3, -4, 5];
    let s: Stats<i32> = Stats::<i32>::new(v);
    println!("s: {s:?}");
    println!("s.max() = {:?}", s.max());
    println!("s.min() = {:?}", s.min());
    println!("s.sum() = {:?}", s.sum());
    println!("s.avg() = {:?}", s.avg());
    println!();

    let mut p1 = PointN::<f64>::new(10);
    p1.show("p1", 2, 7);

    show_op("show_type(p1, \"p1\")");
    show_type(&p1, "p1");

    show_op("show_value_enum(p1.coors(), \"p1\", 2, 7)");
    show_value_enum(p1.coors(), "p1.coors()", 2, 7);
    nl();
    
    show_op("p1.coors()[2] = 3.1415");
    p1.coors()[2] = 3.1415;
    p1.show("p1", 2, 7);

    show_op("show_value_enum(p1.coors(), \"p1.coors()\", 2, 7)");
    show_value_enum(p1.coors(), "p1.coors()", 2, 7);
    nl();

    let p2 = 
      PointN::<i32>::new(1).init(
        vec![0, 1, 2, 3, 4, 4, 3, 2, 1, 0]
      );
    p2.show("p2", 2, 7);
    nl();

    show_op("show_type_scalar(p2, \"p2\")");
    show_type_scalar(&p2, "p2");
    
    println!("p2.len() = {:?}", p2.len());
    nl();

    show_op("p2.show(\"p2\", 2, 7)");
    p2.show("p2", 2, 7);

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
