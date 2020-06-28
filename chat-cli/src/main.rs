#![deny(unsafe_code)]
#![allow(clippy::option_map_unit_fn)]

use client::{Client, Event};
use common::{clock::Clock, comp, net::PostError};
use std::{
    io,
    net::ToSocketAddrs,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};
use tracing::{error, info};

const TPS: u64 = 10; // Low value is okay, just reading messages.
const DEFAULT_PORT: u16 = 14004;

fn read_input() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");

    buffer.trim().to_string()
}

fn main() {
    // Initialize logging.
    tracing_subscriber::fmt::init();

    info!("Starting chat-cli...");

    let mut server_addr = read_server_address();

    // Prompt for user/password until successful login, if a network error occurs
    // during login then prompt for a new server address
    let mut client = None;
    while client.is_none() {
        match attempt_login(&server_addr) {
            Ok(c) => {
                println!("Login successful, connected!");
                client = Some(c)
            },
            Err(LoginError::Client(e)) => {
                println!("Client error: {:?}", e);
            },
            Err(LoginError::Network(e)) => {
                println!("Network error: {:?}", e);
                server_addr = read_server_address();
            },
            Err(LoginError::Other(e)) => {
                println!("Error: {}", e);
            },
        };
    }

    let mut client = client.unwrap();
    println!("Server info: {:?}", client.server_info);

    // Set up an fps clock.
    let mut clock = Clock::start();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let msg = read_input();
            tx.send(msg).unwrap();
        }
    });

    loop {
        for msg in rx.try_iter() {
            client.send_chat(msg)
        }

        let events = match client.tick(
            comp::ControllerInputs::default(),
            clock.get_last_delta(),
            |_| {},
        ) {
            Ok(events) => events,
            Err(err) => {
                error!("Error: {:?}", err);
                break;
            },
        };

        for event in events {
            match event {
                Event::Chat { message, .. } => println!("{}", message),
                Event::Disconnect => {}, // TODO
                Event::DisconnectionNotification(time) => {
                    let message = match time {
                        0 => String::from("Goodbye!"),
                        _ => format!("Connection lost. Kicking in {} seconds", time),
                    };

                    println!("{}", message)
                },
                Event::InitialPlayerListReceived => {
                    println!("Online players: {:?}", client.get_players());
                },
                _ => {},
            }
        }

        // Clean up the server after a tick.
        client.cleanup();

        // Wait for the next tick.
        clock.tick(Duration::from_millis(1000 / TPS));
    }
}

fn read_server_address() -> String {
    println!("Enter the server address:");
    let mut server_addr = read_input();

    // Use default port if none specified
    if !server_addr.contains(':') {
        server_addr.push_str(format!(":{}", DEFAULT_PORT).as_str());
    }

    server_addr
}

enum LoginError {
    Network(Arc<io::Error>),
    Other(String),
    Client(client::Error),
}

impl From<client::Error> for LoginError {
    fn from(err: client::Error) -> LoginError { LoginError::Client(err) }
}

fn attempt_login(server_addr: &str) -> Result<Client, LoginError> {
    println!("Enter your username:");
    let username = read_input();
    println!("Enter your password:");
    let password = read_input();

    // Create a client
    let mut client = Client::new(
        server_addr
            .to_socket_addrs()
            .map_err(|e| LoginError::Network(Arc::new(e)))?
            .next()
            .unwrap(),
        None,
    )
    .map_err(|e| match e {
        client::Error::Network(post_error) => match post_error {
            PostError::Io(err) => LoginError::Network(err),
            _ => LoginError::Other(format!("{:?}", post_error)),
        },
        _ => LoginError::Client(e),
    })?;

    // Attempt login using the provided username/password
    client.register(username, password, |provider| {
        provider == "https://auth.veloren.net"
    })?;

    Ok(client)
}
