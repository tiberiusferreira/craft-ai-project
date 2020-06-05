mod person;
use person::*;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap};
use std::fmt;
use serde::{Deserialize};



#[derive(Debug, Default)]
pub struct Lineage {
    people_graph: Vec<Person>,
    people_graph_indexes: HashMap<String, usize>,
}

impl fmt::Display for Lineage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for person in &self.people_graph {
            let sons: Vec<String> = person
                .sons_idx()
                .iter()
                .map(|son_idx| self.get_from_idx(son_idx.clone()).unwrap().name.clone())
                .collect();
            let daughters: Vec<String> = person
                .daughters_idx()
                .iter()
                .map(|son_idx| self.get_from_idx(son_idx.clone()).unwrap().name.clone())
                .collect();
            writeln!(
                f,
                "{name:?} {sex:?} {alive:?} Mother: {mother:?} Father: {father:?} Sons: {sons:?} Daughters: {daughters:?}",
                name = person.name,
                sex = person.sex,
                father = person.father_idx(),
                mother = person.mother_idx(),
                alive = person.alive,
                sons = sons,
                daughters = daughters
            )?;
        }

        fmt::Result::Ok(())
    }
}


impl Lineage{
    pub fn next_in_line(&self, name: &str) -> Option<Person> {
        let sort_and_get_first_alive = |mut input: Vec<&Person>| -> Option<Person>{
            input.sort();
            input.iter().filter(|person| person.alive).next().map(|person| (*person).to_owned())
        };
        let person = self.get_from_name(name)?;
        if let Some(son) = sort_and_get_first_alive(self.get_sons_of(person)){
            return Some(son);
        }
        if let Some(brother) = sort_and_get_first_alive(self.get_brothers(person)){
            return Some(brother);
        }
        if let Some(nephew) = sort_and_get_first_alive(self.get_nephews(person)){
            return Some(nephew);
        }
        if let Some(daughter) = sort_and_get_first_alive(self.get_daughters_of(person)){
            return Some(daughter);
        }
        if let Some(sister) = sort_and_get_first_alive(self.get_sisters(person)){
            return Some(sister);
        }
        if let Some(niece) = sort_and_get_first_alive(self.get_nieces(person)){
            return Some(niece);
        }
        None
    }

}

// Family accessors
impl Lineage {
    pub fn get_sons_of(&self, person: &Person) -> Vec<&Person>{
        self.idx_to_person_vec(&person.sons_idx())
    }

    pub fn get_daughters_of(&self, person: &Person) -> Vec<&Person>{
        self.idx_to_person_vec(&person.daughters_idx())
    }

    pub fn get_mother_of(&self, person: &Person) -> Option<&Person>{
        let mother_idx = person.mother_idx()?;
        // if a person has a mother idx set and the idx is invalid it is a bug
        Some(self.get_from_idx(mother_idx).expect("Invalid mother idx"))
    }

    pub fn get_father_of(&self, person: &Person) -> Option<&Person>{
        let father_idx = person.father_idx()?;
        // if a person has a father idx set and the idx is invalid it is a bug
        Some(self.get_from_idx(father_idx).expect("Invalid father idx"))
    }

    pub fn get_brothers(&self, person: &Person) -> Vec<&Person>{
        let mut brothers: HashSet<&Person> = HashSet::new();
        if let Some(father) = self.get_father_of(person){
            for brother in self.get_sons_of(father){
                brothers.insert(brother);
            }
        }
        if let Some(mother) = self.get_mother_of(person){
            for brother in self.get_sons_of(mother){
                brothers.insert(brother);
            }
        }
        brothers.remove(person);
        brothers.into_iter().collect()
    }

    pub fn get_sisters(&self, person: &Person) -> Vec<&Person>{
        let mut sisters: HashSet<&Person> = HashSet::new();
        if let Some(father) = self.get_father_of(person){
            for sister in self.get_daughters_of(father){
                sisters.insert(sister);
            }
        }
        if let Some(mother) = self.get_mother_of(person){
            for sister in self.get_daughters_of(mother){
                sisters.insert(sister);
            }
        }
        sisters.remove(person);
        sisters.into_iter().collect()
    }

    /// Nephews are sons of the brothers or sisters
    pub fn get_nephews(&self, person: &Person) -> Vec<&Person>{
        let mut brothers_and_sisters = self.get_brothers(person);
        brothers_and_sisters.extend_from_slice(&self.get_sisters(person));
        let mut nephews = vec![];
        for brother_or_sister in brothers_and_sisters{
            nephews.extend_from_slice(&self.get_sons_of(brother_or_sister));
        }
        nephews
    }

    /// Nieces are daughters of the brothers or sisters
    pub fn get_nieces(&self, person: &Person) -> Vec<&Person>{
        let mut brothers_and_sisters = self.get_brothers(person);
        brothers_and_sisters.extend_from_slice(&self.get_sisters(person));
        let mut nieces = vec![];
        for brother_or_sister in brothers_and_sisters{
            nieces.extend_from_slice(&self.get_daughters_of(brother_or_sister));
        }
        nieces
    }
}


impl Lineage {
    pub fn new() -> Self {
        Self::default()
    }

    fn idx_to_person_vec(&self, idx_vec: &Vec<usize>) -> Vec<&Person>{
        idx_vec.iter().map(|idx| &self.people_graph[idx.clone()]).collect()
    }

    fn insert_or_get_existing(&mut self, name: &str, sex: Sex) -> usize {
        return if self.people_graph_indexes.get(name).is_none() {
            // person is not in Lineage yet, insert it in graph
            self.people_graph.push(Person::new(name, sex));
            let index = self.people_graph.len() - 1;
            // update hashmap with index
            self.people_graph_indexes.insert(name.to_string(), index);
            index
        } else {
            self.people_graph_indexes.get(name).unwrap().clone()
        };
    }

    pub fn get_from_name(&self, name: &str) -> Option<&Person> {
        let idx = self.people_graph_indexes.get(name)?;
        self.people_graph.get(idx.clone())
    }

    pub fn get_from_idx(&self, idx: usize) -> Option<&Person> {
        self.people_graph.get(idx)
    }

    pub fn people(&self) -> &Vec<Person> {
        &self.people_graph
    }

    pub fn kill(&mut self, person_name: &str){
        if let Some(person_idx) = self.people_graph_indexes.get(person_name).cloned(){
            self.people_graph[person_idx].kill();
        }
    }

    pub fn insert(&mut self, parent_child_info: ParentChildInfo) {
        let child_idx = self.insert_or_get_existing(
            &parent_child_info.child_name,
            parent_child_info.child_sex.clone(),
        );
        let parent_idx = self.insert_or_get_existing(
            &parent_child_info.parent_name,
            parent_child_info.parent_sex.clone(),
        );
        match &parent_child_info.child_sex {
            Sex::Male => {
                self.people_graph[parent_idx].sons.push(child_idx);
            }
            Sex::Female => {
                self.people_graph[parent_idx].daughters.push(child_idx);
            }
        }
        match &parent_child_info.parent_sex{
            Sex::Male => {
                self.people_graph[child_idx].father = Some(parent_idx);
            },
            Sex::Female => {
                self.people_graph[child_idx].mother = Some(parent_idx);
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ParentChildInfo {
    parent_name: String,
    parent_sex: Sex,
    child_name: String,
    child_sex: Sex,
}
