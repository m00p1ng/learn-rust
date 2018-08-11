fn vectors() {
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(4);
  println!("a = {:?}", a);

  match a.get(6) {
    Some(x) => println!("a[6] = {}", x),
    None => println!("error, no such element"),
  }

  let last_elem = a.pop(); // Option
  println!("last elem is {:?}, a = {:?}", last_elem, a);

  while let Some(x) = a.pop() {
    println!("{}", x);
  }
}
