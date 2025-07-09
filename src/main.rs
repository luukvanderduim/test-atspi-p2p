use std::sync::Arc;

use async_lock::Mutex;
use atspi::ObjectRef;
use atspi::connection;
use atspi::connection::AccessibilityConnection;
use atspi::connection::P2P;
use atspi::connection::Peer;
use atspi::proxy::accessible::ObjectRefExt;
use pretty_assertions::assert_eq;
use tracing_subscriber::fmt;
use zbus::names::OwnedBusName;
use zbus::names::OwnedUniqueName;
use zbus::zvariant::ObjectPath;

static APP_NAME: &str = "firefox";
static APP_ARG: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    atspi::connection::set_session_accessibility(true)
        .await
        .expect("Failed to set session accessibility");

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

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    let last_busname_before = peers
        .lock()
        .await
        .last()
        .map(|p| p.unique_name().to_string())
        .unwrap_or_else(|| "None".to_string());

    println!("Launching \"{APP_NAME}\"");
    let mut child_process = std::process::Command::new(APP_NAME)
        .arg(APP_ARG)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to launch terminal");

    // Registry needs a bit of time to populate with the new app
    std::thread::sleep(std::time::Duration::from_secs(2));

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
        "The launched app's name should match the app name, but got: \"{launched_human_readable}\""
    );

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Terminating \"{APP_NAME}\"");

    child_process.kill().expect("Failed to kill process");
    child_process.wait().expect("Failed to wait on process");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let mapping = bus_names_to_human_readable(&a11y).await;
    print_peers(peers.clone(), &mapping).await;

    let last_busname_after = peers
        .lock()
        .await
        .last()
        .map(|p| p.unique_name().to_string())
        .unwrap_or_else(|| "None".to_string());

    assert_eq!(
        last_busname_before, last_busname_after,
        "The last peer before launch and after termination should be the same, \
         but they differ: before: \"{last_busname_before}\", after: \"{last_busname_after}\""
    );

    println!("done");
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
