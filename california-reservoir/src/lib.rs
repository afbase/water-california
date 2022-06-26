#![feature(slice_group_by)]
#![feature(array_chunks)]
// pub mod compression;
// pub mod observation;
// pub mod reservoir;
mod observation;
pub use observation::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
