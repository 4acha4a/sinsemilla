use super::K;
use pasta_curves::pallas;
use group::GroupEncoding;

/// The precomputed bases for the [Sinsemilla hash function][concretesinsemillahash].
///
/// [concretesinsemillahash]: https://zips.z.cash/protocol/protocol.pdf#concretesinsemillahash
const COMPRESSED_POINT_SIZE: usize = 32;
const SINSEMILLA_POINT_COUNT: usize = 1 << K;
pub static SINSEMILLA_S_COMPRESSED:
    &[u8; SINSEMILLA_POINT_COUNT * COMPRESSED_POINT_SIZE] =
    include_bytes!("sinsemilla_s_compressed.bin");

pub(crate) fn sinsemilla_s(index: u32) -> pallas::Affine {
    let index = index as usize;

    assert!(
        index < SINSEMILLA_POINT_COUNT,
        "Sinsemilla generator index is out of range"
    );

    let offset = index * COMPRESSED_POINT_SIZE;

    let mut encoded =
        <pallas::Affine as GroupEncoding>::Repr::default();

    encoded.as_mut().copy_from_slice(
        &SINSEMILLA_S_COMPRESSED
            [offset..offset + COMPRESSED_POINT_SIZE],
    );

    pallas::Affine::from_bytes(&encoded)
        .unwrap()
}