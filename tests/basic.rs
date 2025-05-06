use catfishing::catfishing;

#[catfishing(A)]
struct B {
    a: usize,
    v: Vec<char>,
}
struct A(B);

#[test]
fn test() {
    let a = A(B {
        a: 42,
        v: vec!['a', 'b'],
    });

    assert_eq!(*a.a(), 42);
    assert_eq!(a.v(), &['a', 'b']);
}
