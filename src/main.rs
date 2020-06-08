mod lineage;
use lineage::{Lineage, ParentChildInfo};

use crate::lineage::KillError;
use async_std::sync::{Arc, RwLock};
use serde::Deserialize;
use std::convert::Infallible;
use warp::{Filter, Reply};

/// Deserializes a CSV file into a Lineage struct.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the csv file path
///
/// The file should have the following structure
/// ```
///     parent_name, parent_sex, child_name, child_sex
///     Rickard Stark, M, Eddard Stark, M
///     Rickard Stark, M, Brandon Stark, M
/// ```
fn read_lineage_from_file(file_path: &str) -> Lineage {
    let file =
        std::fs::File::open(file_path).unwrap_or_else(|_| panic!("Could open file: {}", file_path));
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All) // trim leading and trailing whitespace from fields
        .from_reader(file);

    let mut lineage = Lineage::new();

    for deserialization_result in rdr.deserialize() {
        // needs to hint the type here so Serde knows what to try to deserialize to
        let person_csv: ParentChildInfo = deserialization_result.expect("Invalid CSV entry");
        lineage.insert(person_csv);
    }
    lineage
}

/// Represents the name query parameter in the request
#[derive(Deserialize)]
struct NameQueryParam {
    name: String,
}

/// GET /successor/{name} => 200 OK with body "{name} successor"
pub fn get_successor_route(
    lineage_ref: Arc<RwLock<Lineage>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get() // only get requests
        .and(warp::path!("successor")) // only matching successor path
        .and(warp::query::<NameQueryParam>()) // only having a name query parameter
        .and(warp::any().map(move || lineage_ref.clone()))
        .and_then(get_successor)
}

/// This cant be inline because async closures are unstable for now, needs to be a standalone function
async fn get_successor(
    query: NameQueryParam,
    lineage: Arc<RwLock<Lineage>>,
) -> Result<impl warp::Reply, Infallible> {
    let maybe_successor = lineage.read().await.next_in_line(&query.name);
    Ok(match maybe_successor {
        None => warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response(),
        Some(successor) => successor.name().to_string().into_response(),
    })
}

/// POST /kill/{name} => 200 OK with body "Killed {name} successfully"
pub fn kill_person_route(
    post_lineage_ref: Arc<RwLock<Lineage>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post() // only post requests
        .and(warp::path!("kill")) // only matching kill path
        .and(warp::query::<NameQueryParam>()) // only having a name query parameter
        .and(warp::any().map(move || post_lineage_ref.clone()))
        .and_then(kill_person)
}

/// This cant be inline because async closures are unstable for now, needs to be a standalone function
async fn kill_person(
    query: NameQueryParam,
    lineage: Arc<RwLock<Lineage>>,
) -> Result<impl warp::Reply, Infallible> {
    let killed = lineage.write().await.kill(&query.name);
    Ok(match killed {
        Ok(()) => format!("Killed {} successfully", query.name).into_response(),
        Err(e) => match e {
            KillError::PersonNotFound => {
                warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response()
            }
            KillError::PersonAlreadyDead => {
                format!("{} was already dead", query.name).into_response()
            }
        },
    })
}

#[tokio::main]
async fn main() {
    let lineage = read_lineage_from_file("got_families.csv");

    // To synchronize reads and writes to the lineage between tasks and threads
    let lineage_shared = Arc::new(RwLock::new(lineage));

    // each route needs a handle to the lineage in order to query or modify it
    let get_successor_lineage_ref = lineage_shared.clone();
    let kill_person_lineage_ref = lineage_shared.clone();

    let routes = get_successor_route(get_successor_lineage_ref)
        .or(kill_person_route(kill_person_lineage_ref));

    // warp runs in a single thread by default, but can be made to run in as many as needed
    // https://github.com/seanmonstar/warp/issues/557#issuecomment-622323015
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
