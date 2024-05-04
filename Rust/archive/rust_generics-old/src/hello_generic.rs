use crate::analysis_generic;  // identify module source file
use analysis_generic::*;      // import public functions and types
use std::fmt::*;

/*---------------------------------------------------------
  HelloGeneric: user-defined generic type
  - Not very useful except as a demonstration of how
    to create a generic type.
  - HelloGeneric instances hold a single value of T
  - Generic parameter T is required to implement traits
    Debug   - supports using debug format {:?}, 
    Default - supports using a default value, T:default(),
    Clone   - implements a copy of the physical type
  - Specified traits, like those above, are often called
    bounds because they limit the types that can be used
    as function and method arguments. 
*/
#[derive(Debug, Clone)]  // compiler generated code
pub struct HelloGeneric<T>
  where T: Debug + Default + Clone
{
  datum: T,
}
impl<T> HelloGeneric<T>
  where T: Debug + Default + Clone
{
  /* construct new instance with datum = d */
  pub fn new(d:T) -> Self {
    HelloGeneric::<T> {
      datum: d,
    }
  }
  /* construct new instance with default data value */
  pub fn default_new() -> Self {
    HelloGeneric::<T> {
      datum: T::default(),
    }
  }
  /*  
    As shown, value() is equivalent to making datum public.
    However value method supports adding code to modify the
    return value.
  */
  pub fn value(&mut self) -> &mut T {
    &mut self.datum
  }
  /* print representation of an instance */
  pub fn show(&self) {
    println!("  HelloGeneric {{ {:?} }}", self.datum);
  }
}
/*---------------------------------------------------------
  Demonstrate creation of HelloGeneric type and use
  of its methods.
*/
#[allow(non_snake_case)]
pub fn demo_HelloGeneric() {

  show_label(" demo user defined HelloGeneric type", 40);
  println!();
  show_op("HelloGeneric<T>");
  println!();
  
  show_op("let mut h = HelloGeneric::<i32>::new(42)");
  let mut h = HelloGeneric::<i32>::new(42);
  h.show();
  println!();

  show_op("*h.value() = 84");
  *h.value() = 84;
  h.show();
  println!();

  show_op("let c = h.clone()");
  let c = h.clone();  // h still valid
  c.show();
  println!();

  show_op("let n = h : transfer ownership of datum");
  let n = h;  // move ownership of datum to h
  n.show();
  
  //h.show();  // h invalid, been moved
}
