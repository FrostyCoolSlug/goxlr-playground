# GoXLR USB Types

The primary purpose of this module is to safely convert types which are defined in `goxlr-shared` 
into types usable by USB.

The logic behind this is to allow for more expressive type handling in implementations which we 
can wrap cleanly into our own internal structures (`channels.rs` is a good example of having multiple
external types, that all come home eventually).

This also means that we don't have to be quite as careful in `goxlr-shared` when it comes to moving,
renaming and reordering structs, the USB crate will always take care of making sure they're correct
before sending them to the GoXLR, take the following example from the util:

```rust
// Defined in goxlr-utility/types/lib.rs
enum Fader {
    A,
    B,
    C,
    D
}
```

What's not clear here, is that ordering is *VITALLY* important, if you changed it to:

```rust
enum Fader {
    A,
    C,
    B,
    D
}
```

Functionally, visually, and interactively, the enum is identical, and that change shouldn't be
expected to cause any problems. However, in the utils USB crate, code exists which turns the enum
into an u32 based on the order:
```rust
// From goxlr-utility/usb/commands.rs
impl Command {
    pub fn command_id(&self) {
        match self {
            Command::SetFader(fader) => (0x805 << 12) | *fader as u32,            
        }        
    }
}
```
This will subsequently flip faders B and C when executing commands to the GoXLR, despite the type 
naming being accurate, and all code interactions logically making sense.

So for here, *ANY* type which is being sent over to USB needs to have its own specific structures
and enums to ensure compatibility.