use faux;

#[faux::create]
pub struct Foo {
    a: u32,
}

#[faux::methods]
impl Foo {
    pub fn new(a: u32) -> Self {
        Foo { a }
    }

    pub fn get_stuff(&self) -> u32 {
        self.a
    }

    pub fn add_stuff(&self, x: i32) -> i32 {
        self.a as i32 + x
    }

    pub fn add_stuff_2(&self, x: i32, y: i32) -> i32 {
        self.a as i32 + x + y
    }

    pub fn some_ref(&self, x: &i32) -> i32 {
        self.a as i32 + *x
    }
}

#[test]
fn real_struct() {
    let real = Foo::new(3);
    assert_eq!(real.get_stuff(), 3);
    assert_eq!(real.add_stuff(2), 5);
}

#[test]
fn faux_single_arg() {
    let mut mock = Foo::faux();
    unsafe { faux::when!(mock.get_stuff).then(|_| 10) }
    assert_eq!(mock.get_stuff(), 10);
}

#[test]
fn faux_multi_arg() {
    let mut mock = Foo::faux();
    unsafe { faux::when!(mock.add_stuff_2).then(|(a, _)| a) }
    assert_eq!(mock.add_stuff_2(90, 30), 90);
}

#[test]
fn faux_ref_arguments() {
    let mut mock = Foo::faux();
    unsafe { faux::when!(mock.some_ref).then(|a| *a) }
    let x = 30 + 30;
    assert_eq!(mock.some_ref(&x), 60);
}

#[test]
#[should_panic]
fn unmocked_faux_panics() {
    let mock = Foo::faux();
    mock.get_stuff();
}
