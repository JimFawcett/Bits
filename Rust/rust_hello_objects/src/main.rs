use std::fmt::*;
use std::default::*;

// CreateObject::main.rs

/*-------------------------------------
  - Declare Point1 struct, like a C++ class
  - Request compiler implement traits Debug & Clone
*/
#[derive(Debug, Clone)]
pub struct Point1 {
    x:i32,
    y:i32,
    z:i32
}
/*-- implement functions new and name --*/
impl Point1 {
  pub fn new() 
        -> Point1 {
      Point1 {
          x:0, y:0, z:0
      }
  }
  /* 
    For simple types like this it would be reasonable to 
    make x, y, z public and remove the getter and setter
    functions. 
  */
  pub fn get_x(&self) -> &i32 {
    &self.x
  }
  pub fn get_y(&self) -> &i32 {
    &self.y
  }
  pub fn get_z(&self) -> &i32 {
    &self.z
  }
  pub fn set_x(&mut self, x:i32) {
    self.x = x
  }
  pub fn set_y(&mut self, y:i32) {
    self.y = y
  }
  pub fn set_z(&mut self, z:i32) {
    self.z = z
  }
  pub fn show(&self) {
    print!("Point1 {{ ");
    print!("{0}, ", self.x);
    print!("{0}, ", self.y);
    print!("{0} }}", self.z);
  }
}

/*-------------------------------------
  - Declare Point2<T> struct, like a C++ template class
  - Request compiler implement traits Debug & Clone
*/
#[derive(Debug, Clone)]
pub struct Point2<T> where T:Debug, T:Default, T:Clone {
  coor: Vec<T>,
}
impl<T> Point2<T> where T:Debug, T:Default, T:Clone {
  pub fn new(n:usize) 
        -> Point2<T> {
      Point2::<T> {
          coor : vec![T::default(); n],
      }
  }
  pub fn init(mut self, coord: Vec<T>) -> Point2<T> {
    self.coor = coord;
    self
  }
  pub fn len(&self) -> usize {
    self.coor.len()
  }
  /* acts as both get_coor() and set_coor(vec![1, 2, 3]) */
  pub fn coors(&mut self) -> &mut Vec<T> {
    return &mut self.coor;
  }
  pub fn show(&self, width:usize, left:usize) {
    print!("Point2<T> {{ ");
    show_fold(&self.coor, width, left);
    print!("}}")
  }
}
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

    let p2a = Point2::<i32>::new(1).init(vec![0,1,2,3,4,4,3,2,1,0]);
    print!("\n--- show_type(p2a.clone(), \"p2a\") ---");
    show_type(p2a.clone(), "p2a");
    print!("\n{}", "\"--- p2a.len() ---\"");
    print!("\nsize of p2a = {:?}", p2a.len());
    print!("\n\n--- p2a.show(8,2) ---\n");
    p2a.show(8,2);
    println!();

    let v = vec![
      0i32, -1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12, -13, 14
    ];
    
    print!("{}", "\n--- show_type(&v, \"&v\") ---");
    show_type(&v, "&v");
    print!("\n--- show_fold(&v, 5, 2) ---");
    show_fold(&v, 5, 2);

    print!("\n\n  That's all Folks!\n\n");
}
/*
  Show facts about a type's elements, e.g., name, type, 
  value, and size.
  show_type is generic function with Debug bound.
  Using format "{:?} requires Debug."
*/
fn show_type<T: std::fmt::Debug>(t: T, nm: &str) {
  let typename = std::any::type_name::<T>();
  println!("\ncall name: {:?}, type: {:?}", nm, typename);
  println!("value: {:?}, size: {}", t, std::mem::size_of::<T>());
}
/* indent print out by "left" spaces */
fn print_offset(left:usize) {
  for _i in 0..left {
    print!(" ");
  }
}
/* fold a vector slice into rows of w elements */
fn show_fold<T:Debug>(d:&Vec<T>, w:usize, left:usize) {
  print!("\n");
  print_offset(left);
  for _i in 1..=d.len() {
    if _i < d.len() {
      print!("{:?}, ", d[_i - 1]);
    }
    else {
      print!("{:?}", d[_i - 1]);
      print!("\n");
      break;
    }
    if _i % w == 0  && _i != 0 && _i != w - 1 {
      print!("\n");
      print_offset(left);
    }
  }
}
/* show string wrapped with dotted lines above and below */
fn show_note(note:&str) {
  print!("\n-------------------------\n");
  print!("{}", note);
  print!("\n-------------------------\n");
}

