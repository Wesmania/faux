#[faux::create]
pub struct Foo {}

#[faux::methods]
impl Foo {
    pub fn foo(&self, _: impl Fn(i32) -> i32) -> i32 {
        todo!()
    }
}

use faux::when;

#[test]
fn generic() {
    let mut foo = Foo::faux();
    when!(foo.foo).safe_then(|add_one| add_one(2) + 5);
    assert_eq!(foo.foo(|i| i + 1), 8);
}
