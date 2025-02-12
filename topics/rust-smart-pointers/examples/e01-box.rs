trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Dog {
    fn speak(&self) {
        println!("Cocorico!");
    }
}

fn main() {
    let dog = Box::new(Dog);
    dog.speak();

    let pet: &dyn Animal = &Dog;
    pet.speak();

    let pet2: &Dog = &Dog;
    pet2.speak();
}
