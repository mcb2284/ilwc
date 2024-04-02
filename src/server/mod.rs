use serde::{Deserialize, Serialize};
use socketioxide::{extract::Data, extract::SocketRef, SocketIo};

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    name: String,
    age: u8,
}

pub async fn server() {
    println!("Server is running...");

    let (_, io) = SocketIo::new_svc();

    io.ns("/", |socket: SocketRef| {
        // Register a handler for the "test" event and extract the data as a `MyData` struct
        // With the Data extractor, the handler is called only if the data can be deserialized as a `MyData` struct
        // If you want to manage errors yourself you can use the TryData extractor
        socket.on("test", |socket: SocketRef, Data::<MyData>(data)| {
            println!("Received a test message {:?}", data);
            socket
                .emit(
                    "test-test",
                    MyData {
                        name: "Test".to_string(),
                        age: 8,
                    },
                )
                .ok(); // Emit a message to the client
        });
    });
    // let (layer, io) = SocketIo::new_layer();

    // // Register a handler for the default namespace
    // io.ns("/", |s: SocketRef| {
    //     // For each "message" event received, send a "message-back" event with the "Hello World!" event
    //     s.on("message", |s: SocketRef| {
    //         s.emit("message-back", "Hello World!").ok();
    //     });
    // });
}
