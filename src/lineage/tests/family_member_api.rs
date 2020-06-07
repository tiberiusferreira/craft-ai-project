use crate::lineage::{
    Lineage, ParentChildInfo,
    Sex::{Female, Male},
};

#[test]
#[should_panic]
fn require_house_name_in_parent() {
    let mut lineage = Lineage::new();
    let parent_child = ParentChildInfo::new("Parent1", Male, "Child1 House1", Male);
    lineage.insert(parent_child);
}

#[test]
#[should_panic]
fn require_house_name_in_child() {
    let mut lineage = Lineage::new();
    let parent_child = ParentChildInfo::new("Parent1 House", Male, "Child1", Male);
    lineage.insert(parent_child);
}

#[test]
#[should_panic]
fn prevents_overwriting_father() {
    let mut lineage = Lineage::new();
    let parent_child = ParentChildInfo::new("Parent1 House1", Male, "Child1 House1", Male);
    let parent_child2 = ParentChildInfo::new("Parent2 House2", Male, "Child1 House1", Male);
    lineage.insert(parent_child);
    lineage.insert(parent_child2);
}

#[test]
#[should_panic]
fn prevents_overwriting_mother() {
    let mut lineage = Lineage::new();
    let parent_child = ParentChildInfo::new("Parent1 House1", Female, "Child1 House1", Male);
    let parent_child2 = ParentChildInfo::new("Parent2 House2", Female, "Child1 House1", Male);
    lineage.insert(parent_child);
    lineage.insert(parent_child2);
}

#[test]
fn can_get_father_and_son() {
    let mut lineage = Lineage::new();
    let father_name = "Father House";
    let son_name = "Son House";
    let parent_child = ParentChildInfo::new(father_name, Male, son_name, Male);
    lineage.insert(parent_child);
    let son = lineage.get_from_name(son_name).unwrap();
    assert_eq!(lineage.get_father_of(son).unwrap().name, father_name);
    let father = lineage.get_from_name(father_name).unwrap();
    let sons = lineage.get_sons_of(father);
    assert_eq!(lineage.get_daughters_of(father).len(), 0);
    assert_eq!(sons.len(), 1);
    assert_eq!(sons.first().unwrap().name, son_name);
}

#[test]
fn can_get_mother_and_daughter() {
    let mut lineage = Lineage::new();
    let mother_name = "Father House";
    let daughter_name = "Daughter House";
    let parent_child = ParentChildInfo::new(mother_name, Female, daughter_name, Female);
    lineage.insert(parent_child);
    let daughter = lineage.get_from_name(daughter_name).unwrap();
    assert_eq!(lineage.get_mother_of(daughter).unwrap().name, mother_name);
    let mother = lineage.get_from_name(mother_name).unwrap();
    let daughters = lineage.get_daughters_of(mother);
    assert_eq!(lineage.get_sons_of(mother).len(), 0);
    assert_eq!(daughters.len(), 1);
    assert_eq!(daughters.first().unwrap().name, daughter_name);
}

#[test]
fn can_get_brother_and_sister() {
    let mut lineage = Lineage::new();
    let mother_name = "Mother House";
    let father_name = "Father House";
    let daughter_name = "Daughter House";
    let son_name = "Son House";
    let father_second_son_name = "SecondSon House";
    let mother_daughter = ParentChildInfo::new(mother_name, Female, daughter_name, Female);
    let father_daughter = ParentChildInfo::new(father_name, Male, daughter_name, Female);
    let mother_son = ParentChildInfo::new(mother_name, Female, son_name, Male);
    let father_son = ParentChildInfo::new(father_name, Male, son_name, Male);
    let father_second_son = ParentChildInfo::new(father_name, Male, father_second_son_name, Male);
    lineage.insert(mother_daughter);
    lineage.insert(mother_son);
    lineage.insert(father_son);
    lineage.insert(father_second_son);
    lineage.insert(father_daughter);

    // Daughter must have two brothers
    let daughter = lineage.get_from_name(daughter_name).unwrap();
    let mut sister_brothers = lineage.get_brothers(daughter);
    assert_eq!(sister_brothers.len(), 2);
    sister_brothers.sort();
    assert_eq!(
        sister_brothers.first().unwrap().name,
        father_second_son_name
    );
    assert_eq!(sister_brothers[1].name, son_name);

    // Each brother must have one sister
    let son = lineage.get_from_name(son_name).unwrap();
    let sisters = lineage.get_sisters(son);
    assert_eq!(sisters.len(), 1);
    assert_eq!(sisters.first().unwrap().name, daughter_name);

    let second_son = lineage.get_from_name(father_second_son_name).unwrap();
    let sisters = lineage.get_sisters(second_son);
    assert_eq!(sisters.len(), 1);
    assert_eq!(sisters.first().unwrap().name, daughter_name);

    // Each brother must have only each other as brother
    let son_brothers = lineage.get_brothers(son);
    assert_eq!(son_brothers.len(), 1);
    assert_eq!(son_brothers.first().unwrap().name, father_second_son_name);

    let second_son_brothers = lineage.get_brothers(second_son);
    assert_eq!(second_son_brothers.len(), 1);
    assert_eq!(second_son_brothers.first().unwrap().name, son_name);
}

#[test]
fn can_get_nephew_and_niece() {
    // nephew is the son of ones brother or sister
    let mut lineage = Lineage::new();
    let mother_name = "Mother House";
    let sister_name = "Daughter House";
    let brother_name = "Son House";
    // this is the son of the brother, so it is the nephew of the sister
    let nephew_name = "Nephew House";
    // this is the daughter of the brother, so it is the niece of the sister
    let niece_name = "Niece House";
    let mother_daughter = ParentChildInfo::new(mother_name, Female, sister_name, Female);
    let mother_son = ParentChildInfo::new(mother_name, Female, brother_name, Male);
    let brother_nephew = ParentChildInfo::new(brother_name, Male, nephew_name, Male);
    let brother_niece = ParentChildInfo::new(brother_name, Male, niece_name, Female);
    lineage.insert(mother_daughter);
    lineage.insert(mother_son);
    lineage.insert(brother_nephew);
    lineage.insert(brother_niece);
    let sister = lineage.get_from_name(sister_name).unwrap();
    let nephew = lineage.get_nephews(sister);
    assert_eq!(nephew.len(), 1);
    assert_eq!(nephew.first().unwrap().name, nephew_name);
    let niece = lineage.get_nieces(sister);
    assert_eq!(niece.len(), 1);
    assert_eq!(niece.first().unwrap().name, niece_name);
}
