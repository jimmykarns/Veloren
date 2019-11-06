#![allow(incomplete_features)]
#![feature(const_generics, test)]
#![feature(associated_type_bounds)]
#![feature(generic_associated_types)]

extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate test;

pub mod job;
pub mod lodstore;
pub mod region;
pub mod regionmanager;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
