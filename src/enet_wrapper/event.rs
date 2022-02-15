#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::{packet::ENetPacket, peer::ENetPeer};
use enet_sys::{
    _ENetEvent, _ENetEventType_ENET_EVENT_TYPE_CONNECT, _ENetEventType_ENET_EVENT_TYPE_DISCONNECT,
    _ENetEventType_ENET_EVENT_TYPE_NONE, _ENetEventType_ENET_EVENT_TYPE_RECEIVE,
};

pub enum ENetEventType {
    None,
    Connect(ENetPeer),
    Disconnect(ENetPeer),
    Receive(ENetPeer, ENetPacket),
}

impl ENetEventType {
    pub fn new(event: &_ENetEvent) -> Option<ENetEventType> {
        match event.type_ {
            _ENetEventType_ENET_EVENT_TYPE_NONE => None,

            _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                Some(ENetEventType::Connect(ENetPeer::new(event.peer)))
            }

            _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                Some(ENetEventType::Disconnect(ENetPeer::new(event.peer)))
            }

            _ENetEventType_ENET_EVENT_TYPE_RECEIVE => Some(ENetEventType::Receive(
                ENetPeer::new(event.peer),
                ENetPacket::new(event.packet),
            )),

            _ => panic!("unrecognized event type: {}", event.type_),
        }
    }
}
