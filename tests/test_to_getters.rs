#[macro_use]
extern crate enum_methods;

#[test]
fn test_to_getters() {
    #[derive(EnumIntoGetters, EnumToGetters, Debug)]
    enum MyEnum {
        Foo(i64),
        Bar(bool),
        Baz(String),
        Tup(i32, String, Vec<bool>),
    }

    let foo = MyEnum::Foo(42);
    let bar = MyEnum::Bar(false);
    let baz = MyEnum::Baz("hurry boy, it's waiting there for you".to_string());
    let tup = MyEnum::Tup(42, String::from("Hello, Tuple, my old friend!"), vec![true, false, true]);
    assert_eq!(foo.to_foo(), 42);
    assert_eq!(bar.to_bar(), false);
    assert_eq!(baz.to_baz(), "hurry boy, it's waiting there for you");
    assert_eq!(foo.into_foo(), 42);
    assert_eq!(bar.into_bar(), false);
    assert_eq!(baz.into_baz(), "hurry boy, it's waiting there for you");
    assert_eq!(tup.into_tup(), (42, String::from("Hello, Tuple, my old friend!"), vec![true, false, true]));
}

#[test]
fn test_to_getter_names() {
    #[derive(EnumIntoGetters, EnumToGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz(
        "there's nothing that a hundred men or more could ever do".to_string(),
    );
    assert_eq!(first.to_foo_bar(), true);
    assert_eq!(
        second.to_bar_baz(),
        "there's nothing that a hundred men or more could ever do"
    );
    assert_eq!(first.into_foo_bar(), true);
    assert_eq!(
        second.into_bar_baz(),
        "there's nothing that a hundred men or more could ever do"
    );
}

#[test]
fn test_getter_structs() {
    #[derive(EnumIntoGetters, EnumToGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
        SomeStruct { foo: i32 }, // should be skipped
    }

    impl MyEnum {
        pub fn to_some_struct(&self) -> i32 {
            if let &MyEnum::SomeStruct { ref foo } = self {
                foo.clone()
            } else {
                unreachable!()
            }
        }
        pub fn into_some_struct(self) -> i32 {
            if let MyEnum::SomeStruct { foo } = self {
                foo
            } else {
                unreachable!()
            }
        }
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz(
        "there's nothing that a hundred men or more could ever do".to_string(),
    );
    let third = MyEnum::SomeStruct { foo: 42 };
    assert_eq!(first.to_foo_bar(), true);
    assert_eq!(
        second.to_bar_baz(),
        "there's nothing that a hundred men or more could ever do"
    );
    assert_eq!(third.to_some_struct(), 42);
    assert_eq!(first.into_foo_bar(), true);
    assert_eq!(
        second.into_bar_baz(),
        "there's nothing that a hundred men or more could ever do"
    );
    assert_eq!(third.into_some_struct(), 42);
}
