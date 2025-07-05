# test-atspi-p2p

This tests the `p2p` branch of [`atspi`](<http://github.com/odilia-app/atspi.git>)

Currently it:

- subscribes to tracing messages
- shows the last three applications in the p2p `Peers` list of the connection
- starts an application (gedit)
- prints the last three applications
- terminates the application a few seconds later.
- prints the last three applications

This way one can see that chreation of initial peers works as well as insertion and deletion when new peers appear on the bus.

## Example output

```bash
~/c/test-atspi-p2p (main)> cargo r
    Finished `dev` profile [unoptimized] target(s) in 0.06s
     Running `target/debug/test-atspi-p2p`
Starting accessibility connection with P2P support
2025-07-05T15:03:46.245981Z  INFO main ThreadId(01) new: Connecting to a11y bus
2025-07-05T15:03:46.247151Z  INFO main ThreadId(01) new: Connected to a11y bus name=":1.81"
Last three `Peers`:
Peer: ":1.30", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-XYU482/socket"
Peer: ":1.36", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-DGC582/socket"
Peer: ":1.69", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-V96X82/socket"
launching "gedit"
2025-07-05T15:03:46.551893Z  INFO zbus::Connection executor ThreadId(28) Inserted unique name: :1.82 into the peer list.
Last three `Peers`:
Peer: ":1.36", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-DGC582/socket"
Peer: ":1.69", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-V96X82/socket"
Peer: ":1.82", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-NRUW82/socket"
terminating "gedit"
2025-07-05T15:03:52.320056Z  INFO zbus::Connection executor ThreadId(28) Peer with unique name: :1.82 left the bus.
Last three `Peers`:
Peer: ":1.30", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-XYU482/socket"
Peer: ":1.36", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-DGC582/socket"
Peer: ":1.69", well-known name: "None". address: "unix:path=/run/user/1000/at-spi2-V96X82/socket"
done

```
