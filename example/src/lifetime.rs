struct Person {
  name: String,
}

struct Company<'z> {
  name: String,
  ceo: &'z Person,
}

impl Person {
  // fn get_ref_name(&self) -> &String {
  fn get_ref_name<'a>(&'a self) -> &'a String {
    &self.name
  }
}

fn lifetime() {
  let mut z: &String;

  {
    let p = Person {
      name: String::from("John"),
    };
    z = p.get_ref_name()();
  }
}
