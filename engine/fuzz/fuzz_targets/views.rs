//! The plugins' readers of the game's structures (opence_runtime::views),
//! over memory made of the input: each answers, none panics or reads
//! outside.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|memory: &[u8]| {
    let _ = opence_runtime::views::walk(memory);
});
