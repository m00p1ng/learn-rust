fn owner() {
  let v = vec![1, 2, 3];

  let print_vector = |x: Vec<i32>| -> Vec<i32> {
    println!("{:?}", x);
    x
  };

  let vv = print_vector(v);
  println!("{}", vv[0]);
}
