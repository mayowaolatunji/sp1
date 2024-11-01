use sha2::{Digest, Sha256};

use crate::error::Error;

/// Hashes the public inputs in the same format as the Plonk and Groth16 verifiers.
pub fn hash_public_inputs(public_inputs: &[u8]) -> [u8; 32] {
    let mut result = Sha256::digest(public_inputs);

    // The Plonk and Groth16 verifiers operate over a 254 bit field, so we need to zero
    // out the first 3 bits. The same logic happens in the SP1 Ethereum verifier contract.
    result[0] &= 0x1F;

    result.into()
}

/// Decodes the sp1 vkey hash from the string from a call to `vk.bytes32`.
pub fn decode_sp1_vkey_hash(sp1_vkey_hash: &str) -> Result<[u8; 32], Error> {
    let bytes = hex::decode(&sp1_vkey_hash[2..]).map_err(|_| Error::InvalidProgramVkeyHash)?;
    bytes.try_into().map_err(|_| Error::InvalidProgramVkeyHash)
}
