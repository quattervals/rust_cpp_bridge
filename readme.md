# Use C++ in Rust using Cxx/Autocxx

## Approach

We want to use the C++ Library "Person" in Rust.
One of the constraints is that we don't want to alter the C++ library. Thus, we prefer leaking C++ types (i.e. their cxx-crate equivalents into Rust, but not the other way.


## Todo

- switch to autocxx
- switch to diplomat
- make a binding crate and a consumer crate
