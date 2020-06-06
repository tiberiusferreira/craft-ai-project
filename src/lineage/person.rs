use serde::Deserialize;
use std::cmp::Ordering;

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
    /// Its own id in the graph, useful for linking a standalone (copy) of a Person struct to the
    /// original struct in the graph
    pub(super) id: usize,
    pub(super) name: String,
    pub(super) house: String,
    pub(super) sex: Sex,
    pub(super) alive: bool,
    pub(super) father: Option<usize>,
    pub(super) mother: Option<usize>,
    pub(super) sons: Vec<usize>,
    pub(super) daughters: Vec<usize>,
}

/// People are ordered alphabetically
impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

/// People are ordered alphabetically
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Person {
    pub(super) fn new(name: &str, sex: Sex, id: usize) -> Self {
        let split_names: Vec<&str> = name.trim().split_ascii_whitespace().collect();
        // Person must have at least 2 names, first name and house name
        assert!(
            split_names.len() >= 2,
            format!("Person must have at least first and house name {}", name)
        );
        let house = split_names.last().unwrap().to_string();
        Person {
            id,
            name: name.to_string(),
            house,
            sex,
            alive: true,
            father: None,
            sons: vec![],
            daughters: vec![],
            mother: None,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sex(&self) -> Sex {
        self.sex.clone()
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    /// In order to kill a person one must go through the "Lineage" struct, not call it here
    /// directly
    pub(super) fn kill(&mut self) {
        self.alive = false;
    }

    pub fn sons_idx(&self) -> &Vec<usize> {
        &self.sons
    }

    pub fn daughters_idx(&self) -> &Vec<usize> {
        &self.daughters
    }

    pub fn father_idx(&self) -> Option<usize> {
        self.father.clone()
    }

    pub fn mother_idx(&self) -> Option<usize> {
        self.mother.clone()
    }
}
