extern crate type_printer;

fn main() {
    println!("\nMessin' with Types\n");

    // lets say we want an empty vector
    //
    // This can't compile, because we don't know its type
    // let empty_vec = vec![];

    let mut empty_vec: Vec<i32> = vec![];
    let new_vec = vector_adder(empty_vec);
    println!("new vec: {:?}", new_vec);

    // why does this compile?
    //
    // is it because the empty vec is used right away in a function
    // that requires a Vec<i32> type?
    println!("new vec: {:?}", vector_adder(vec![]));
}

fn vector_adder(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.push(1);
    new_vec
}
