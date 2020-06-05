use std::cmp::Ordering;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum Sex {
    #[serde(rename(deserialize = "M"))]
    Male,
    #[serde(rename(deserialize = "F"))]
    Female,
}

/// Represents a person of the family. Fields are private outside super to help avoid the creation of
/// an invalid person (setting father to an invalid id for example)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Person {
    pub (in super) name: String,
    pub (in super) sex: Sex,
    pub (in super) alive: bool,
    pub (in super) father: Option<usize>,
    pub (in super) mother: Option<usize>,
    pub (in super) sons: Vec<usize>,
    pub (in super) daughters: Vec<usize>,
}

impl Ord for Person{
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Person{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}


impl Person {
    pub fn new(name: &str, sex: Sex) -> Self {
        Person {
            name: name.to_string(),
            sex,
            alive: true,
            father: None,
            sons: vec![],
            daughters: vec![],
            mother: None
        }
    }

    pub fn name(&self) -> &str{
        &self.name
    }

    pub fn sex(&self) -> Sex{
        self.sex.clone()
    }

    pub fn alive(&self) -> bool{
        self.alive
    }

    pub fn kill(&mut self){
        self.alive = false;
    }

    pub fn sons_idx(&self) -> &Vec<usize>{
        &self.sons
    }

    pub fn daughters_idx(&self) -> &Vec<usize>{
        &self.daughters
    }

    pub fn father_idx(&self) -> Option<usize>{
        self.father.clone()
    }

    pub fn mother_idx(&self) -> Option<usize>{
        self.mother.clone()
    }
}
