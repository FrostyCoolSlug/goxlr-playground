# GoXLR USB Messaging

This is a somewhat temporary crate where new USB communication handling is being prototyped 
and given a base implementation. Over time, the current `goxlr-usb` crate will be slowly
migrated here, and eventually this will replace it.

The main reason this is being handled separately is that the existing USB crate needs a 
reorganise, and is currently getting too much in the way of the rework, it was becoming too
easy to get trapped in the old paradigm rather than shifting to a new one. 