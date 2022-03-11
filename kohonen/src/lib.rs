pub mod consts;
pub mod kohonen;

pub use consts::*;
pub use kohonen::*;

#[cfg(feature = "persist")]
pub use kohonen::persist;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
