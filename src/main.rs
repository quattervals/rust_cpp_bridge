// #[cxx::bridge(namespace = "prs")]
#[cxx::bridge]
mod ffi {


    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("person.hpp");

        type Person;


        fn newPerson(name: &str, zip: &str, dob: u32) -> UniquePtr<Person>;

        // Public functions for person.hpp

        fn getAge() -> u32;
        fn getZip() -> &str;
        fn updateZip(zip: &str);
    }
}


fn main() {
    let mut person = ffi::newPerson("Susi Blüemli", "666", 65);
    println!("Age of person is {}", person.getAge());

    person.updateZip("667");

    println!("Zip of person is {}" person.getZip());

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
