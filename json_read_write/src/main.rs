use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Employee {
  name: String,
  age: i8,
  salary: i32,
}

fn main() {
    let emp1 = Employee { name: String::from("akhil"), age: 26, salary: 100000 };

    let ser = serde_json::to_string(&emp1).unwrap();

    println!("strinigified emp1 {:?}", ser);

    let deser: Employee = serde_json::from_str(&ser).unwrap();

    println!("emp1 {:?}", deser);

}
