// Simulate a segmentation fault.
fn seg_fault() {
    panic!("segmentation fault");
}

fn main() {
    // Rough modeling in Rust of C++ unique_ptr<int>
    // because C++ pointers can always be null.
    let mut orig: Option<Box<int>> = Some(box 5i);

    match orig {
        // In C++, deferencing null seg faults.
        None => seg_fault(),
        Some(orig_nonnull) => {
            println!("{}", *orig_nonnull);

            // The equivalent of C++ unique_ptr move.
            let stolen = Some(orig_nonnull);
            orig = None;

            match orig {
                // We seg fault after the C++ style move.
                None => seg_fault(),
                Some(orig_nonnull) => println!("{}", *orig_nonnull)
            }
        }
    }
}
