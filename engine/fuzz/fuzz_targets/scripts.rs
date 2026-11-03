//! An Xbox cache's scenario's scripts, fixed up as the game loads them,
//! from the cache's header and its inflated body: the body is deflated
//! here, so that the inputs reach the syntax data, not just the stream.
#![no_main]

use libfuzzer_sys::fuzz_target;
use opence_import::cache::HEADER_SIZE;
use opence_import::script::{fixed_up_nodes, import_scripts};
use opence_import::Cache;

fuzz_target!(|input: &[u8]| {
    if input.len() < HEADER_SIZE {
        return;
    }
    let (header, body) = input.split_at(HEADER_SIZE);
    let mut file = header.to_vec();
    file[8..12].copy_from_slice(&((HEADER_SIZE + body.len()) as u32).to_le_bytes());
    file.extend_from_slice(&miniz_oxide::deflate::compress_to_vec_zlib(body, 1));
    if let Ok(cache) = Cache::open(file) {
        let _ = import_scripts(&cache);
        let _ = fixed_up_nodes(&cache);
    }
});
