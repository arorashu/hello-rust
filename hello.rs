// hello world in Rust


// array2.rs
// read as: slice of i32
fn sum(values: &[i32]) -> i32 {
  let mut res = 0;
  for i in 0..values.len() {
      res += values[i]
  }
  res
}

fn learn_slices() {
  let arr = [10,20,30,40,50];
  
  // println!("{}", arr[5]);

  // look at that &
  println!("Sum of slice");
  let res = sum(&arr);
  println!("sum {}", res);


  println!("Safe operation on slices using `get()` and `Options`");
  let slice = &arr;
  let first = slice.get(0);
  let last = slice.get(5);

  println!("first {:?}", if first.is_some() == true {first.unwrap()} else {&-1} );
  println!("last {:?}", if last.is_some() == true {last.unwrap()} else {&-1});
  println!("last {:?}", last.unwrap_or(&-1));

}


fn main() {
    println!("Hello, World!");
    let answer = 42;
    println!("Hello, {}", answer);
    assert_eq!(answer, 42);

    println!("Learning slices");
    learn_slices();
}

