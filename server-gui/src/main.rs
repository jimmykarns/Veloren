#![deny(unsafe_code)]

use common::clock::Clock;
use druid::{
    widget::{Button, Flex, Label, List, Scroll, WidgetExt},
    AppLauncher, Data, Lens, Widget, WindowDesc,
};
use log::info;
use server::{Event, Input, Server, ServerSettings};
use std::{
    rc::Rc,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

enum Cmd {
    Start,
    Stop,
}

fn run_server(
    settings: ServerSettings,
    cmd_rx: mpsc::Receiver<Cmd>,
    event_tx: mpsc::Sender<Event>,
) {
    match cmd_rx.recv() {
        Ok(Cmd::Start) => {}
        _ => return,
    };

    // Set up an fps clock
    const TPS: u64 = 30;
    let mut clock = Clock::start();

    // Create server
    let mut server = Server::new(settings).expect("Failed to create server instance!");

    let mut running = true;
    while running {
        let events = server
            .tick(Input::default(), clock.get_last_delta())
            .expect("Failed to tick server");

        for event in events {
            let _ = event_tx.send(event);
        }

        for cmd in cmd_rx.try_iter() {
            match cmd {
                Cmd::Stop => running = false,
                _ => {}
            }
        }

        // Clean up the server after a tick.
        server.cleanup();

        // Wait for the next tick.
        clock.tick(Duration::from_millis(1000 / TPS));
    }
}

#[derive(Clone, Data, Lens)]
struct State {
    cmd_tx: Rc<mpsc::Sender<Cmd>>,
    event_rx: Rc<mpsc::Receiver<Event>>,
    log: Arc<Vec<String>>,
}

fn build_window() -> impl Widget<State> {
    let controls = Flex::row()
        .with_child(
            Button::new("Start", |ctx, state: &mut State, _| {
                ctx.set_active(false);
                let _ = state.cmd_tx.send(Cmd::Start);
                Arc::make_mut(&mut state.log).push("Started server.".to_string());
            }),
            1.0,
        )
        .with_child(
            Button::new("Stop", |ctx, state: &mut State, _| {
                Arc::make_mut(&mut state.log).push("Stopping server...".to_string());
                ctx.window().close();
            }),
            1.0,
        )
        .with_child(
            Button::new("Update", |_, state: &mut State, _| {
                let log = Arc::make_mut(&mut state.log);
                for event in state.event_rx.try_iter() {
                    match event {
                        Event::ClientConnected { entity: _ } => {
                            log.push(format!("Client connected!"))
                        }
                        Event::ClientDisconnected { entity: _ } => {
                            log.push(format!("Client disconnected!"))
                        }
                        Event::Chat { entity: _, msg } => log.push(format!("[Client] {}", msg)),
                    };
                }
            }),
            1.0,
        );

    let log_list = Scroll::new(List::new(|| {
        Label::new(|entry: &String, _: &_| entry.clone())
    }))
    .vertical()
    .lens(State::log);

    Flex::column()
        .with_child(controls, 0.1)
        .with_child(log_list, 1.0)
}

fn main() {
    // Init logging
    pretty_env_logger::init();

    info!("Starting server-gui...");

    // Load settings
    let settings = ServerSettings::load();

    let window = WindowDesc::new(build_window);

    let (cmd_tx, cmd_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();
    let worker = thread::spawn(|| run_server(settings, cmd_rx, event_tx));

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(State {
            cmd_tx: Rc::new(cmd_tx.clone()),
            event_rx: Rc::new(event_rx),
            log: Arc::new(Vec::new()),
        })
        .expect("Failed to launch application");

    cmd_tx.send(Cmd::Stop).unwrap();
    worker.join().expect("Server did not stop successfully");
}
