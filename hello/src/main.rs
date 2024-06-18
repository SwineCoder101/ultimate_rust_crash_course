use hello::greet;
use std::collections::HashMap;

struct RedFox {
    name: String,
    sound: String,
}

trait Noisy {
    fn get_noise(&self) -> &str;
}


impl Noisy for RedFox {
    fn get_noise(&self) -> &str{
        &self.sound
    }
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {"BYTE!"}
}

fn print_noise<T:Noisy>(item: T) {
    println!("{}", item.get_noise());
}





impl RedFox {
    fn new () -> Self {
        Self{
            name: "Red Fox".to_string(),
            sound: "Ring-ding-ding-ding-dingeringeding!".to_string(),
        }
    }

    fn copy_of (&self) -> Self {
        Self {
            //concatenating copy of with the name
            name: self.name.clone() + " the copy",
            sound: self.sound.clone(),
        }
    }  

    fn new_of (name: String, sound: String) -> Self {
        Self { name, sound }
    }

    fn sound(&self) -> &str {
        &self.sound
    }

    fn move_in(&self){
        println!("The {} moves quickly.",  self.name);
    }

    fn greet(&self) {
        println!("What does the {} say? {}", self.name, self.sound);
    }
}


enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8},
}

enum Option<T> {
    Some(T),
    None,
}

fn check_option(opt: Option<u8>) {
    if let Some(x) = opt {
        println!("{}",x);
    } else {
        println!("None");
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(2);

    let mut v = vec![1, 2, 3, 4, 5];


    let mut h: HashMap<u8,bool> = HashMap::new();
    h.insert(1, true);
    h.insert(2, false);

    let have_five = h.remove(&5).unwrap();


    greet();
    let fox = RedFox::new();
    let running_fox: RedFox = RedFox:: new_of("bilbo".to_string(), "they are taking the hobbits to isengard!!".to_string());
    let copy_fox = fox.copy_of();
    let mut chamelion = copy_fox;

    fox.greet();
    running_fox.greet();
    running_fox.move_in();

    print_noise(5_u8);

}
