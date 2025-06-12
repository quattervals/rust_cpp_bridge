use cxx::{UniquePtr, let_cxx_string};

#[cxx::bridge(namespace = "prs")]
mod ffi {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("person.hpp");

        type Person;

        fn newPerson(name: &CxxString, zip: &CxxString, dob: u32) -> UniquePtr<Person>;

        fn getAge(self: &Person) -> u32;
        fn getZip(self: &Person) -> &CxxString;
        fn updateZip(self: Pin<&mut Person>, zip: &CxxString);
    }
}

/// A Rust wrapper for the C++ Person class that provides a clean, idiomatic API
struct Person {
    inner: UniquePtr<ffi::Person>,
}

impl Person {
    pub fn new(name: &str, zip: &str, dob: u32) -> Self {
        let_cxx_string!(cxx_name = name);
        let_cxx_string!(cxx_zip = zip);

        Self {
            inner: ffi::newPerson(&cxx_name, &cxx_zip, dob),
        }
    }

    pub fn age(&self) -> u32 {
        self.inner.getAge()
    }

    pub fn zip(&self) -> String {
        self.inner.getZip().to_string()
    }

    pub fn update_zip(&mut self, zip: &str) {
        let_cxx_string!(cxx_zip = zip);
        self.inner.as_mut().unwrap().updateZip(&cxx_zip);
    }
}

fn main() {
    let mut person = Person::new("Susi Bluemli", "698", 1965);
    println!("Current age of person is {}", person.age());

    person.update_zip("667");
    println!("Zip of person is {}", person.zip());

    println!("Person details: age={}, zip={}", person.age(), person.zip());
}
