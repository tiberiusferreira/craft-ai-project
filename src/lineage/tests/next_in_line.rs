use crate::lineage::{
    Lineage, ParentChildInfo,
    Sex::{Female, Male},
};

/// Creates the following dummy lineage used in testing
///
/// The first has a Female of Lineage L1 (F_L1) who is married to a Male
/// of Lineage L2 (M_L2).
///
/// The lines indicate who is child of which couple. The terminology is:
/// SA_L2 =  Son A of Lineage L2
/// DA_L2 = Daughter A of Lineage L2
/// EM_LE2 = External Male of Lineage LE2
/// EF_LE3 = External Female of Lineage LE3
/// (EF_LE1 + SA_L2) = Couple formed by EF_LE1 and SA_L2
///
///                        (F_L1    +    M_L2)
///                       /   |       |     \
///                     /     |       |      \
///     (EF_LE1 + SA_L2)   SB_L2   DA_L2   (DB_L2    +     EM_LE2)
///            \      \                      \     \        \     \
///             \      \                      \     \        \     \
/// (EF_LE3 + SC_L2) DC_L2                SD_LE2  SE_LE2  DD_LE2  DE_LE2
///      /    \
///     /      \
///   SF_L2   DF_L2
fn create_lineage() -> Lineage {
    let mut lineage = Lineage::new();

    lineage.insert(ParentChildInfo::new("F L1", Female, "SA L2", Male));
    lineage.insert(ParentChildInfo::new("F L1", Female, "SB L2", Male));
    lineage.insert(ParentChildInfo::new("F L1", Female, "DA L2", Female));
    lineage.insert(ParentChildInfo::new("F L1", Female, "DB L2", Female));
    lineage.insert(ParentChildInfo::new("M L1", Male, "SA L2", Male));
    lineage.insert(ParentChildInfo::new("M L1", Male, "SB L2", Male));
    lineage.insert(ParentChildInfo::new("M L1", Male, "DA L2", Female));
    lineage.insert(ParentChildInfo::new("M L1", Male, "DB L2", Female));

    lineage.insert(ParentChildInfo::new("EF LE1", Female, "SC L2", Male));
    lineage.insert(ParentChildInfo::new("EF LE1", Female, "DC L2", Female));
    lineage.insert(ParentChildInfo::new("SA L2", Male, "SC L2", Male));
    lineage.insert(ParentChildInfo::new("SA L2", Male, "DC L2", Female));

    lineage.insert(ParentChildInfo::new("DB L2", Female, "SD LE2", Male));
    lineage.insert(ParentChildInfo::new("DB L2", Female, "SE LE2", Male));
    lineage.insert(ParentChildInfo::new("DB L2", Female, "DD LE2", Female));
    lineage.insert(ParentChildInfo::new("DB L2", Female, "DE LE2", Female));

    lineage.insert(ParentChildInfo::new("EM L2", Male, "SD LE2", Male));
    lineage.insert(ParentChildInfo::new("EM L2", Male, "SE LE2", Male));
    lineage.insert(ParentChildInfo::new("EM L2", Male, "DD LE2", Female));
    lineage.insert(ParentChildInfo::new("EM L2", Male, "DE LE2", Female));

    lineage.insert(ParentChildInfo::new("EF LE3", Female, "SF L2", Male));
    lineage.insert(ParentChildInfo::new("EF LE3", Female, "DF L2", Female));

    lineage.insert(ParentChildInfo::new("SC L2", Male, "SF L2", Male));
    lineage.insert(ParentChildInfo::new("SC L2", Male, "DF L2", Female));

    lineage
}

#[test]
fn son_is_next_in_line() {
    let mut lin = create_lineage();
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SC L2");
    // check we respect alphabetical order
    assert_eq!(lin.next_in_line("DB L2").unwrap().name, "SD LE2");
    lin.kill("SD LE2").unwrap();
    assert_eq!(lin.next_in_line("DB L2").unwrap().name, "SE LE2");
}

#[test]
fn brother_is_after_son() {
    let mut lin = create_lineage();
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SC L2");
    lin.kill("SC L2").unwrap(); // kill son
                                // Brother is next in line
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SB L2");
}

#[test]
fn nephew_is_after_brother() {
    let mut lin = create_lineage();
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SC L2");
    lin.kill("SC L2").unwrap(); // kill son
                                // Brother is next in line
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SB L2");
    lin.kill("SB L2").unwrap(); // kill brother
                                // Nephew (from sister DB_L2) is next in line
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SD LE2");
    lin.kill("SD LE2").unwrap();
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "SE LE2");
}

#[test]
fn daughter_is_after_nephew() {
    let mut lin = create_lineage();
    // evaluating SA L2
    lin.kill("SC L2").unwrap(); // kill son
    lin.kill("SB L2").unwrap(); // kill brother
    lin.kill("SD LE2").unwrap(); // kill nephew 1
    lin.kill("SE LE2").unwrap(); // kill nephew 2
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "DC L2");
}

#[test]
fn sister_is_after_daughter() {
    let mut lin = create_lineage();
    // evaluating SA L2
    lin.kill("SC L2").unwrap(); // kill son
    lin.kill("SB L2").unwrap(); // kill brother
    lin.kill("SD LE2").unwrap(); // kill nephew 1
    lin.kill("SE LE2").unwrap(); // kill nephew 2
    lin.kill("DC L2").unwrap(); // kill daughter
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "DA L2");
    lin.kill("DA L2").unwrap(); // kill first sister
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "DB L2");
}

#[test]
fn niece_is_after_sister() {
    let mut lin = create_lineage();
    // evaluating SA L2
    lin.kill("SC L2").unwrap(); // kill son
    lin.kill("SB L2").unwrap(); // kill brother
    lin.kill("SD LE2").unwrap(); // kill nephew 1
    lin.kill("SE LE2").unwrap(); // kill nephew 2
    lin.kill("DC L2").unwrap(); // kill daughter
    lin.kill("DA L2").unwrap(); // kill first sister
    lin.kill("DB L2").unwrap(); // kill second sister
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "DD LE2");
    lin.kill("DD LE2").unwrap(); // kill first niece
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "DE LE2"); // second niece should assume
}

#[test]
fn anyone_alive_from_house_after_niece() {
    let mut lin = create_lineage();
    // evaluating SA L2
    lin.kill("SC L2").unwrap(); // kill son
    lin.kill("SB L2").unwrap(); // kill brother
    lin.kill("SD LE2").unwrap(); // kill nephew 1
    lin.kill("SE LE2").unwrap(); // kill nephew 2
    lin.kill("DC L2").unwrap(); // kill daughter
    lin.kill("DA L2").unwrap(); // kill first sister
    lin.kill("DB L2").unwrap(); // kill second sister
    lin.kill("DD LE2").unwrap(); // kill first niece
    lin.kill("DE LE2").unwrap(); // kill second niece
    assert_eq!(lin.next_in_line("SA L2").unwrap().name, "DF L2");
}
