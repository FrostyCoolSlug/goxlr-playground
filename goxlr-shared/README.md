# GoXLR Shared

This crate is designed to primarily include items that are 'shared' between crates (such as the daemon, and USB), it's
not intended for things which are exclusive to the Daemon (such as IPC structs).

If the USB implementation is slightly different from the definitions here, an Into trait should be used for conversion
(see `goxlr-usb/src/channels.rs` for an example).