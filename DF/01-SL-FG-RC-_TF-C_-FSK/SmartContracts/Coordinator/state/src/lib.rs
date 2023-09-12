
#![no_std]

use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};


#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    /// Get a list of pingers.
    pub fn pingers(state: State) -> Vec<ActorId> {
        state.into_iter().map(|(pinger, _)| pinger).collect()
    }

    /// Get ping count for a pinger.
    pub fn ping_count(state: State, actor: ActorId) -> u128 {
        state
            .into_iter()
            .find_map(|(pinger, ping_count)| (pinger == actor).then_some(ping_count))
            .unwrap_or_default()
    }

    
     pub fn finalstates(state: State, actor: ActorId) -> Vec<&Str> {
        state.into_iter().map(|(_, state)| state).collect()
    }
}