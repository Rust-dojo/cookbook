// Juntar dois valores de texto
fn main(){
  let first_name = "Satoshi";
  let last_name = "Nakamoto";

  // using concat
  let full_name = [first_name, " ", last_name].concat();
  
  // using join
  //let full_name = [first_name, last_name].join(" ");

  println!("{full_name}");
}
