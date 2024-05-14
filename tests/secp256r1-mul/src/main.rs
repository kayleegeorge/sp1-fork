#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_zkvm::precompiles::secp256r1::Secp256r1Operations;
use sp1_zkvm::precompiles::utils::AffinePoint;

#[sp1_derive::cycle_tracker]
pub fn main() {
    // generator.
    // 48439561293906451759052585252797914202762949526041747995844080717082404635286
    // 36134250956749795798585127919587881956611106672985015071877198253568414405109
    let a: [u8; 64] = [
        150, 194, 152, 216, 69, 57, 161, 244, 160, 51, 235, 45, 129, 125, 3, 119, 242, 64, 164, 99,
        229, 230, 188, 248, 71, 66, 44, 225, 242, 209, 23, 107, 245, 81, 191, 55, 104, 64, 182,
        203, 206, 94, 49, 107, 87, 51, 206, 43, 22, 158, 15, 124, 74, 235, 231, 142, 155, 127, 26,
        254, 226, 66, 227, 79,
    ];

    let mut a_point = AffinePoint::<Secp256r1Operations, 16>::from_le_bytes(a);

    // scalar.
    // 3
    let scalar: [u32; 8] = [3, 0, 0, 0, 0, 0, 0, 0];

    println!("cycle-tracker-start: secp256r1_mul");
    a_point.mul_assign(&scalar);
    println!("cycle-tracker-end: secp256r1_mul");

    // 3 * generator.
    // 42877656971275811310262564894490210024759287182177196162425349131675946712428
    // 61154801112014214504178281461992570017247172004704277041681093927569603776562
    let c: [u8; 64] = [
        108, 253, 231, 198, 27, 102, 65, 251, 133, 169, 173, 239, 33, 183, 198, 230, 101, 241, 75,
        29, 149, 239, 247, 200, 68, 10, 51, 166, 209, 228, 203, 94, 50, 80, 125, 162, 39, 177, 121,
        154, 61, 184, 79, 56, 54, 176, 42, 216, 236, 162, 100, 26, 206, 6, 75, 55, 126, 255, 152,
        73, 12, 100, 52, 135,
    ];

    assert_eq!(a_point.to_le_bytes(), c);

    println!("done");
}
