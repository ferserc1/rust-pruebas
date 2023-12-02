//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = 6;
/// 
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}