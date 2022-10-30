use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

trait Animal: Display {
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);

    fn get_type(&self) -> &str;
    fn set_type(&mut self, a_type: String);
}

impl PartialEq<Self> for dyn Animal {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_type() == other.get_type()
    }
}

impl Eq for dyn Animal {}

impl PartialOrd<Self> for dyn Animal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_type().partial_cmp(&other.get_type())? {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => self.get_name().partial_cmp(&other.get_name()),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

impl Ord for dyn Animal {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.get_name().cmp(&other.get_name()),
            Ordering::Greater => Ordering::Greater
        }
    }
}

#[derive(Eq, PartialEq)]
struct Fish {
    fin_amount: usize,
    name: String,
    a_type: String,
}

impl Display for Fish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. Name: {} Fin Amount: {}", self.a_type, self.name, self.fin_amount)
    }
}

impl Animal for Fish {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn get_type(&self) -> &str {
        &self.a_type
    }

    fn set_type(&mut self, a_type: String) {
        self.a_type = a_type
    }
}

impl Fish {
    pub fn get_fin_amount(&self) -> usize {
        self.fin_amount
    }

    pub fn set_fin_amount(&mut self, fin_amount: usize) {
        self.fin_amount = fin_amount;
    }
    pub fn new(fin_amount: usize, name: String) -> Self {
        Self { fin_amount, name, a_type: String::from("Fish") }
    }
}

#[derive(PartialEq, Eq)]
struct Hamster {
    tail_length: usize,
    color: String,
    name: String,
    a_type: String
}

impl Hamster {
    pub fn tail_length(&self) -> usize {
        self.tail_length
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn set_tail_length(&mut self, tail_length: usize) {
        self.tail_length = tail_length;
    }
    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn new(tail_length: usize, name: String, color: String) -> Self {
        Self { tail_length, color, name, a_type: String::from("Hamster") }
    }
}

impl Display for Hamster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. Name: {} Color: {} Tail length: {}", self.a_type, self.name, self.color, self.tail_length)
    }
}

impl Animal for Hamster {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn get_type(&self) -> &str {
        &self.a_type
    }

    fn set_type(&mut self, a_type: String) {
        self.a_type = a_type
    }
}

fn main() {
    let mut animals = vec![
        Box::new(Fish::new(1, String::from("Nemo"))) as Box<dyn Animal>,
        Box::new(Hamster::new(2, String::from("dog"), String::from("red"))) as Box<dyn Animal>,
        Box::new(Fish::new(4, String::from("FishB"))) as Box<dyn Animal>,
        Box::new(Hamster::new(3, String::from("FishB"), String::from("red"))) as Box<dyn Animal>,
    ];

    animals.sort();

    find_animals(&animals, "FishB").iter().for_each(|animal| println!("{}", animal));

    animals.iter().for_each(|animal| println!("{}", animal))
}

fn find_animals<'a>(animals: &'a Vec<Box<dyn Animal>>, name: &str) -> Vec<&'a Box<dyn Animal>> {
    animals.iter().filter(|animal| animal.get_name() == name).collect()
}
