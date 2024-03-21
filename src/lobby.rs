use std::collections::{HashMap, HashSet};

use actix::prelude::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use crate::messages::{
    ClientActorMessage, Connect, Content, Disconnect, SyncMsg, SyncType, WsMessage,
};

type Socket = Recipient<WsMessage>;

const MOST_PAITIES: usize = 2;

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms:    HashMap<Uuid, HashSet<Uuid>>, // room id  to list of users id
}

impl Lobby {
    fn send_message(&self, message: Content, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            socket_recipient.do_send(WsMessage(message));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    let msg = serde_json::to_string(&SyncMsg {
                        type_:  SyncType::Leave,
                        device: msg.id.to_string(),
                    })
                    .unwrap();
                    self.send_message(Content::Text(msg), user_id)
                });

            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    // only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.self_id, msg.addr.clone());

        self.rooms
            .entry(msg.lobby_id)
            .or_default()
            .insert(msg.self_id);

        let room = self.rooms.get(&msg.lobby_id).unwrap();

        room
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                let msg = serde_json::to_string(&SyncMsg {
                    type_: SyncType::Join,
                    device: msg.self_id.to_string(),
                }).unwrap();
                self.send_message(Content::Text(msg), conn_id)
            });

        let uuid = serde_json::to_string(&SyncMsg {
            type_:  SyncType::GenUuid,
            device: msg.self_id.to_string(),
        })
        .unwrap();
        self.send_message(Content::Text(uuid), &msg.self_id);

        if room.len() == MOST_PAITIES {
            room
            .iter()
            .for_each(|conn_id| {
                let msg = serde_json::to_string(&SyncMsg {
                    type_: SyncType::Start,
                    ..Default::default()
                }).unwrap();
                self.send_message(Content::Text(msg), conn_id);
            });
        }
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let room_id = msg.room_id;
        let msg = Content::Binary(msg.msg);

        self.rooms
            .get(&room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(msg.clone(), client));
    }
}
