use cxx::{let_cxx_string};

#[cxx::bridge(namespace = "prs")]
// #[cxx::bridge]
mod ffi {

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("demo/include/person.hpp");

        type Person;

        fn newPerson(name: &CxxString, zip: &CxxString, dob: u32) -> UniquePtr<Person>;

        // Public functions for person.hpp

        fn getAge(self: &Person) -> u32;
        fn getZip(self: &Person) -> &CxxString;
        fn updateZip(self: Pin<&mut Person>, zip: &CxxString);
    }
}

fn main() {
    let_cxx_string!(some_person = "Susi Bluemli");
    let_cxx_string!(some_zip = "Susi Bluemli");
    let mut person = ffi::newPerson(&some_person, &some_zip, 65);
    println!("Age of person is {}", person.getAge());


    let_cxx_string!(updated_zip = "667");
    person.as_mut().unwrap().updateZip(&updated_zip);

    println!("Zip of person is {}", person.getZip());

    // Upload a blob.
    // let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    // let mut buf = MultiBuf { chunks, pos: 0 };
    // let blobid = client.put(&mut buf);
    // println!("blobid = {}", blobid);

    // // Add a tag.
    // client.tag(blobid, "rust");

    // // Read back the tags.
    // let metadata = client.metadata(blobid);
    // println!("tags = {:?}", metadata.tags);
}
