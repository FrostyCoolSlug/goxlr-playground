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

struct GoXLRState {
    buttonStates: EnumMap<InteractiveButtons, ButtonState>,
    volumeMap: EnumMap<InteractiveFaders, u8>,
    encoderMap: EnumMap<InteractiveEncoders, u8>,
}

// This needs to be platform specific, on Windows we use the TUSB event handler to trigger events,
// where as under Linux, we use RUSB polling for events. We need to cleanly support both.

impl GoXLRState {}
