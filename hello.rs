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


fn learn_vectors() {
  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  println!("vector: {:?}", v );
  println!("vector length: {:?}", v.len() );

  // create vector using `vec!` macro
  let mut v2 = vec![ 200, 300, 100];
  assert_ne!(v, v2);
  v2.sort();
  println!("sorted vector v2: {:?}", v2 );

  let  v3 = vec![1, 2];
  // vectors are compared with each other and slices by value
  assert_eq!(v, v3);

  // immutable clone has same memory
  let v4 = v2.clone();
  assert_eq!(v2, v4);
  assert_eq!(&v2, &v4);

  // mutated clone with same values has same memory
  let mut v5 = v2.clone();
  v5.pop();
  v5.clear();
  v5.push(300);
  v5.push(200);
  v5.push(100);
  v5.sort();
  assert_eq!(v2, v5);
  assert_eq!(&v2, &v5);


  // mutated clone with DIFF values has DIFF memory
  let mut v6 = v2.clone();
  v6.pop();
  v5.push(500);
  assert_ne!(v2, v6);
  assert_ne!(&v2, &v6);


}

fn learn_iterators() {
  let arr = [10, 20, 30];
  println!("Values in array are: ");
  for i in &arr {
    println!("{}", i);
  }

  // In fact, it is more efficient to iterate over an array or slice this way (using iterators)
  // than to use for i in 0..slice.len() {} 
  // because Rust does not have to obsessively check every index operation.

  for i in arr.iter() {
    println!("{}", i);
  }

}

fn main() {
  println!("Hello, World!");
  let answer = 42;
  println!("Hello, {}", answer);
  assert_eq!(answer, 42);

  println!("Learning slices");
  learn_slices();

  println!("Learning Vectors");
  learn_vectors();

  println!("Learning Iterators");
  learn_iterators();
}
