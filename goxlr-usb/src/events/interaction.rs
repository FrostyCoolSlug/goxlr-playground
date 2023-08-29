use goxlr_shared::interaction::{InteractiveButtons, InteractiveEncoders, InteractiveFaders};

#[derive(Debug)]
pub enum InteractionEvent {
    ButtonDown(InteractiveButtons),
    ButtonUp(InteractiveButtons),
    VolumeChange(InteractiveFaders, u8),
    EncoderChange(InteractiveEncoders, i8),
}
