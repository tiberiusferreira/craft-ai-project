use crate::Lineage;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::text_anchor::*;
use std::collections::HashMap;

pub struct Vis<'a> {
    drawing_area: DrawingArea<BitMapBackend<'a>, Shift>,
}

struct PersonLayout {
    name: String,
    rel_position: (i32, i32),
}
impl<'a> Vis<'a> {
    pub fn new() -> Self {
        Self {
            drawing_area: BitMapBackend::new("result.png", (800, 800)).into_drawing_area(),
        }
    }

    pub fn create_layout(&self, lineage: &Lineage) {
        let people = lineage.people();
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

        let a = Dot::with_config(&deps, &[Config::EdgeNoLabel]);
        println!("{:?}", a);

        // let pg = deps.add_node("petgraph");
        // let fb = deps.add_node("fixedbitset");
        // let qc = deps.add_node("quickcheck");
        // let rand = deps.add_node("rand");
        // let libc = deps.add_node("libc");
        // deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);
    }

    pub fn draw(&mut self) {
        self.drawing_area
            .draw(
                &(EmptyElement::at((100, 100))
                    + Text::new(
                        "Elias",
                        (0, 0),
                        &"sans-serif"
                            .into_font()
                            .resize(20.0)
                            .color(&WHITE)
                            .pos(Pos::new(HPos::Center, VPos::Center)),
                    )
                    + Circle::new((0, 0), 40, Into::<ShapeStyle>::into(&WHITE))),
            )
            .unwrap();

        return;
    }
}
