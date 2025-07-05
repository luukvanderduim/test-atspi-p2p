use std::sync::Arc;

use async_lock::Mutex;
use atspi::connection;
use atspi::connection::AccessibilityConnection;
use atspi::connection::P2P;
use atspi::connection::Peer;

use tracing_subscriber::fmt;

static APP_NAME: &str = "gedit";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt().event_format(format).init();

    println!("Starting accessibility connection with P2P support");
    connection::set_session_accessibility(true).await.unwrap();

    let a11y = AccessibilityConnection::new()
        .await
        .expect("Failed to create accessibility connection");

    let peers = a11y.peers().await;

    print_peers(peers.clone()).await;

    println!("launching \"{APP_NAME}\"");
    let mut child_process = std::process::Command::new(APP_NAME)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to launch terminal");

    std::thread::sleep(std::time::Duration::from_millis(3000));
    print_peers(peers.clone()).await;

    std::thread::sleep(std::time::Duration::from_millis(3000));
    println!("terminating \"{APP_NAME}\"");

    child_process.kill().expect("Failed to kill process");
    child_process.wait().expect("Failed to wait on process");

    std::thread::sleep(std::time::Duration::from_millis(3000));
    print_peers(peers.clone()).await;

    println!("done");
    Ok(())
}

async fn print_peers(peers: Arc<Mutex<Vec<Peer>>>) {
    println!("Last three `Peers`:");
    let peers = peers.lock().await;

    let peers_to_print = &*peers.iter().rev().take(3).collect::<Vec<&Peer>>();

    for peer in peers_to_print.iter().rev() {
        let mut well_known_name = String::from("None");
        if let Some(name) = peer.well_known_name() {
            well_known_name = name.to_string();
        }

        println!(
            "Peer: \"{}\", well-known name: {:?}. address: \"{}\"",
            peer.unique_name(),
            well_known_name,
            peer.socket_address()
        );
    }
}
