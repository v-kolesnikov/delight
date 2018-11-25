pub fn one_right_1_to_0(x: i32) -> i32 {  x & (x - 1) }
pub fn one_right_0_to_1(x: i32) -> i32 {  x | (x + 1) }
pub fn all_right_1_to_0(x: i32) -> i32 {  x & (x + 1) }
pub fn all_right_0_to_1(x: i32) -> i32 {  x | (x - 1) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(one_right_1_to_0(0b01011000), 0b01010000);
        assert_eq!(one_right_0_to_1(0b10100111), 0b10101111);
        assert_eq!(all_right_1_to_0(0b10100111), 0b10100000);
        assert_eq!(all_right_0_to_1(0b10101000), 0b10101111);
    }
}
