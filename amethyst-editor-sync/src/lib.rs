use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use std::net::UdpSocket;
use serde::Serialize;
use amethyst::ecs::*;
use serde::export::PhantomData;

extern crate amethyst;
extern crate crossbeam_channel;
#[macro_use]
extern crate serde;
extern crate serde_json;

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

#[derive(Debug, Clone)]
pub struct SyncComponentSystem<T> {
    name: &'static str,
    sender: Sender<String>,
    _marker: PhantomData<T>,
}

impl<T> SyncComponentSystem<T> {
    pub fn new(name: &'static str, send_to: &SyncEditorSystem) -> Self {
        SyncComponentSystem {
            name,
            sender: send_to.sender.clone(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> System<'a> for SyncComponentSystem<T> where T: Component + Serialize {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, T>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, transforms) = data;

        let mut entity_data = Vec::new();
        for (entity,) in (&*entities,).join() {
            entity_data.push(EntityData {
                id: entity.id(),
                generation: entity.gen().id(),
            });
        }

        let mut component_data = Vec::new();
        for (entity, transform) in (&*entities, &transforms).join() {
            component_data.push((entity.id(), transform));
        }
        let serialized_data = serde_json::to_string(&component_data).expect("Failed to serialize message");

        let formatted = format!(
            "\"{}\":{}",
            self.name,
            serialized_data,
        );
        println!("{}", formatted);

        self.sender.send(formatted);
    }
}

#[derive(Debug, Clone)]
pub struct SyncEditorSystem {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl SyncEditorSystem {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        SyncEditorSystem { sender, receiver }
    }
}


impl<'a> System<'a> for SyncEditorSystem {
    type SystemData = (
        ReadExpect<'a, UdpSocket>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (socket, entities) = data;

        let mut all_components = String::new();
        if let Some(component) = self.receiver.try_recv() {
            all_components = component;
        }

        while let Some(component_string) = self.receiver.try_recv() {
            all_components.push(',');
            all_components.push_str(&component_string);
        }

        let mut entity_data = Vec::new();
        for (entity,) in (&*entities,).join() {
            entity_data.push(EntityData {
                id: entity.id(),
                generation: entity.gen().id(),
            });
        }
        let entity_string = serde_json::to_string(&entity_data).expect("Failed to serialize entities");

        // Create the message and serialize it to JSON.
        let mut message_string = format!(
            r#"{{
                "type": "message",
                "data": {{
                    "entities": {},
                    "components": {{ {} }}
                }}
            }}"#,
            entity_string,
            all_components,
        );

        println!("{}", message_string);

        // NOTE: We need to append a page feed character after each message since that's what node-ipc
        // expects to delimit messages.
        message_string.push_str("\u{C}");

        // Send the JSON message.
        socket.send(message_string.as_bytes()).expect("Failed to send message");
    }
}
