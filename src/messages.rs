use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Message, Serialize)]
#[rtype(result = "futures::channel::oneshot::Receiver<u8>")]
//#[rtype(result = "u8")]
pub struct MsgToActor1;

#[derive(Debug, Deserialize, Message, Serialize)]
#[rtype(result = "u8")]
pub struct MsgToActor2;

#[derive(Debug, Deserialize, Message, Serialize)]
#[rtype(result = "u8")]
pub struct MsgToActor3 {
    pub n: u8,
}
