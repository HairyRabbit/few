pub static button: &'static str = "btn";

// pub enum Style {
//   Foo = "foo"
// }

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref style: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("foo".to_owned(), "foo-hashed");
        m
    };
}