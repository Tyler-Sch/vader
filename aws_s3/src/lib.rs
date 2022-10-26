pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod s3;
use s3::get_s3_data;
 pub use aws_sdk_s3::types::AggregatedBytes;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
