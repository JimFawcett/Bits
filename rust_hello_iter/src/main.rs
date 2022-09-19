/*-- Iteration --------------------------------*/
/*
  Rust std::String
  ----------------
  Rust std::String is a container of utf8 chars 
  with no null terminator.
  - utf8 characters may consist of 1 up to 4 
    bytes, so String instances can not be indexed.
    Character boundaries are defined by specific
    byte sequences, used by String's chars() 
    iterator to return characters.
*/

fn string_iteration() {
    let test_string = 
        String::from("a test string");
    /* chars() returns iter over utf8 chars */
    let mut iter = test_string.chars();  
    print!(
        "\n  utf8 characters from {:?}:\n  ", 
        &test_string
    );

    loop {
        /*-- iter.next returns std::Option --*/
        let opt = iter.next();
        if opt == None {
            break;
        }
        /*-- unwrap char from Some(char) --*/
        print!("{} ",opt.unwrap()); 
    }

    let ls = test_string.as_str();
    print!("\n  test_string: {:?}", ls);
    println!();
}

fn idomatic_string_iteration() {
    let test_string = 
      String::from("another test string");
    print!(
      "\n  idiomatic utf8 chars from {:?}:\n  ", 
      &test_string
    );

    for ch in test_string.chars() {
        /*-- option handling implicit here --*/
        print!("{} ",ch);
    }
    /*-- nth(i) returns Option --*/
    let i = 1;
    let rslt = &test_string.chars().nth(i);
    if let Some(ch) = rslt {
      print!(
          "\n  at index {} char of {:?} is {}",
          i, test_string, ch
      );
    }
    else {
      print!("\n  index {} out of range", i);
    }
    let ls = test_string.as_str();
    print!("\n  test_string: {:?}", ls);
    println!();
}
/*-----------------------------------------------
  demonstrate chars(), is_alphabetic, is_..., 
  for_each, filter, and collect

  There are many iterator adapters.  These are
  some of the most often used.
*/
fn string_adapters() {
    let ls = "abc123";

    /*-- are all chars alphabetic --*/
    print!(
        "\n  {:?} is alphabetic   {}", ls,  
        ls.chars().all(|c| {c.is_alphabetic()})
    );

    /*-- are all chars alphanumeric? --*/
    print!(
        "\n  {:?} is alphanumeric {}", ls, 
        ls.chars().all(|c| {c.is_alphanumeric()})
    );

    /*-- are all chars numeric? --*/
    print!(
        "\n  {:?} is numeric      {}", ls, 
        ls.chars().all(|c| {c.is_numeric()})
    );

    /*-- are all chars ascii? --*/
    print!(
        "\n  {:?} is ascii        {}", ls, 
        ls.chars().all(|c| {c.is_ascii()})
    );
    
    /*-- display chars from str slice --*/
    let (min, max) = (2usize, 4usize);
    if min <= 
          ls.len() && 
          max <= ls.len() && 
          min <= max {
        let slice = &ls[min..max];
        print!(
          "\n  3rd and 4th chars of {:?} are: ", 
          ls
        );
        slice.chars()
             .for_each(|c| print!("{}", c));
    }
    else {
        print!(
          "\n  invalid {} and {} for {}",
          min, max, ls
        )
    }

    /*-- from numeric chars in source, ls --*/
    print!(
        "\n  numeric chars of {:?} are: {:?}", ls,
        ls.chars()
          .filter(|c| c.is_numeric())
          .collect::<String>()
    );
    println!();
}
/*
  Rust byte arrays
  ----------------
  Rust arrays have sizes that must be determined at 
  compile-time, even those created on the heap.

  Rust Vectors have sizes that can be determined at 
  run-time, and they will readily give access to 
  their internal heap-based arrays by taking slices.

  This is perfectly data-safe, because:
  - slices have a len() function
  - even if you index past the end of the array, 
    you can't read or write that memory, because a 
    panic occurs immediately.
*/
fn define_and_iterate_byte_array() {
    let ba: [u8;5] = [1,2,3,4,5];  
    // size is determined at compile-time, even for
    // arrays created on the heap (unlike C++)
    let max = ba.len();
    print!("\n  bytes from byte array:\n  [");
    /*-- display all but last --*/
    for i in 0..max-1 {
        print!("{}, ", ba[i]);
    }
    /*-- display last char --*/
    print!("{}]", ba[max-1]);
}

fn idiomatic_define_and_iterate_byte_array() {
    let v: Vec<u8> = vec![5,4,3,2,1];
    let ba: &[u8] = &v[..];  
    /*-------------------------------------------
      type of slice of Vector is 
      byte slice: &[u8]
      - slices implement len() function
      - &v[..] slice of all elements of v
      - &v[m..n] slice of elements m up to,
        but not including n 
      - Length of slice &v[..] determined by 
        length of v, which can be determined at 
        run-time.
    */
    print!("\n  idiomatic bytes from byte array:");
    print!(
      "\n  length of byte slice: {}", 
      ba.len()
    );
    let max = ba.len();
    /*-- print all but the last --*/
    print!("\n  [");
    for item in ba.iter().take(max-1) {
        print!("{}, ", item);
    }
    /*-- print last one without trailing comma --*/
    print!("{}]", ba[max - 1]);

    print!(
      "\n  printing array with implicit iteration:"
    );
    print!("\n  {:?}", ba);
}

fn main() {
    print!("\n  -- demonstrate iteration --\n");

    print!("\n  -- string iteration --");
    string_iteration();
    idomatic_string_iteration();
    print!("\n  -- string iteration adapters --");
    string_adapters();
    print!("\n\n  -- byte array iteration --");
    define_and_iterate_byte_array();
    idiomatic_define_and_iterate_byte_array();

    print!("\n\n  That's all Folks!\n");
}
