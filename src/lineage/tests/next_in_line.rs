use crate::lineage::{
    Lineage, ParentChildInfo,
    Sex::{Female, Male},
};

#[test]
fn son_takes_position_of_father() {
    let mut lineage = Lineage::new();
    let father_name = "Father House";
    let son_name = "Son House";
    let parent_child = ParentChildInfo::new(father_name, Male, son_name, Male);
    lineage.insert(parent_child);
    assert_eq!(lineage.next_in_line(father_name).unwrap().name, son_name);
    lineage.kill(son_name);
    // after son is dead, there should be no successors
    assert!(lineage.next_in_line(father_name).is_none());
}

