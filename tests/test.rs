use named_return::{named_return, named_return_attr};

#[derive(Debug, PartialEq, Eq)]
pub struct A;
#[derive(Debug, PartialEq, Eq)]
pub struct B;

named_return!(
    fn fa() {}
);

named_return!(
    fn fb() -> () {}
);

named_return!(
fn fc() -> (a: A) {
    a = A;
    a
});
// ps. the return is not a tuple

named_return!(
fn fd() -> (a: A, b: B) {
    a = A;
    b = B;
    (a, b)
});

#[test]
fn macros() {
    assert_eq!(fa(), ());
    assert_eq!(fb(), ());
    assert_eq!(fc(), A);
    assert_eq!(fd(), (A, B));
}

#[named_return_attr]
fn fa2() {}

#[named_return_attr]
fn fb2() -> () {}

// attr macros still dont work with named returns
#[named_return_attr]
// fn fc2() -> (a: A) { // TODO: uncomment
fn fc2() -> (A) {
    let a: A; // TODO: comment
    a = A;
    a
}

// attr macros still dont work with named returns
#[named_return_attr]
// fn fd2() -> (a: A, b: B) { // TODO: uncomment
fn fd2() -> (A, B) {
    let (a, b): (A, B); // TODO: comment
    a = A;
    b = B;
    (a, b)
}

#[test]
fn macros_attr() {
    assert_eq!(fa2(), ());
    assert_eq!(fb2(), ());
    assert_eq!(fc2(), A);
    assert_eq!(fd2(), (A, B));
}
