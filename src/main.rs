mod bls12_381;
mod goldilocks;

use crate::bls12_381::chal_pubkey;
use crate::goldilocks::chal_goldilocks;

fn main() {
    chal_goldilocks();
    chal_pubkey();
}
