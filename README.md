# craft ai Project

This project aims to help Westerosi Maesters figure out the lines of successions in the noble families.

They use as input the dataset `got_families.csv` which represents the state of the different houses at the beginning of the books.

The data is presented in the following form:

|parent_name  |parent_sex|child_name    |child_sex|
|-------------|----------|--------------|---------|
|Rickard Stark| M        | Eddard Stark | M       |
|Rickard Stark| M        | Brandon Stark| M       |
|Rickard Stark| M        | Benjen Stark | M       |


## Succession rules

Here are the succession rules for the Westeros houses (neither book nor show accurate). 

Because we are not really sure on anyone's age, the Maesters are forgetful sometimes, 
the alphabetical order is used to break ties.

- Sons
- Brothers
- Nephews (son of the brother)
- Daughters
- Sisters
- Nieces (daughter of the brother)
- Any remaining member of the house

## Queries

The goal is to create a simple HTTP API that makes it possible to perform the following queries:

- Find next in line: given a name, return who is next in line for their title.
    
- Kill: given a name, kill the character! Obviously this will have an impact on their house's lineage.

## Output

We expect a fully runnable project with source code.
We will assess the overall architecture and code quality of the project.
Technical choices are up to you.
