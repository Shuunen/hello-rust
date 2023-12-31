mod persons;
use persons::Person;

use std::{env};

/*
    - Découper le code
    - Voir la déclaration de contract (struct, enum, trait, impl)
    - Benchmarks
    - Connection avec une DB
*/

fn get_output(name: &str) -> String {
    let now = chrono::Local::now().format("%d/%m/%Y %H:%M");
    return format!("Hello, {} :)\n\nThe date and time is: {}", name, now);
}

fn hello(name: &str) {
    let output = get_output(name);
    println!("{}", &output);
    // write the output to a file
    std::fs::write("output.txt", output).unwrap();
    println!("output.txt created or overwritten");
}

fn create_person(age: u8) -> Person {
    let name = "toto";
    return Person {
        name,
        age,
    };
}

fn main() {
    if let Some(name) = env::args().nth(1) {
        hello(&name);
    } else {
        println!("Please provide an argument");
    }
    let personA = create_person(42);
    println!("Hello, {:?} :)", personA);
    let personB = create_person(51);
    println!("Hello, {:?} :)", personB);
    assert!(personB != personA);
}

#[cfg(test)]
mod tests {
    use super::{get_output};

    #[test]
    fn test_get_output() {
        assert!(get_output("toto").contains("toto"));
    }
}
