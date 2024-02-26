// This program demonstrates the ownership of the heap memory
// Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.
fn main() {
    // The String, Vec, HashMap returns a pointer to the heap memory
    // So first is a pointer to the heap memory [F,e,r,r,i,s]
    let first = String::from("Ferris");
    // We pass the pointer to the function add_suffix changing the ownership
    let full = add_suffix(first);
    // Full gets the ownership of the pointer to the heap memory [F,e,r,r,i,s, ,J,r,.]
    println!("{full}");
    // If we try to print first we will get an error because the ownership was transferred
    // and first points to deallocated memory
}

fn add_suffix(mut name: String) -> String {
    // First does not exist anymore and the ownership was transferred to name
    // Now name is a pointer to the heap memory [F,e,r,r,i,s, ,J,r,.]
    name.push_str(" Jr.");
    // We return the pointer to the heap memory [F,e,r,r,i,s, ,J,r,.]
    name
}
