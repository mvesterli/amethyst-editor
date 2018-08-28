extern crate amethyst;
#[macro_use]
extern crate serde;
extern crate serde_json;

use amethyst::ecs::*;
use std::net::UdpSocket;
use std::str;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct EntityData {
    id: u32,
    generation: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message<T> {
    #[serde(rename = "type")]
    ty: &'static str,
    data: T,
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8001").expect("Failed to bind socket");
    socket.connect("127.0.0.1:8000").expect("Failed to connect to editor");

    // Create some entities in the world.
    let mut world = World::new();
    for _ in 0..10 {
        world.create_entity().build();
    }
    world.maintain();

    // Create a list of the entities in the world.
    let entity_data = {
        let entities = world.entities();
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

    let mut buffer = [0; 2048];
    loop {
        let bytes_read = socket.recv(&mut buffer).expect("Failed to recieve bytes");
        let message_bytes = &buffer[..bytes_read];
        let message = str::from_utf8(message_bytes).expect("Message was invalid UTF-8");
        println!("{}", message.trim());
    }
}
