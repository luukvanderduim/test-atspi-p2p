# test-atspi-p2p

The P2P feature of [`atspi`](<http://github.com/odilia-app/atspi.git>) spawns a task that listens for [`NameOwnerChanged`](<https://dbus.freedesktop.org/doc/dbus-java/api/index.html?org/freedesktop/DBus.NameOwnerChanged.html>) events.

This `DBus` signal is used to update the `Peers` list - applications that support P2P - in `AccessibilityConnection`.

It launches a known supported application or one supplied at the command line.
It then verifies whether the application gets added to the peers list.
Then it terminates the application and verifies whether it gets removed from the list.

## Currently not verified

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

## Command line arguments

```console
Usage: test-atspi-p2p [<child_name>] [-v] [-s <sleep>]

Command line arguments for the application

Positional Arguments:
  child_name        name of the child process to launch

Options:
  -v, --verbose     enable verbose output from child process
  -s, --sleep       configure the sleep time in seconds between child launch and
                    assertions
  --help, help      display usage information
```

## Example output

```shell
/test-atspi-p2p (main)> cargo run -- mate-calc
    Finished `dev` profile [unoptimized] target(s) in 0.03s
     Running `target/debug/test-atspi-p2p mate-calc`
2025-07-18T15:10:45.534154Z  INFO main ThreadId(01) CI(p2p): Set session accessibility to true
2025-07-18T15:10:45.538490Z  INFO main ThreadId(01) CI(p2p): Create accessibility connection
2025-07-18T15:10:45.540512Z  INFO main ThreadId(01) new: Connecting to a11y bus
2025-07-18T15:10:45.541409Z  INFO main ThreadId(01) new: Connected to a11y bus name=":1.39"
2025-07-18T15:10:45.568757Z  INFO main ThreadId(01) CI(p2p): Launching child process "mate-calc"
2025-07-18T15:10:45.688332Z  INFO tokio-runtime-worker ThreadId(19) Inserted unique name: :1.40 into the peer list.
2025-07-18T15:10:46.606777Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 24)
Peer: ":1.29", human readable name: "code-insiders"
Peer: ":1.32", human readable name: "element-desktop"
Peer: ":1.40", human readable name: "mate-calc"

2025-07-18T15:10:46.606816Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer insertion assertion passed
2025-07-18T15:10:47.608781Z  INFO                 main ThreadId(01) CI(p2p): Terminating "mate-calc"
2025-07-18T15:10:47.613761Z  INFO tokio-runtime-worker ThreadId(19) Peer with unique name: :1.40 left the bus - removed from peer list.
2025-07-18T15:10:48.627097Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 23)
Peer: ":1.28", human readable name: "Firefox"
Peer: ":1.29", human readable name: "code-insiders"
Peer: ":1.32", human readable name: "element-desktop"

2025-07-18T15:10:48.627137Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer removal assertion passed
2025-07-18T15:10:48.627141Z  INFO                 main ThreadId(01) CI(p2p): ✅ All assertions passed, exiting

```
