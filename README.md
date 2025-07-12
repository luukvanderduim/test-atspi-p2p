# test-atspi-p2p

The P2P feature of [`atspi`](<http://github.com/odilia-app/atspi.git>) spawns a task that listens for [`NameOwnerChanged`](<https://dbus.freedesktop.org/doc/dbus-java/api/index.html?org/freedesktop/DBus.NameOwnerChanged.html>) events.

This `DBus` signal is used to update the `Peers` list - applications that support P2P - in `AccessibilityConnection`.

We start a known supported application, verify whether it gets added, then terminate the same application and verify whether it gets removed from the list.

What is currently not verified:

- Entry of a `WellKnownName`
- Transfer of ownership of a `WellKnownName`

To verify this behavior, we need an example of such an application.
Suggestions are welcome.

See: [`atspi`](http://github.com/odilia-app/atspi.git)

Currently it:

1. Subscribes to tracing messages
2. Shows the applications in the `Peers` list
3. Launches an application and verifies it appears in the peer list
4. Terminates the application and verifies it gets removed from the peer list

This demonstrates that peer insertion and deletion work correctly when applications join and leave the accessibility bus.

## Example output

```shell
    Compiling test-atspi-p2p v0.1.0 (/home/luuk/code/test-atspi-p2p)
    Finished `dev` profile [unoptimized] target(s) in 0.29s
     Running `target/debug/test-atspi-p2p`
2025-07-12T16:09:32.492830Z  INFO main ThreadId(01) CI(p2p): Set session accessibility to true
2025-07-12T16:09:32.495862Z  INFO main ThreadId(01) CI(p2p): Create accessibility connection
2025-07-12T16:09:32.497238Z  INFO main ThreadId(01) new: Connecting to a11y bus
2025-07-12T16:09:32.498035Z  INFO main ThreadId(01) new: Connected to a11y bus name=":1.88"
2025-07-12T16:09:32.532920Z  INFO main ThreadId(01) CI(p2p): Launching child process "gedit"
2025-07-12T16:09:32.773586Z  INFO tokio-runtime-worker ThreadId(16) Inserted unique name: :1.89 into the peer list.
2025-07-12T16:09:33.545410Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 25)
Peer: ":1.65", human readable name: "Firefox"
Peer: ":1.86", human readable name: "code-insiders"
Peer: ":1.89", human readable name: "gedit"

2025-07-12T16:09:33.545453Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer insertion assertion passed
2025-07-12T16:09:34.546743Z  INFO                 main ThreadId(01) CI(p2p): Terminating "gedit"
2025-07-12T16:09:34.556065Z  INFO tokio-runtime-worker ThreadId(16) Peer with unique name: :1.89 left the bus - removed from peer list.
2025-07-12T16:09:35.573602Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 24)
Peer: ":1.42", human readable name: "signal-desktop"
Peer: ":1.65", human readable name: "Firefox"
Peer: ":1.86", human readable name: "code-insiders"

2025-07-12T16:09:35.573683Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer removal assertion passed
2025-07-12T16:09:35.573698Z  INFO                 main ThreadId(01) CI(p2p): ✅ All assertions passed, exiting

```
