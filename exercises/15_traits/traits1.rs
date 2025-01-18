// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

// A trait is a group of functions that define bahaviour
// Thus I added two equivalent functions to accomplish the same result 
trait AppendBar {
    fn append_bar(self) -> Self;
    fn append_bar2(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self {
        // make a mutable slice of whatever the current string data is
        let mut new_string: String = String::from(&self[..]);
        new_string.push_str("Bar");
        new_string
    }
    fn append_bar2(self) -> Self {
        // this solution looks shorter but also very opaque because of the macro   
        let new_string: String = format!("{}{}", &self[..], "Bar");
        new_string
    }
}
fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
