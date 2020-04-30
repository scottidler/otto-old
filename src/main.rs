use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yaml::Error> {
    let p1 = Point { x: 1.0, y: 2.0 };
    println!("p1: {:?}", &p1);

    let s = serde_yaml::to_string(&p1)?;
    assert_eq!(s, "---\nx: 1.0\ny: 2.0");

    let p2: Point = serde_yaml::from_str(&s)?;
    assert_eq!(p1, p2);
    println!("p2: {:?}", &p2);
    Ok(())
}
