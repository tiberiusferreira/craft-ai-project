# craft ai Project

This project aims to help Westerosi Maesters figure out the lines of successions in the noble families.

They use as input the dataset `got_families.csv` which represents the state of the different houses at the beginning of the books.

The data is presented in the following form:

|parent_name  |parent_sex|child_name    |child_sex|
|-------------|----------|--------------|---------|
|Rickard Stark| M        | Eddard Stark | M       |
|Rickard Stark| M        | Brandon Stark| M       |
|Rickard Stark| M        | Benjen Stark | M       |

An SVG for easier visualization is provided at https://github.com/tiberiusferreira/craft-ai-project/blob/master/got_families.svg and shown below. It was generated using the [Lineage::to_graphviz](https://github.com/tiberiusferreira/craft-ai-project/blob/e93114191b264f7c8177091fdc12b2df330eaf65/src/lineage/mod.rs#L174) function and rendering the graphviz using http://www.webgraphviz.com/.

![GoT SVG](./got_families.svg)

# Table of contents

- [Succession rules](#succession-rules)

- [Usage](#usage)

- [Technical decisions](#technical-decisions)
- - [Underlying Data Structure](#underlying-data-structure)
- - [Request Complexity](#request-complexity)
- - [Data persistence](#data-persistence)
- - [Http Library Choice and Scalability](#http-library-choice-and-scalability)
- - [Tests](#tests)
 
## Succession rules

Here are the succession rules for the Westeros houses (neither book nor show accurate). 

Because we are not really sure on anyone's age, the Maesters are forgetful sometimes, 
the alphabetical order is used to break ties.

- Sons
- Brothers
- Nephews (son of the brother or sister)
- Daughters
- Sisters
- Nieces (daughter of the brother or sister)
- Any remaining member of the house


## Usage

Clone the repository and run `cargo run --release`.

A webserver should be started at 127.0.0.1:3030 with two endpoints:

----
### Next in line

**Description** : Returns who is next in line for the title of person named {name}.

**URL** : `/successor/?{name}` name is passed as an url encoded query parameter

**Method** : `GET`

### Success Response

**Code** : `200 OK`

**Body** : Successors name (String)



### Error Responses

On person not found

**Code** : `404 NOT_FOUND`



**Example**

GET /successor?name=Tytos%20Lannister

**Code** `200 OK`

**Body** `Kevan Lannister`


----

### Kill person

**Description** : Kills the person named {name}.

**URL** : `/kill/?{name}` name is passed as an url encoded query parameter

**Method** : `POST`

### Success Response

**Code** : `200 OK`

**Body** : Killed {name} successfully or {name} was already dead

### Error Responses

On person not found

**Code** : `404 NOT_FOUND`


**Example**

POST /kill?name=Kevan%20Lannister

**Code** `200 OK`

**Body** `Killed Kevan Lannister successfully`


## Technical decisions

## Underlying Data Structure

The family tree is represented using the following structure:
```Rust
/// Represents a family lineage, contains the information of who is alive and the relationship
/// between its members.
/// `people_graph` contains the information itself and `people_graph_indexes` provides a way
/// to translate a person's name to its index in `people_graph`
#[derive(Debug, Default)]
pub struct Lineage {
    people_graph: Vec<Person>,
    people_graph_indexes: HashMap<String, usize>,
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
```

Using `people_graph_indexes` one can obtain a `Person` from a name in O(1) time by getting the corresponding people_graph index from the hashmap and accessing it. The Person struct contains the indices of all direct relatives (parents and children).

## Request Complexity

### Next in Line

Since getting to the Person struct is O(1), getting the next in line is a matter of checking all relatives in the succession rules until arriving at one alive. Each relative can be check in O(1) using the indices stored in the Person struct.

The worst case scenario is when we arrive at the condition:

- Any remaining member of the house

To be considered of the same house the only requirement is having the same last name as the person being queried.

So one has to go through everybody in `people_graph` which is O(n) + sorting(O(n log n) worst case) where n = number of people in graph.

#### Possible Optimization

`Lineage` could be augmented with a field `people_alive_in_house: HashMap<String, Vec<usize>>` storing the indices of people alive on a per house basis, allowing `Any remaining member of the house` to be performed at O(1) if people indices are stored already sorted. However, this complicates the implementation and was not implemented for simplicity's sake.


### Kill person

Killing someone is O(1) since it just sets a flag in the Person struct.


## Data persistence

Changes (killing someone) are kept in memory only. This was deliberate to keep the implementation simple. However, it could be persisted to a database such as `Postgresql` with a table storing the status (alive or dead) of each person.

When killing someone, the program would modify the database table and then its own in memory representation. 

If many instances of the program are running in parallel, they could be notified of changes by using Kafka messaging to ensure consistency. 


## Http Library Choice and Scalability

[Warp](https://github.com/seanmonstar/warp) was chosen as the http library because it compiles on the stable release of Rust, supports async-io, is widely used ([350k downloads](https://crates.io/crates/warp)) and its author is `seanmonstar` a long time Rust contributor and also author of [Hyper](https://github.com/hyperium/hyper).

Warp should allow the application to handle multiple requests in the same thread and also scale linearly with the number of threads provided (by default it runs in a single thread).

Task syncronization on the Lineage struct is performed by using an asyncronous [RwLock](https://docs.rs/async-std/1.6.0/async_std/sync/struct.RwLock.html). It allows multiple tasks to read the same data and ensures consistency on writes. Also, since its asyncronous, if one task blocks (for example, on a write) others can still make progress on the same thread. 

If neither performance nor usage of the stable release of Rust were priorities, [Rocket](https://rocket.rs/) could have been used for an arguably easier development experience, specially since it will [compile on stable](https://github.com/SergioBenitez/Rocket/issues/19) on the 16th of July 

## Tests

The directory `tests` inside the lineage module provides tests for both the internal family member query APIs and the external next_in_line API to ensure correctness.

The tests can be run by running at the root folder `cargo test`.




