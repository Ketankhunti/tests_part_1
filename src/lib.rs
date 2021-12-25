/*#[derive(Debug)]
struct Rectangle {
    width:i32,
    height:i32
}

impl Rectangle {
    fn can_hold(&self,other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
}
}*/

pub fn greeting(name:&str)->String {
    format!("Hello")
}
#[cfg(test)]
mod tests {
   /* use crate::Rectangle;
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width:9,
            height:7
        };
        let smaller = Rectangle{
            width:6,
            height:5
        };

        assert!(larger.can_hold(&smaller));*/
    use crate::greeting;

    #[test]
    #[should_panic(expected = "assertion failed: result.contains(\\\"ketan\\\")")]  // expected value must be equal to return substring value of function
    fn greeting_contains_ketan(){
        let result = greeting("ketan");
        assert!(result.contains("ketan"));
    }
}
