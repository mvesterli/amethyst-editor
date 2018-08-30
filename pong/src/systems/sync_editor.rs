use {EntityData, Message};
use amethyst::core::Transform;
use amethyst::ecs::*;
use serde_json;
use std::net::UdpSocket;

#[derive(Debug, Clone, Copy, Default)]
pub struct SyncEditorSystem;

#[derive(Debug, Serialize)]
struct Data<'a> {
    entities: Vec<EntityData>,
    transforms: Vec<(u32, &'a Transform)>,
}

impl<'a> System<'a> for SyncEditorSystem {
    type SystemData = (
        ReadExpect<'a, UdpSocket>,
        Entities<'a>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (socket, entities, transforms) = data;

        let mut entity_data = Vec::new();
        for (entity,) in (&*entities,).join() {
            entity_data.push(EntityData {
                id: entity.id(),
                generation: entity.gen().id(),
            });
        }

        let mut transform_data = Vec::new();
        for (entity, transform) in (&*entities, &transforms).join() {
            transform_data.push((entity.id(), transform));
        }

        // Create the message and serialize it to JSON.
        let message = Message {
            ty: "message",
            data: Data {
                entities: entity_data,
                transforms: transform_data,
            },
        };
        let mut message_string = serde_json::to_string(&message).expect("Failed to serialize message");

        // NOTE: We need to append a page feed character after each message since that's what node-ipc
        // expects to delimit messages.
        message_string.push_str("\u{C}");

        // Send the JSON message.
        socket.send(message_string.as_bytes()).expect("Failed to send message");
    }
}
