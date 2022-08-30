trait Animal {
    fn make_sound(&self) -> String;
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "meow".to_string()
    }
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "woof".to_string()
    }
}

struct Bird;
impl Animal for Bird {
    fn make_sound(&self) -> String {
        "squawk".to_string()
    }
}

// let dog: Dog = Dog;
// let cat: Cat = Cat;
// let bird: Bird = Bird;
// let mut v: Vec<Box<dyn Animal>> = Vec::new();
// v.push(Box::new(cat));
// v.push(Box::new(dog));
// v.push(Box::new(bird));
// for animal in v.iter() {
//     println!("{}", animal.make_sound());
// }
