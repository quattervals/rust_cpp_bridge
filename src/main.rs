use cxx::let_cxx_string;
use autocxx::prelude::*;

include_cpp! {
    #include "person.hpp"
    safety!(unsafe)
    generate!("prs::Person")

}

fn main() {
    let_cxx_string!(some_person = "Susi Blüemli");
    let_cxx_string!(some_zip = "698");
    let mut _person2 = ffi::prs::Person::newPerson(&some_person, &some_zip, 1960);

    let mut person = ffi::prs::Person::new(&some_person, &some_zip, 1965).within_unique_ptr();

    println!("Current age of person is {}", person.getAge());

    let_cxx_string!(updated_zip = "667");
    person.pin_mut().updateZip(&updated_zip);

    println!("Zip of person is {}", person.getZip());
}
