mod lineage;
mod visualization;
use lineage::{Lineage, ParentChildInfo};

use crate::lineage::{Person, KillError};
use serde::Deserialize;
use std::sync::{Arc, RwLock};
use warp::Filter;

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
    struct NameQueryParam {
        name: String,
    }

    let query_lineage_ref = lineage.clone();

    // GET /successor/{name} => 200 OK with body "{name} successor"
    let successor_api =
        warp::get()
            .and(warp::path!("successor"))
        .and(warp::query::<NameQueryParam>())
        .map(move |query: NameQueryParam| {
            let maybe_successor = query_lineage_ref.read().unwrap().next_in_line(&query.name);
            return match maybe_successor {
                None => "Invalid name".to_string(),
                Some(successor) => successor.name().to_string(),
            };
        });

    let kill_lineage_ref = lineage.clone();

    // POST /kill/{name} => 200 OK with body "Killed {name} successfully"
    let kill_api = warp::post()
        .and(warp::path!("kill"))
        .and(warp::query::<NameQueryParam>())
        .map(move |query: NameQueryParam| {
            let killed = kill_lineage_ref.write().unwrap().kill(&query.name);
            match killed{
                Ok(()) => format!("Killed {} successfully", query.name),
                Err(e) => {
                    match e{
                        KillError::PersonNotFound => {
                            format!("{} was not found", query.name)
                        },
                        KillError::PersonAlreadyDead => {
                            format!("{} was already dead", query.name)
                        },
                    }
                },
            }
        });

    warp::serve(successor_api.or(kill_api))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
