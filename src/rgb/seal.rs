// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use bitcoin::hash_types::Txid;
use crate::bp::{
    short_id::ShortId,
    blind::{OutpointReveal, OutpointHash}
};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Display, Default)]
#[display_from(Debug)]
pub struct Type(pub u16);


#[derive(Clone, PartialEq, PartialOrd, Debug, Display)]
#[display_from(Debug)]
pub enum Seal {
    /// Seal contained within the commitment transaction
    Local(u16),
    /// Seal that is revealed
    Revealed(OutpointReveal, Option<ShortId>),
    /// Seal that is not revealed yet
    Blinded(OutpointHash)
}

impl Seal {
    pub fn local(vout: u16) -> Self {
        Self::Local(vout)
    }
    pub fn revealed(txid: Txid, vout: u16, blinding: u64) -> Self {
        Seal::Revealed(OutpointReveal { blinding, txid, vout, }, None)
    }
    pub fn maybe_from_outpoint(outpoint: bitcoin::OutPoint, blinding: u64) -> Option<Self> {
        let vout = outpoint.vout;
        if vout > std::u16::MAX as u32 {
            return None
        }
        Some(Seal::Revealed(OutpointReveal { blinding, txid: outpoint.txid, vout: vout as u16 }, None))
    }
    pub fn blinded(hash: OutpointHash) -> Self {
        Self::Blinded(hash)
    }
}
