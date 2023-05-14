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
mod stats;
use stats::*;

/*---------------------------------------------------------
  Demo object instances in action
*/
fn main() {
    show_label("demonstrate object creation", 50);

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

    show_label(
      "std::library objects, string and vector<T>", 46
    );
    nl();

    /* #region - to use this feature install #region folding */
    show_op("let mut s1 = \"a string\".to_string()");
    /* #endregion */
    let mut s1 = "a string".to_string();

    /* #region */
    show_op("show_type_scalar(&s1, \"&s1\")");
    /* #endregion */
    show_type_scalar(&s1, "&s1");
    nl();

    /* #region */
    show_op("s1.push_str(\" and more\");");
    /* #endregion */
    s1.push_str(" and more");
    show_type_scalar(&s1, "&s1");
    nl();
    
    /* #region */
    show_op("let mut v:Vec<i32> = vec![1, 2, 3, 2, 1];");
    /* #endregion */
    let mut v: Vec<i32> = vec![1, 2, 3, 2, 1];
    show_type_scalar(&v, "&v");
    nl();
    
    /* #region */
    show_op("v.push(42);");
    /* #endregion */
    v.push(42);
    show_type_scalar(&v, "&v");

    show_label("instances of user-defined types", 35);
    nl();

    /* #region */
    show_op("let mut p1 = Point4D::new()");
    /* #endregion */
    let mut p1 = Point4D::new();
    p1.show("p1");
    p1.set_x(42);
    p1.set_y(-3);
    p1.set_z(2);
    p1.show("p1");
    show_type_scalar(&p1, "p1");
    nl();

    /* #region */
    show_op("let p1a = p1.clone()");
    /* #endregion */
    let p1a = p1.clone();
    p1a.show("p1a.clone()");    
    nl();

    let mut p2 = Point2::<f64>::new(10);
    p2.show("p2", 2, 7);

    /* #region */
    show_op("show_type(p2, \"p2\")");
    /* #endregion */
    show_type(&p2, "p2");

    /* #region */
    show_op("show_value_enum(p2.coors(), \"p2\", 2, 7)");
    /* #endregion */
    show_value_enum(p2.coors(), "p2.coors()", 2, 7);
    nl();
    
    /* #region */
    show_op("p2.coors()[2] = 3.1415");
    /* #endregion */
    p2.coors()[2] = 3.1415;

    /* #region */
    show_op("show_value_enum(p2.coors(), \"p2.coors()\", 2, 7)");
    /* #endregion */
    show_value_enum(p2.coors(), "p2.coors()", 2, 7);
    nl();

    let p2a = 
      Point2::<i32>::new(1).init(
        vec![0, 1, 2, 3, 4, 4, 3, 2, 1, 0]
      );
    p2a.show("p2a", 2, 7);
    nl();

    /* #region */
    show_op("show_type_scalar(p2a, \"p2a\")");
    /* #endregion */
    show_type_scalar(&p2a, "p2a");
    
    println!("p2a.len() = {:?}", p2a.len());
    nl();

    /* #region */
    show_op("p2a.show(\"p2a\", 2, 7)");
    /* #endregion */
    p2a.show("p2a", 2, 7);

    show_label("Testing formats for collections", 35);
    nl();

    let v = 
      vec![
        0i32, -1, 2, -3, 4, -5, 6, -7, 8, -9, 
        10, -11, 12, -13, 14
      ];

    /* #region */
    show_op("show_type_scalar(&v, \"&v\")");
    /* #endregion */
    show_type_scalar(&v, "&v");
    nl();

    /* #region */
    show_op("show_fold(&v, 2, 5)");
    /* #endregion */
    show_fold(&v, 2, 5);
    nl();

    /* #region */
    show_op("let v = vec![0i32, ... ];");
    show_op("fold(&v, 2, 12)");
    /* #endregion */
    let stmp = fold(&v, 2, 12);
    println!("{stmp}");
    nl();

    /* #region */
    show_op("let a = &[f64; 8];");
    /* #endregion */
    let a = 
      &[1.0, 2.5, -3.1, 4.3, 5.0, 6.0, 7.0, -8.0];
    show_op("show_fold(a, 4, 5)");
    show_fold(a, 4, 5);
    
    print!("\n\n  That's all Folks!\n\n");
}
