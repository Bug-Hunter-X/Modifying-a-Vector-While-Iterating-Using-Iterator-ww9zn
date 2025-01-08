fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    
    // Safe way to modify the vector while iterating
    let new_vec = vec.iter().map(|&x| x * 2).collect::<Vec<_>>();
    println!("Modified Vector: {:?}", new_vec);

    // Or use a for loop and index
    for i in 0..vec.len() {
        vec[i] = vec[i] * 2
    }
    println!("Modified Vector using for loop: {:?}", vec);
} 