pub mod extras;

use extras::extras::add;

pub fn call_add(a: u32, b: u32) -> u32 {
    add(a, b)
}