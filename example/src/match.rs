fn match_statement() {
  let country_code = 4;

  let country = match country_code {
    5 => "UK",
    6 => "Thailand",
    7 => "Korea",
    1...999 => "Unknown",
    _ => "Invalid",
  };

  println!("Country {}", country)
}
