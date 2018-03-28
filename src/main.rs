extern crate is_fourtytwo;
use is_fourtytwo::Is;

fn main() {
    println!("{}", Is::new(42).fourtytwo());
}

#[test]
fn test() {
    assert_eq!(true, Is::new(42).fourtytwo());
    assert_eq!(false, Is::new(42).plus(1).fourtytwo());
    assert_eq!(false, Is::new(42).minus(1).fourtytwo());
}
