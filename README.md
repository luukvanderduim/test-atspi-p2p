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

```bash
~/c/test-atspi-p2p (main)> cargo r
    Finished `dev` profile [unoptimized] target(s) in 0.37s
     Running `target/debug/test-atspi-p2p`
Starting accessibility connection with P2P support
2025-07-09T13:49:11.408336Z  INFO main ThreadId(01) new: Connecting to a11y bus
2025-07-09T13:49:11.408862Z  INFO main ThreadId(01) new: Connected to a11y bus name=":1.59"
Last three `Peers`:
Peer: ":1.23", well-known name: "None". human readable name: "blueman-applet"
Peer: ":1.25", well-known name: "None". human readable name: "blueman-tray"
Peer: ":1.31", well-known name: "None". human readable name: "code-insiders"
Launching "firefox"
2025-07-09T13:49:12.173975Z  INFO tokio-runtime-worker ThreadId(20) Inserted unique name: :1.60 into the peer list.
Last three `Peers`:
Peer: ":1.25", well-known name: "None". human readable name: "blueman-tray"
Peer: ":1.31", well-known name: "None". human readable name: "code-insiders"
Peer: ":1.60", well-known name: "None". human readable name: "Firefox"
Terminating "firefox"
2025-07-09T13:49:15.468729Z  INFO tokio-runtime-worker ThreadId(20) Peer with unique name: :1.60 left the bus - removed from peer list.
Last three `Peers`:
Peer: ":1.23", well-known name: "None". human readable name: "blueman-applet"
Peer: ":1.25", well-known name: "None". human readable name: "blueman-tray"
Peer: ":1.31", well-known name: "None". human readable name: "code-insiders"
done
```
