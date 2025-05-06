use catfishing::catfishing;

#[catfishing(Robot)]
struct Human {
    age: usize,
    name: &'static str,
}

struct Robot(Human);

#[test]
fn test() {
    let human = Human {
        age: 42,
        name: "John",
    };
    let robot = Robot(human);
    assert_eq!(*robot.age(), 42);
    assert_eq!(*robot.name(), "John");
}
