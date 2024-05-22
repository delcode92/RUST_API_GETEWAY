use std::rc::Rc;

fn main() {
    let name = Rc::new(String::from("John Doe"));
    let another_name = Rc::clone(&name);
  
    println!("Original reference: {}", name);
    println!("Another reference: {}", another_name);
  
    // Reference count shows the number of Rc pointers to the same data
    println!("Reference count: {}", Rc::strong_count(&name)); // Output: 2
}
