// why must using Rc, why not just like below, just clone()
fn main() {
    let name = String::from("John Doe");
    let another_name = name.clone();
  
    println!("Another reference: {}", name);
    println!("Another reference: {}", another_name);
  }
  