// Reference-counted smart pointer.
use std::rc::Rc;

fn main() {
    let v = vec![Rc::new(5i)];

    println!("{}", *v[0]);

    let pointer_to_5 = v[0].clone();
    println!("{}", *pointer_to_5);

    println!("{}", *v[0]);
}
