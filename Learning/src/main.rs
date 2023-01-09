use std::iter::Sum;

trait Summary {
    fn add(&self, one: i32, other: i32) -> i32;
}

struct AddStruct {

}

impl Summary for AddStruct {
    fn add(&self, one: i32, other: i32) -> i32 {
       one + other
    }
}

fn main() {
    let add_item = AddStruct {};
    println!("{:?}", add_item.add(1, 2));
}