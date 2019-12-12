// XXX this name seems too long?
use crate::note_commitment_tree::SaplingNoteTreeRootHash;

use crate::redjubjub::{self, Binding, SpendAuth};

/// A _Spend Description_, as described in [protocol specification §7.3][ps].
///
/// [ps]: https://zips.z.cash/protocol/protocol.pdf#spendencoding
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpendDescription {
    /// A value commitment to the value of the input note.
    ///
    /// XXX refine to a specific type.
    pub cv: [u8; 32],
    /// A root of the Sapling note commitment tree at some block height in the past.
    pub anchor: SaplingNoteTreeRootHash,
    /// The nullifier of the input note.
    ///
    /// XXX refine to a specific type.
    pub nullifier: [u8; 32],
    /// The randomized public key for `spend_auth_sig`.
    pub rk: redjubjub::PublicKeyBytes<SpendAuth>,
    /// The ZK spend proof.
    ///
    /// XXX add proof types.
    /// XXX for now it's [u64; 24] instead of [u8; 192] to get trait impls
    pub zkproof: [u64; 24],
    /// A signature authorizing this spend.
    pub spend_auth_sig: redjubjub::Signature<SpendAuth>,
}

/// A _Output Description_, as described in [protocol specification §7.4][ps].
///
/// https://zips.z.cash/protocol/protocol.pdf#outputencoding
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputDescription {
    /// A value commitment to the value of the input note.
    ///
    /// XXX refine to a specific type.
    pub cv: [u8; 32],
    /// The u-coordinate of the note commitment for the output note.
    ///
    /// XXX refine to a specific type.
    pub cmu: [u8; 32],
    /// An encoding of an ephemeral Jubjub public key.
    ///
    /// XXX refine to a Jubjub key agreement type, not RedJubjub.
    pub ephemeral_key: [u8; 32],
    /// A ciphertext component for the encrypted output note.
    ///
    /// XXX refine to a specific type.
    /// XXX this is a Vec<u8> rather than a [u8; 580] to get trait impls
    pub enc_ciphertext: Vec<u8>,
    /// A ciphertext component for the encrypted output note.
    ///
    /// XXX refine to a specific type.
    /// XXX this is a [u64; 10] rather than a [u8; 80] to get trait impls
    pub out_ciphertext: [u64; 10],
    /// The ZK output proof.
    ///
    /// XXX add proof types.
    /// XXX for now it's [u64; 24] instead of [u8; 192] to get trait impls
    pub zkproof: [u64; 24],
}

/// Sapling-on-Groth16 spend and output descriptions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShieldedData {
    /// A sequence of [`SpendDescription`]s for this transaction.
    pub shielded_spends: Vec<SpendDescription>,
    /// A sequence of shielded outputs for this transaction.
    pub shielded_outputs: Vec<OutputDescription>,
    /// A signature on the transaction hash.
    pub binding_sig: redjubjub::Signature<Binding>,
}
