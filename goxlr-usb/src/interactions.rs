// This file primarily handles interactions with the GoXLR, we track the basic state of buttons,
// (pressed / unpressed) and (old value / new value), and send events upstream to handle them.

// We're not responsible for any 'Hold' behaviours here, as they aren't technically GoXLR related
// functionality, instead are handled upstream.

// Nor do we need to care about what fader is assigned to what, nor it's volume. All we care about
// here is whether something has physically changed on the GoXLR.

use enum_map::EnumMap;
use goxlr_shared::interaction::{
    ButtonState, InteractiveButtons, InteractiveEncoders, InteractiveFaders,
};
use tokio::sync::mpsc;

struct GoXLRStateTracker {
    buttonStates: EnumMap<InteractiveButtons, ButtonState>,
    volumeMap: EnumMap<InteractiveFaders, u8>,
    encoderMap: EnumMap<InteractiveEncoders, u8>,

    // TODO: We need independent receivers for this struct, as well as an upstream sender.
    // Under Linux, the receiver will be the timed poller, and under Windows the receiver will
    // handle events received from TUSB. This class need to be generic, and simply handle the IO
    // from other locations.
    sender: mpsc::Sender<ChangeEvent>,
    receiver: mpsc::Receiver<ChangeEvent>,
}

// This needs to be platform specific, on Windows we use the TUSB event handler to trigger events,
// where as under Linux, we use RUSB polling for events. We need to cleanly support both.

impl GoXLRStateTracker {
    pub fn new(receiver: mpsc::Receiver<ChangeEvent>, sender: mpsc::Sender<ChangeEvent>) -> Self {
        Self {
            sender,
            receiver,
            ..Default::default()
        }
    }
}

// It's important not to map these together, under Linux with polling the 'Incoming' change may
// match the existing value, we need to only trigger an outgoing change if the incoming != the
// current value.
enum ChangeEvent {
    ButtonDown(InteractiveButtons),
    ButtonUp(InteractiveButtons),
    VolumeChange(InteractiveFaders, u8),
    EncoderChange(InteractiveEncoders, i8),
}
