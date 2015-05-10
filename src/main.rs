extern crate type_printer;
mod the_printers;
mod lesson_one;

fn main() {
    the_printers::title();

    let new_vector: Vec<[i32; 2]> = vector_filler();
    type_printer::print_type_of(&new_vector);
    println!("new vector: {:?}", new_vector);

    let crushed_vec = vector_crusher(new_vector);
    type_printer::print_type_of(&crushed_vec);
}

fn vector_filler() -> Vec<[i32; 2]> {
    vec![[1, 2], [2, 3]]
}

fn vector_crusher(vector_to_crush: Vec<[i32; 2]>) -> Vec<i32> {
    let inital_vector: Vec<[i32; 2]> = vector_to_crush.clone();
    let mut crushed_vector: Vec<i32> = vec![];

    for arr in inital_vector.iter() {
        crushed_vector.push(arr[0]*arr[1]);
    }

    crushed_vector
}
