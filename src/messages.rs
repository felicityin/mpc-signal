use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub Content);

#[derive(Clone)]
pub enum Content {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr:     Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id:  Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id:      Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id:      Uuid,
    pub msg:     Vec<u8>,
    pub room_id: Uuid,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SyncMsg {
    pub type_:  SyncType,
    pub device: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum SyncType {
    #[default]
    Join = 0,
    Leave = 1,
    GenUuid = 2,
    Start = 3,
}
