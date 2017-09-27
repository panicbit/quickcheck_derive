#[macro_use]
extern crate quickcheck_derive;
extern crate quickcheck;
extern crate rand;

use quickcheck::{Arbitrary,StdGen};
use rand::IsaacRng;

#[derive(Arbitrary,Clone,Debug,PartialEq)]
struct UnitStruct;

#[derive(Arbitrary,Clone,Debug,PartialEq)]
struct StructStruct {
    a: i32,
    b: String,
}

#[derive(Arbitrary,Clone,Debug,PartialEq)]
struct TupleStruct(i32, String);

#[derive(Arbitrary,Clone,Debug,PartialEq)]
struct GenericStruct<T,U> {
    t: T,
    u: U,
}

#[test]
fn unit_struct() {
    let ref mut gen = gen();
    assert_eq!(UnitStruct::arbitrary(gen), UnitStruct);
}

#[test]
fn struct_struct() {
    let ref mut gen = gen();
    assert_eq!(StructStruct::arbitrary(gen), StructStruct {
        a: -2,
        b: "ẩ".into(),
    });
}

#[test]
fn tuple_struct() {
    let ref mut gen = gen();
    assert_eq!(TupleStruct::arbitrary(gen), TupleStruct(
        -2,
        "ẩ".into(),
    ));
}

#[test]
fn generic_struct() {
    let ref mut gen = gen();
    assert_eq!(GenericStruct::<i32,String>::arbitrary(gen), GenericStruct {
        t: -2,
        u: "ẩ".into(),
    });
}

fn gen() -> StdGen<IsaacRng> {
    let max_size = 4;
    StdGen::new(IsaacRng::new_unseeded(), max_size)
}
