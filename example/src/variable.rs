fn variable() {
  let a: u8 = 123;
  println!("a = {}", a);

  let mut b: i8 = 0;
  println!("b = {}", b);

  b = 1;
  println!("b = {}", b);

  let mut c = 123456789;
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

  c = -1;
  println!("c = {} after modification", c);

  let z: isize = 123;
  let size_of_z = mem::size_of_val(&z);
  println!(
    "z = {}, takes up {} bytes, {}-bit os",
    z,
    size_of_z,
    size_of_z * 8
  );

  let d = 'x';
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

  let e: f64 = 2.5;
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  let g = false;
  println!("g = {}", g);
}
