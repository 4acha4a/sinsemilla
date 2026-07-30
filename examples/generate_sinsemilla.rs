use group::{Curve, GroupEncoding};
use pasta_curves::{
    arithmetic::CurveExt,
    pallas,
};
use std::{
    fs::File,
    io::Write,
};

const K: usize = 10;
const S_PERSONALIZATION: &str = "z.cash:SinsemillaS";
const OUTPUT_PATH: &str = "src/sinsemilla_s_compressed.bin";

fn main() {
    let hasher = pallas::Point::hash_to_curve(S_PERSONALIZATION);
    let mut output = File::create(OUTPUT_PATH)
        .expect("failed to create compressed Sinsemilla table");

    for index in 0..(1u32 << K) {
        let point = hasher(&index.to_le_bytes()).to_affine();
        let encoded = point.to_bytes();

        output
            .write_all(encoded.as_ref())
            .expect("failed to write compressed Sinsemilla point");
    }

    output.flush().expect("failed to flush output file");
}