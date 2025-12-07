pub mod backtracking;
pub mod direction;
pub mod extensions;
pub mod grid;
pub mod ranges;

pub fn num_digits(n: u64) -> u32 {
    f32::log10(n as f32) as u32 + 1
}
