use crate::error::Error;
use common::msg::{
    ClientChunkMsg, ClientDefaultMsg, ClientState, ClientStateMsg, RequestStateError,
    ServerChunkMsg, ServerDefaultMsg, ServerLoginMsg, ServerStateMsg,
};
use futures_util::{select, FutureExt};
use hashbrown::HashSet;
use network::{Participant, Stream};
use serde::Serialize;
use specs::{Component, FlaggedStorage};
use specs_idvs::IdvStorage;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tracing::debug;
use vek::*;

pub struct Client {
    pub client_state: ClientState,
    pub participant: Mutex<Option<Participant>>,
    pub registration_stream: Stream,
    pub default_stream: Stream,
    pub chunks_stream: Stream,
    pub network_error: AtomicBool,
    pub last_ping: f64,
    pub login_msg_sent: bool,
}

impl Component for Client {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>;
}

impl Client {
    fn int_notify<M: Serialize>(network_error: &AtomicBool, stream: &mut Stream, msg: M) {
        if !network_error.load(Ordering::Relaxed) {
            if let Err(e) = stream.send(msg) {
                debug!(?e, "got a network error with client");
                network_error.store(true, Ordering::Relaxed);
            }
        }
    }

    pub fn notify(&mut self, msg: ServerDefaultMsg) {
        Self::int_notify(&self.network_error, &mut self.default_stream, msg);
    }

    pub fn notify_register(&mut self, msg: ServerLoginMsg) {
        Self::int_notify(&self.network_error, &mut self.registration_stream, msg);
    }

    pub fn notify_state(&mut self, msg: ServerStateMsg) {
        Self::int_notify(&self.network_error, &mut self.registration_stream, msg);
    }

    pub fn notify_chunk(&mut self, msg: ServerChunkMsg) {
        Self::int_notify(&self.network_error, &mut self.chunks_stream, msg);
    }

    pub async fn recv(
        &mut self,
    ) -> Result<
        (
            Option<ClientDefaultMsg>,
            Option<ClientStateMsg>,
            Option<ClientChunkMsg>,
        ),
        Error,
    > {
        if !self.network_error.load(Ordering::Relaxed) {
            match select!(
                msg = self.default_stream.recv().fuse() => (Some(msg), None, None),
                msg = self.registration_stream.recv().fuse() => (None, Some(msg), None),
                msg = self.chunks_stream.recv().fuse() => (None, None, Some(msg)),
            ) {
                (Some(Ok(msg)), None, None) => Ok((Some(msg), None, None)),
                (None, Some(Ok(msg)), None) => Ok((None, Some(msg), None)),
                (None, None, Some(Ok(msg))) => Ok((None, None, Some(msg))),
                (Some(Err(e)), None, None)
                | (None, Some(Err(e)), None)
                | (None, None, Some(Err(e))) => {
                    debug!(?e, "got a network error with client while recv");
                    self.network_error.store(true, Ordering::Relaxed);
                    Err(Error::StreamErr(e))
                },
                _ => unreachable!("Cannot return from select"),
            }
        } else {
            Err(Error::StreamErr(network::StreamError::StreamClosed))
        }
    }

    pub fn is_registered(&self) -> bool {
        match self.client_state {
            ClientState::Registered | ClientState::Spectator | ClientState::Character => true,
            _ => false,
        }
    }

    pub fn is_ingame(&self) -> bool {
        match self.client_state {
            ClientState::Spectator | ClientState::Character => true,
            _ => false,
        }
    }

    pub fn allow_state(&mut self, new_state: ClientState) {
        self.client_state = new_state;
        self.notify_state(ServerStateMsg::StateAnswer(Ok(new_state)));
    }

    pub fn error_state(&mut self, error: RequestStateError) {
        self.notify_state(ServerStateMsg::StateAnswer(Err((error, self.client_state))));
    }

    pub fn allow_state_register(&mut self, new_state: ClientState) {
        self.client_state = new_state;
        self.notify_state(ServerStateMsg::StateAnswer(Ok(new_state)));
    }

    pub fn error_state_register(&mut self, error: RequestStateError) {
        self.notify_state(ServerStateMsg::StateAnswer(Err((error, self.client_state))));
    }
}

// Distance from fuzzy_chunk before snapping to current chunk
pub const CHUNK_FUZZ: u32 = 2;
// Distance out of the range of a region before removing it from subscriptions
pub const REGION_FUZZ: u32 = 16;

#[derive(Clone, Debug)]
pub struct RegionSubscription {
    pub fuzzy_chunk: Vec2<i32>,
    pub regions: HashSet<Vec2<i32>>,
}

impl Component for RegionSubscription {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>;
}
