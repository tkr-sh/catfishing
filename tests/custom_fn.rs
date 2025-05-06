use catfishing::catfishing;

fn custom_fn<T: Clone>(ok: &Option<T>) -> T {
    ok.as_ref().unwrap().clone()
}

#[catfishing(crate::hey::A(custom_fn))]
#[derive(Debug, Clone)]
pub struct B {
    a: usize,
    v: Vec<char>,
}

pub mod hey {
    pub struct A(pub Option<crate::B>);
}

#[test]
fn test() {
    let a = hey::A(Some(B {
        a: 42,
        v: vec!['a', 'b'],
    }));

    assert_eq!(a.a(), 42);
    assert_eq!(a.v(), &['a', 'b']);
}
