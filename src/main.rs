mod lineage;
mod visualization;
use lineage::{Lineage, ParentChildInfo};

use serde::Deserialize;
use warp::Filter;
use std::sync::{Arc, RwLock};
use crate::lineage::Person;

#[tokio::main]
async fn main() {

    let file = std::fs::File::open("got_families.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut lineage = Lineage::new();

    for deserialization_result in rdr.deserialize() {
        let person_csv: ParentChildInfo = deserialization_result.unwrap();
        lineage.insert(person_csv);
    }

    let lineage = Arc::new(RwLock::new(lineage));

    #[derive(Deserialize)]
    struct SuccessorQuery {
        name: String
    }

    let query_lineage_ref = lineage.clone();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let successor_api = warp::path!("successor")
        .and(warp::query::<SuccessorQuery>())
        .map(move |query: SuccessorQuery| {
            let maybe_successor = query_lineage_ref.read().unwrap().next_in_line(&query.name);
            return match maybe_successor {
                None => {
                    "Invalid name".to_string()
                },
                Some(successor) => successor.name().to_string(),
            }
        });

    let kill_lineage_ref = lineage.clone();
    let kill_api = warp::path!("kill")
        .and(warp::query::<SuccessorQuery>())
        .map(move |query: SuccessorQuery| {
            let killed = kill_lineage_ref.write().unwrap().kill(&query.name);
            return if killed {
                format!("Killed {} successfully", query.name)
            } else {
                format!("{} is not a valid person", query.name)
            }
        });

    warp::serve(successor_api.or(kill_api))
        .run(([127, 0, 0, 1], 3030))
        .await;

}
