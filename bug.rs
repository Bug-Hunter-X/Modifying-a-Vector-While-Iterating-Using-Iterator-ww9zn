fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    // Modify the vector while iterating over it.
    vec.push(3); // This is the bug!
    println!("Second element: {:?}", iter.next());
}