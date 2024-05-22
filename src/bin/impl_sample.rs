
#[derive(Debug)]
struct Animal{
    age: u8,
    animal_type: AnimalType 
}

struct Animal2{
    age: u8,
    animal_type: AnimalType 
} 

#[derive(Debug)]
enum AnimalType{
    Cat,
    Dog,
}

impl Animal {
    
    fn new() -> Animal{
        Animal {
            age: 8,
            animal_type: AnimalType::Cat
        }
    }

    fn change_to_dog(&mut self){
        println!("changing animal to dog");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self){
        println!("changing animal to cat");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self){
        match self.animal_type{
            AnimalType::Dog => println!("the animal is a dog"),
            AnimalType::Cat => println!("the animal is a dog"),
        }
    }


}

fn main(){
    let mut new_animal = Animal::new();

    new_animal.check_type();
    new_animal.change_to_dog();
    
    new_animal.check_type();
    new_animal.change_to_cat();
    
    new_animal.check_type();


}