fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Correct way to access elements is to iterate through the vector
    for i in &vec {
        println!("Vector element: {}", i);
    }

    //Alternative safe way using slices
    let slice = &vec[..];
    for i in slice {
        println!("Vector element: {}", i);
    }
} 