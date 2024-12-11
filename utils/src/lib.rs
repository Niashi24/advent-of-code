
pub mod grid;
pub mod ranges;
pub mod extensions;
pub mod backtracking;

pub fn num_digits(n: u64) -> u32 {
    f32::log10(n as f32) as u32 + 1
}