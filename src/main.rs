use std::sync::Arc;
use std::time::Duration;

use async_lock::Mutex;
use atspi::ObjectRef;
use atspi::connection::AccessibilityConnection;
use atspi::connection::P2P;
use atspi::connection::Peer;
use atspi::proxy::accessible::ObjectRefExt;
use tracing::info;
use tracing_subscriber::fmt;
use zbus::names::OwnedBusName;
use zbus::names::OwnedUniqueName;
use zbus::zvariant::ObjectPath;

static APP_NAME_PRE: &str = "evince";
static APP_ARG_PRE: &str = "test.pdf";
static APP_NAME: &str = "gedit";
static APP_ARG: &str = "test.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create test files
    std::fs::write("test.pdf", b"dummy pdf content")?;
    std::fs::write("test.txt", b"dummy text content")?;

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

    info!("CI(p2p): Set session accessibility to true");
    atspi::connection::set_session_accessibility(true)
        .await
        .expect("Failed to set session accessibility");

    info!("CI(p2p): Create accessibility connection");
    let a11y = AccessibilityConnection::new()
        .await
        .expect("Failed to create accessibility connection");

    let peers = a11y.peers().await;

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    info!("CI(p2p): Launching first child process \"{APP_NAME_PRE}\"");
    let mut child_process_pre = launch_child(APP_NAME_PRE, Some(APP_ARG_PRE), true);

    // Sleep to allow the first app to register
    tokio::time::sleep(Duration::from_secs(2)).await;

    let last_busname_before = peers
        .lock()
        .await
        .last()
        .map(|p| p.unique_name().to_string())
        .unwrap_or_else(|| "Empty peer list".to_string());

    info!("CI(p2p): Launching second child process \"{APP_NAME}\"");
    let mut child_process = launch_child(APP_NAME, Some(APP_ARG), true);

    // Registry needs a bit of time to populate with the new app
    tokio::time::sleep(Duration::from_secs(2)).await;

    let launched_busname = peers
        .lock()
        .await
        .last()
        .map(|p| p.unique_name())
        .unwrap()
        .clone();
    let launched_human_readable = to_human_readable(&launched_busname, &a11y).await;

    assert_eq!(
        launched_human_readable.to_lowercase(),
        APP_NAME,
        "The launched app's name should match \"{APP_NAME}\", but got: \"{launched_human_readable}\""
    );
    info!("CI(p2p): ✅ Peer insert assertion passed");

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("CI(p2p): Terminating \"{APP_NAME}\"");

    child_process.kill().expect("Failed to kill process");
    child_process.wait().expect("Failed to wait on process");
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    let last_busname_after = peers
        .lock()
        .await
        .last()
        .map(|p| p.unique_name().to_string())
        .unwrap_or_else(|| "Empty peer list".to_string());

    assert_eq!(
        last_busname_before, last_busname_after,
        "The last peer before launch and after termination should be the same, \
         but they differ: before: \"{last_busname_before}\", after: \"{last_busname_after}\""
    );
    info!("CI(p2p): ✅ Peer removal assertion passed");

    info!("CI(p2p): Terminating \"{APP_NAME_PRE}\"");
    child_process_pre.kill().expect("Failed to kill process");
    child_process_pre.wait().expect("Failed to wait on process");

    info!("CI(p2p): ✅ All assertions passed, exiting");
    Ok(())
}

async fn print_peers(
    peers: Arc<Mutex<Vec<Peer>>>,
    bus_name_to_natural_name: &[(OwnedBusName, String)],
) {
    println!("Last three `Peers`:");
    let peers = peers.lock().await;

    let peers_to_print = &*peers.iter().rev().take(3).collect::<Vec<&Peer>>();

    for peer in peers_to_print.iter().rev() {
        let mut well_known_name = String::from("None");
        if let Some(name) = peer.well_known_name() {
            well_known_name = name.to_string();
        }

        let human_readable = bus_name_to_natural_name
            .iter()
            .find(|(bus_name, _)| **bus_name == **peer.unique_name())
            .map(|(_, name)| name.clone());

        println!(
            "Peer: \"{}\", well-known name: {:?}. human readable name: \"{}\"",
            peer.unique_name(),
            well_known_name,
            human_readable.unwrap_or_else(|| "Unknown".to_string())
        );
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

// Transforms a single bus name to a human readable name
async fn to_human_readable(bus_name: &OwnedUniqueName, a11y: &AccessibilityConnection) -> String {
    static ROOT_PATH: ObjectPath<'static> =
        ObjectPath::from_static_str_unchecked("/org/a11y/atspi/accessible/root");
    let conn = a11y.connection();

    let root_object = ObjectRef::new(bus_name.as_ref(), ROOT_PATH.clone());
    let root_accessible = root_object
        .as_accessible_proxy(conn)
        .await
        .expect("Failed to get root accessible");

    root_accessible
        .name()
        .await
        .expect("Failed to get name of root accessible")
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
