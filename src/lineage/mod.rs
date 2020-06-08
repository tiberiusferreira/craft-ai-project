mod person;
pub use person::{Person, Sex};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub enum KillError {
    PersonNotFound,
    PersonAlreadyDead,
}

/// Struct containing a parent child family relationship. It is consumed by Lineage in order to
/// construct its family graph
#[derive(Debug, Deserialize)]
pub struct ParentChildInfo {
    pub parent_name: String,
    pub parent_sex: Sex,
    pub child_name: String,
    pub child_sex: Sex,
}

impl ParentChildInfo {
    pub fn new<S: Into<String>>(
        parent_name: S,
        parent_sex: Sex,
        child_name: S,
        child_sex: Sex,
    ) -> Self {
        ParentChildInfo {
            parent_name: parent_name.into(),
            parent_sex,
            child_name: child_name.into(),
            child_sex,
        }
    }
}

/// Represents a family lineage, contains the information of who is alive and the relationship
/// between its members.
/// `people_graph` contains the information itself and `people_graph_indexes` provides a way
/// to translate a person's name to its index in `people_graph`
#[derive(Debug, Default)]
pub struct Lineage {
    people_graph: Vec<Person>,
    people_graph_indexes: HashMap<String, usize>,
}

impl Lineage {
    pub fn next_in_line(&self, name: &str) -> Option<Person> {
        let sort_and_get_first_alive = |mut input: Vec<&Person>| -> Option<Person> {
            input.sort();
            input
                .iter()
                .find(|person| person.alive)
                .map(|person| (*person).to_owned())
        };
        let queried_person = self.get_from_name(name)?;
        if let Some(son) = sort_and_get_first_alive(self.get_sons_of(queried_person)) {
            return Some(son);
        }
        if let Some(brother) = sort_and_get_first_alive(self.get_brothers(queried_person)) {
            return Some(brother);
        }
        if let Some(nephew) = sort_and_get_first_alive(self.get_nephews(queried_person)) {
            return Some(nephew);
        }
        if let Some(daughter) = sort_and_get_first_alive(self.get_daughters_of(queried_person)) {
            return Some(daughter);
        }
        if let Some(sister) = sort_and_get_first_alive(self.get_sisters(queried_person)) {
            return Some(sister);
        }
        if let Some(niece) = sort_and_get_first_alive(self.get_nieces(queried_person)) {
            return Some(niece);
        }

        let mut alive_people_from_house: Vec<&Person> = self
            .people()
            .iter()
            .filter(|each_person| {
                (each_person.house == queried_person.house)
                    && each_person.alive()
                    && (each_person.name != queried_person.name)
            })
            .collect();
        alive_people_from_house.sort();

        alive_people_from_house.first().cloned().cloned()
    }
}

// Family accessors
impl Lineage {
    pub fn get_sons_of(&self, person: &Person) -> Vec<&Person> {
        self.idx_to_person_vec(&person.sons_idx())
    }

    pub fn get_daughters_of(&self, person: &Person) -> Vec<&Person> {
        self.idx_to_person_vec(&person.daughters_idx())
    }

    pub fn get_mother_of(&self, person: &Person) -> Option<&Person> {
        let mother_idx = person.mother_idx()?;
        // if a person has a mother idx set and the idx is invalid it is a bug
        Some(self.get_from_idx(mother_idx).expect("Invalid mother idx"))
    }

    pub fn get_father_of(&self, person: &Person) -> Option<&Person> {
        let father_idx = person.father_idx()?;
        // if a person has a father idx set and the idx is invalid it is a bug
        Some(self.get_from_idx(father_idx).expect("Invalid father idx"))
    }

    pub fn get_brothers(&self, person: &Person) -> Vec<&Person> {
        let mut brothers: HashSet<&Person> = HashSet::new();
        if let Some(father) = self.get_father_of(person) {
            for brother in self.get_sons_of(father) {
                brothers.insert(brother);
            }
        }
        if let Some(mother) = self.get_mother_of(person) {
            for brother in self.get_sons_of(mother) {
                brothers.insert(brother);
            }
        }
        brothers.remove(person);
        brothers.into_iter().collect()
    }

    pub fn get_sisters(&self, person: &Person) -> Vec<&Person> {
        let mut sisters: HashSet<&Person> = HashSet::new();
        if let Some(father) = self.get_father_of(person) {
            for sister in self.get_daughters_of(father) {
                sisters.insert(sister);
            }
        }
        if let Some(mother) = self.get_mother_of(person) {
            for sister in self.get_daughters_of(mother) {
                sisters.insert(sister);
            }
        }
        sisters.remove(person);
        sisters.into_iter().collect()
    }

