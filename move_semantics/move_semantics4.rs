// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `create_filled_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec1 = create_filled_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `create_filled_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn create_filled_vec() -> Vec<i32> {
    // Instead, let's create and fill the Vec in here - how do you do that?
    vec![22,44,66,88]
}
