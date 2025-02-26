use cxx::let_cxx_string;

#[cxx::bridge(namespace = "prs")]
mod ffi {

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("rust-cpp-integration/include/person.hpp");

        type Person;

        fn newPerson(name: &CxxString, zip: &CxxString, dob: u32) -> UniquePtr<Person>;

        fn getAge(self: &Person) -> u32;
        fn getZip(self: &Person) -> &CxxString;
        fn updateZip(self: Pin<&mut Person>, zip: &CxxString);
    }
}

fn main() {
    let_cxx_string!(some_person = "Susi Bluemli");
    let_cxx_string!(some_zip = "698");
    let mut person = ffi::newPerson(&some_person, &some_zip, 1965);
    println!("Current age of person is {}", person.getAge());


    let_cxx_string!(updated_zip = "667");
    person.as_mut().unwrap().updateZip(&updated_zip);

    println!("Zip of person is {}", person.getZip());
}
