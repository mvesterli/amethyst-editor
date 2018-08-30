use {EntityData, Message};
use amethyst::ecs::*;
use serde_json;
use std::net::UdpSocket;

#[derive(Debug, Clone, Copy, Default)]
pub struct SyncEditorSystem;

impl<'a> System<'a> for SyncEditorSystem {
    type SystemData = (
        ReadExpect<'a, UdpSocket>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (socket, entities) = data;

        let entity_data = {
            let mut result = Vec::new();
            for (entity,) in (&*entities,).join() {
                result.push(EntityData {
                    id: entity.id(),
                    generation: entity.gen().id(),
                });
            }
            result
        };

        // Create the message and serialize it to JSON.
        let message = Message {
            ty: "message",
            data: entity_data,
        };
        let mut message_string = serde_json::to_string(&message).expect("Failed to serialize message");

        // NOTE: We need to append a page feed character after each message since that's what node-ipc
        // expects to delimit messages.
        message_string.push_str("\u{C}");

        // Send the JSON message.
        socket.send(message_string.as_bytes()).expect("Failed to send message");
    }
}