    /// Nephews are sons of the brothers or sisters
    pub fn get_nephews(&self, person: &Person) -> Vec<&Person> {
        let mut brothers_and_sisters = self.get_brothers(person);
        brothers_and_sisters.extend_from_slice(&self.get_sisters(person));
        let mut nephews = vec![];
        for brother_or_sister in brothers_and_sisters {
            nephews.extend_from_slice(&self.get_sons_of(brother_or_sister));
        }
        nephews
    }

    /// Nieces are daughters of the brothers or sisters
    pub fn get_nieces(&self, person: &Person) -> Vec<&Person> {
        let mut brothers_and_sisters = self.get_brothers(person);
        brothers_and_sisters.extend_from_slice(&self.get_sisters(person));
        let mut nieces = vec![];
        for brother_or_sister in brothers_and_sisters {
            nieces.extend_from_slice(&self.get_daughters_of(brother_or_sister));
        }
        nieces
    }

    pub fn to_graphviz(&self) -> String {
        let people = self.people();
        let mut deps = Graph::<String, &str>::new();
        let mut person_index_map: HashMap<usize, NodeIndex> = HashMap::new();
        for (person_idx, person) in people.iter().enumerate() {
            let idx = deps.add_node(format!("{} ({:?})", person.name(), person.sex()));
            person_index_map.insert(person_idx, idx);
        }

        for (person_idx, person) in people.iter().enumerate() {
            let person_node = person_index_map.get(&person_idx).unwrap();
            for child in person.sons_idx() {
                let son_node = person_index_map.get(&child).unwrap();
                deps.add_edge(person_node.clone(), son_node.clone(), Default::default());
            }

            for child in person.daughters_idx() {
                let son_node = person_index_map.get(&child).unwrap();
                deps.add_edge(person_node.clone(), son_node.clone(), Default::default());
            }
        }

        let graphviz = Dot::with_config(&deps, &[Config::EdgeNoLabel]);
        graphviz.to_string()
    }
}

impl Lineage {
    pub fn new() -> Self {
        Self::default()
    }

    fn idx_to_person_vec(&self, idx_vec: &[usize]) -> Vec<&Person> {
        idx_vec.iter().map(|idx| &self.people_graph[*idx]).collect()
    }

    fn insert_or_get_existing(&mut self, name: &str, sex: Sex) -> usize {
        if self.people_graph_indexes.get(name).is_none() {
            // person is not in Lineage yet, insert it in graph
            let index = self.people_graph.len();
            self.people_graph.push(Person::new(name, sex, index));
            // update hashmap with index
            self.people_graph_indexes.insert(name.to_string(), index);
            index
        } else {
            *self.people_graph_indexes.get(name).unwrap()
        }
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

    /// Returns true if the person was found and killed, false if the person did not exist
    pub fn kill(&mut self, person_name: &str) -> Result<(), KillError> {
        if let Some(person_idx) = self.people_graph_indexes.get(person_name).cloned() {
            if !self.people_graph[person_idx].alive {
                return Err(KillError::PersonAlreadyDead);
            }
            self.people_graph[person_idx].kill();
            Ok(())
        } else {
            Err(KillError::PersonNotFound)
        }
    }

    pub fn insert(&mut self, parent_child_info: ParentChildInfo) {
        // insert or get the existing index of the parent and child in the graph
        let child_idx = self.insert_or_get_existing(
            &parent_child_info.child_name,
            parent_child_info.child_sex.clone(),
        );
        let parent_idx = self.insert_or_get_existing(
            &parent_child_info.parent_name,
            parent_child_info.parent_sex.clone(),
        );
        // update parent info about son or daughter
        match &parent_child_info.child_sex {
            Sex::Male => {
                self.people_graph[parent_idx].sons.push(child_idx);
            }
            Sex::Female => {
                self.people_graph[parent_idx].daughters.push(child_idx);
            }
        }
        // update child info about father or mother, panic if the father or mother were already set
        let child = &self.people_graph[child_idx];

        match &parent_child_info.parent_sex {
            Sex::Male => {
                assert!(
                    child.father.is_none(),
                    format!(
                        "tried to overwrite father {} with father {} for child {}",
                        self.get_father_of(child).unwrap().name,
                        parent_child_info.parent_name,
                        parent_child_info.child_name
                    )
                );
                self.people_graph[child_idx].father = Some(parent_idx);
            }
            Sex::Female => {
                assert!(
                    child.mother.is_none(),
                    format!(
                        "tried to overwrite mother {} with mother {} for child {}",
                        self.get_mother_of(child).unwrap().name,
                        parent_child_info.parent_name,
                        parent_child_info.child_name
                    )
                );
                self.people_graph[child_idx].mother = Some(parent_idx);
            }
        }
    }
}
