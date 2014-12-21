fn main() {
    let v = vec![box 5i];

    println!("{}", *v[0]);

    // Attempted move: type error at compile time.
    let pointer_to_5 = v[0];
    println!("{}", *pointer_to_5);

    println!("{}", *v[0]);
}
