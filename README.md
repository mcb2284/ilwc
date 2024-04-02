# Internet Long Wave Chat

![GitHub all releases](https://img.shields.io/github/downloads/mcb2284/ilwc/total?color=green)
![GitHub language count](https://img.shields.io/github/languages/count/mcb2284/ilwc)
![GitHub top language](https://img.shields.io/github/languages/top/mcb2284/ilwc)
![Bitbucket open issues](https://img.shields.io/bitbucket/issues/mcb2284/ilwc)

Welcome to Internet Long Wave Chat, an innovative command-line interface (CLI) chat application built with Rust. Leveraging the power of asynchronous programming, our app offers real-time communication across the internet with minimal latency. Designed for developers and tech enthusiasts, Internet Long Wave Chat combines a robust architecture with an intuitive user experience, making digital conversations more seamless than ever.

## Key Features

- **Real-time communication**: Enjoy live chat capabilities with users around the globe.
- **Rust-based**: Built with Rust for safety, speed, and high performance.
- **Asynchronous runtime**: Utilizes Tokio, Rust's asynchronous runtime, to handle concurrent tasks efficiently.
- **Flexible protocol**: Employs SocketIO for real-time bidirectional event-based communication.

## Architecture Overview

Our application's architecture is designed to be both robust and efficient, ensuring a seamless chat experience. Below, you'll find an overview of the main components that make Internet Long Wave Chat a reliable and fast CLI chat app.

### Runtime

Internet Long Wave Chat is powered by [Tokio](https://tokio.rs/), a Rust asynchronous runtime designed for building fast and scalable network applications. Tokio provides the foundation for our app's non-blocking I/O operations, enabling it to handle thousands of concurrent connections without compromising performance.

- **Tokio on GitHub**: [View Repository](https://docs.rs/tokio/latest/tokio/)

### Protocol

For real-time communication, our app utilizes the SocketIO protocol, known for its efficiency in real-time bidirectional event-based communication.

- **SocketIO Documentation**: [Learn More](https://socket.io/docs/v4/)

#### Client

The client-side of Internet Long Wave Chat is built on `rust-socketio`, a Rust client for SocketIO, facilitating easy and efficient communication with the server.

- **rust-socketio on GitHub**: [View Repository](https://github.com/1c3t3a/rust-socketio)
- **rust-socketio Documentation**: [Read Docs](https://docs.rs/rust_socketio/latest/rust_socketio/)

#### Server

For the server-side, we utilize `socketioxide`, a Rust implementation of the SocketIO server. It's designed to work seamlessly with our Rust-based client, providing a sturdy backend for managing connections and data transmission.

- **socketioxide on GitHub**: [View Repository](https://github.com/Totodore/socketioxide?tab=readme-ov-file)
- **socketioxide Documentation**: [Read Docs](https://docs.rs/socketioxide/0.12.0/socketioxide/)

---

This README aims to provide a clear and concise overview of the Internet Long Wave Chat application, its core architecture, and the technologies it employs. Whether you're a developer interested in contributing or a user eager to dive into real-time chat, this guide is your starting point for exploring the capabilities of Internet Long Wave Chat.
