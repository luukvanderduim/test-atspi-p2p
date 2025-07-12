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
2. Shows the last three applications in the `Peers` list
3. Launches an application and verifies it appears in the peer list
4. Terminates the application and verifies it gets removed from the peer list

This demonstrates that peer insertion and deletion work correctly when applications join and leave the accessibility bus.

## Example output

```shell
    Finished `dev` profile [unoptimized] target(s) in 0.35s
     Running `target/debug/test-atspi-p2p`
2025-07-12T15:35:14.435159Z  INFO main ThreadId(01) CI(p2p): Set session accessibility to true
2025-07-12T15:35:14.439176Z  INFO main ThreadId(01) CI(p2p): Create accessibility connection
2025-07-12T15:35:14.440801Z  INFO main ThreadId(01) new: Connecting to a11y bus
2025-07-12T15:35:14.441240Z  INFO main ThreadId(01) new: Connected to a11y bus name=":1.72"
2025-07-12T15:35:14.469968Z  INFO main ThreadId(01) CI(p2p): Launching first child process "gedit"
2025-07-12T15:35:14.711976Z  INFO tokio-runtime-worker ThreadId(07) Inserted unique name: :1.73 into the peer list.
2025-07-12T15:35:15.480746Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer insertion assertion passed
2025-07-12T15:35:15.480786Z  INFO                 main ThreadId(01) CI(p2p): Launching second child process "eog"
2025-07-12T15:35:15.626576Z  INFO tokio-runtime-worker ThreadId(21) Inserted unique name: :1.74 into the peer list.
2025-07-12T15:35:16.495014Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 26)
Peer: ":1.65", human readable name: "Firefox"
Peer: ":1.73", human readable name: "gedit"
Peer: ":1.74", human readable name: "eog"

2025-07-12T15:35:16.495062Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer insertion assertion passed
2025-07-12T15:35:17.495762Z  INFO                 main ThreadId(01) CI(p2p): Terminating "eog"
2025-07-12T15:35:17.499494Z  INFO tokio-runtime-worker ThreadId(21) Peer with unique name: :1.74 left the bus - removed from peer list.
2025-07-12T15:35:18.511014Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 25)
Peer: ":1.42", human readable name: "signal-desktop"
Peer: ":1.65", human readable name: "Firefox"
Peer: ":1.73", human readable name: "gedit"

2025-07-12T15:35:18.511062Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer removal assertion passed
2025-07-12T15:35:18.511067Z  INFO                 main ThreadId(01) CI(p2p): Terminating "gedit"
2025-07-12T15:35:18.515983Z  INFO tokio-runtime-worker ThreadId(21) Peer with unique name: :1.73 left the bus - removed from peer list.
2025-07-12T15:35:19.526207Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer removal assertion passed
2025-07-12T15:35:19.526227Z  INFO                 main ThreadId(01) CI(p2p): ✅ All assertions passed, exiting
```
