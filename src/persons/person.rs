#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: &'static str,
    pub age: u8,
}