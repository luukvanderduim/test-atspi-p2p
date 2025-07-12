use async_lock::Mutex;
use atspi::connection::AccessibilityConnection;
use atspi::connection::P2P;
use atspi::connection::Peer;
use atspi::proxy::accessible::ObjectRefExt;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::fmt;
use zbus::names::OwnedBusName;

static APP_NAME: &str = "gedit";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Configure a custom tracing event formatter
    let format = fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt().event_format(format).init();

    info!("CI(p2p): Set session accessibility to true");
    atspi::connection::set_session_accessibility(true)
        .await
        .expect("Failed to set session accessibility");

    info!("CI(p2p): Create accessibility connection");
    let a11y = AccessibilityConnection::new()
        .await
        .expect("Failed to create accessibility connection");

    let peers = a11y.peers().await;

    info!("CI(p2p): Launching child process \"{APP_NAME}\"");
    let mut child_process = launch_child(APP_NAME, None, false);

    // Registry needs a bit of time to populate with the new app
    tokio::time::sleep(Duration::from_secs(1)).await;

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    // Assert that the second app is part of the makking
    assert!(
        mapping
            .iter()
            .any(|(_bus_name, human_readable_name)| human_readable_name
                .to_lowercase()
                .contains(APP_NAME)),
        "App \"{APP_NAME}\" not registered as P2P application in Peers list."
    );
    info!("CI(p2p): ✅ Peer insertion assertion passed");

    tokio::time::sleep(Duration::from_secs(1)).await;
    info!("CI(p2p): Terminating \"{APP_NAME}\"");

    // Termination and removal of app

    child_process.kill().expect("Failed to kill process");
    child_process.wait().expect("Failed to wait on process");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    // Assert that the app is no longer part of the makking
    assert!(
        !mapping
            .iter()
            .any(|(_bus_name, human_readable_name)| human_readable_name
                .to_lowercase()
                .contains(APP_NAME)),
        "App \"{APP_NAME}\" not removed as P2P application from Peers list."
    );
    info!("CI(p2p): ✅ Peer removal assertion passed");

    info!("CI(p2p): ✅ All assertions passed, exiting");
    Ok(())
}

// If there are more then three peers, prints the last three peers
// Let's indicate that there are more peers by printing "..." if there are more than three
// We also print the human readable name if available and print pretty information about the peer
async fn print_peers(peers: Arc<Mutex<Vec<Peer>>>, mapping: &[(OwnedBusName, String)]) {
    info!("CI(p2p): Printing peers...");

    let peers_locked = peers.lock().await;
    let total_peers = peers_locked.len();

    // The take` adaptor will take maximum N elements from left to right, so we reverse the iterator first
    let last_peers: Vec<&Peer> = peers_locked.iter().rev().take(3).collect();

    // If there are more than 3 peers, indicate there are more
    if total_peers > 3 {
        println!("Peer: ... (total: {total_peers})");
    }

    // Print in reverse to restore chronological order
    for peer in last_peers.iter().rev() {
        let unique_name = peer.unique_name();

        // Look up the human-readable name from the mapping
        // Mapping may be longer than `last_peers`
        let human_readable = mapping
            .iter()
            .find_map(|(bus_name, name)| {
                if bus_name.as_str() == unique_name.as_str() {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "not found".to_string());

        println!("Peer: \"{unique_name}\", human readable name: \"{human_readable}\"");
    }

    if total_peers == 0 {
        info!("CI(p2p): No peers found");
    } else {
        println!();
    }
}

// Gets the accessible apps in registry, returns a mapping of bus names to human readable names
async fn bus_names_to_human_readable(
    a11y: &AccessibilityConnection,
) -> Vec<(OwnedBusName, String)> {
    let conn = a11y.connection();

    let registry_accessible = a11y
        .root_accessible_on_registry()
        .await
        .expect("Failed to get root accessible on registry");

    let children = registry_accessible
        .get_children()
        .await
        .expect("Failed to get children of root accessible");

    // Create a mapping of bus_names to human readable names
    let mut bus_name_to_human_readable: Vec<(OwnedBusName, String)> = Vec::new();

    for child in children {
        let ap = child
            .as_accessible_proxy(conn)
            .await
            .expect("Failed to get accessible proxy");

        let natural_name = ap.name().await.expect("Failed to get name");
        let bus_name: OwnedBusName = ap.inner().destination().to_owned().into();
        bus_name_to_human_readable.push((bus_name, natural_name));
    }

    bus_name_to_human_readable
}

fn launch_child(child_name: &str, child_arg: Option<&str>, noisy: bool) -> std::process::Child {
    let mut command = std::process::Command::new(child_name);
    if let Some(arg) = child_arg {
        command.arg(arg);
    }

    // With inherit() - child output mixes with parent output
    // With null() - only parent output appears, child is silent
    if noisy {
        command
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
    } else {
        command
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
    }

    command.spawn().expect("Failed to launch child process")
}
