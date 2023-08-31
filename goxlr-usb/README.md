# GoXLR USB Messaging

This crate is primarily responsible for all USB handling and messaging.

In the GoXLR Utility, the code had to be responsible for all end-to-end communication with the device, including 
initialisation, messaging, event handling etc. For Open GoXLR instead all control over the device itself is managed by
this crate, and uses messaging queues to notify of events, state changes, and to send / receive messages.

Be advised, this crate is receiving frequent updates, so the info here might be slightly out of date, if you spot an
error, open a bug report to let me know and I'll update it!

### Plug And Play Handler

Setting up the initial PnP handler, you need to create and spawn a task with a notifier:

```rust
let (pnp_stop_send, pnp_stop_recv) = oneshot::channel();
let (device_send, mut device_recv) = mpsc::channel(32);

let pnp_configuration = PnPConfiguration {
    stop_signal: pnp_stop_recv,
    device_sender: device_send,
};

let pnp = task::spawn(start_pnp_runner(pnp_configuration));
```

You can then simply sit on `device_recv` which will trigger when on device attach, or removal. If there are already
GoXLRs attached when `start_pnp_runner` is called, it'll trigger Attached for those devices. Both attach and removal 
come with a `USBLocation` that defines where the device is attached to the machine. These can be used in the next step.

`pnp_stop_send` can be used to instruct the PnP handler to stop, it's a oneshot command, so once it's sent it can not
be aborted. It's best to join on the handler to pause your codes execution until everything is shut down:

```rust
let _ = pnp_stop_send.send(());
let _ = join!(pnp);

// When you get here the handler has been closed.
```

*TODO*: Get Devices without spawning a PnP handler 

### Device Handler
Once you have a `USBLocation` it's time to spawn up a device handler. This is primarily responsible for sending messages
and handling device stuffs. It's a little more complicated than the PnP handler because there's a little more to take
care of, but as a basic starting point, you can go with something like:

```rust
// The Device we're setting up for (received from the PnP Handler)
let device: USBLocation = device;

// These are device specific messages sent to us by the handler..
let (event_send, mut event_recv) = mpsc::channel(16);

// This is the command channel, for sending commands, and receiving responses from the device
let (command_send, command_recv) = mpsc::channel(32);

// These are callbacks for physical interactions with the device (Buttons Pressed / Volumes Changed)
let (interaction_send, mut interaction_recv) = mpsc::channel(128);

// A signalling channel to tell the device workers to stop
let (stop_send, stop_recv) = oneshot::channel();

// A signal from the device runner to tell us it's ready to go.
let (ready_send, ready_recv) = oneshot::channel();

// Build the configuration for the USB Runner, with the relevant messaging queues
let configuration = GoXLRUSBConfiguration {
    device,
    interaction_event: Some(interaction_send),
    device_event: event_send,
    command_receiver: command_recv,
    stop: stop_recv,
};
let runner = task::spawn(start_usb_device_runner(configuration, ready_send));

// Use the ready signal to hold here, until the usb running is running, this will also
// provide us with the device info (such as serial, features, versions, etc).
let device = match ready_recv.await {
    Ok(recv) => recv,
    Err(e) => {
        bail!("Error on Starting Receiver, aborting: {}", e);
    }
};
```

While there are quite a few queues involved (5 total), they all have very specific usages, and we're primarily passing
the necessary senders or receivers in, as a quick breakdown:

`ready_recv`: When launching the device runner, it may need to perform certain tasks, such as initialising the device
or waiting for animations to finish, during which time you should not be sending commands in, or listening for any
events (none are sent). The ready receiver will simply wait until the runner has done any needed setup and will be
triggered with some more useful and specific device information.

`interaction_recv` (optional): This will instruct the USB crate to automatically watch for changes to the physical state
of the GoXLR (button press / release, volume and encoder changes). The receiver will simply receive a message indicating
the type of change, or the new value (where applicable).

`event_recv`: Informs of important events occurring on the GoXLR, these may be critical errors, or (if 
`interaction_recv` is set to `None`) notification that the state may have changed, and you need to re-check the button behaviours.

**NOTE**: Under Linux, due to not being able to listen for interrupts, the 'StatusChange' message will be triggered
every 20ms like clockwork. You should be prepared to handle it, and be weary of having code that stalls this receiver,
if in doubt, let the USB crate handle it for you.

`command_send`: This is the primary method of communicating with the GoXLR. All commands have a message and a oneshot
responder of the correct type. Commands which don't have a specific response (so Result<()>) are bundled into a
`BasicResultCommand` for ease of use. Commands that have responses to be handled have their own oneshot response format.

`stop_send`: Relatively simple, a oneshot command which will instruct the device handler to shutdown. As with the PnP
handler you should join onto the task and wait for its completion to ensure it's successfully shut down.