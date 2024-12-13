fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This is incorrect and will result in undefined behavior.
    let ptr = vec.as_ptr();
    println!("Pointer value: {:p}", ptr);

    // The vector is dropped here, making the pointer invalid
}