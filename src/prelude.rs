pub use actix::{
    fut::{ActorFuture, ActorStream},
    *,
};
pub use bitcoin_hashes::{
    hex::{FromHex, ToHex},
    hmac, ripemd160, sha256, Hash, HashEngine,
};
pub use futures_locks as locks;
pub use tokio::{
    io, net,
    prelude::{
        future::{Future, Loop},
        stream::Stream,
        *,
    },
    reactor, runtime, sync,
};

macro_rules! arbiter_spawn {
    ($expr:expr) => {
        Arbiter::spawn($expr.then(|_| Ok(())))
    };
}
