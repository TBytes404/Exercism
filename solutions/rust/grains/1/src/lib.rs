pub fn square(s: u32) -> u64 {
    1u64 << (s - 1)
}

pub fn total() -> u64 {
    u64::MAX
}

// 1 => 1
// 2 => 2
// 3 => 4
// 4 => 8
