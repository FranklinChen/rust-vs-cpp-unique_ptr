fn main() {
    let orig = box 5i;

    println!("{}", *orig);

    let stolen = orig;

    println!("{}", *orig);
}
