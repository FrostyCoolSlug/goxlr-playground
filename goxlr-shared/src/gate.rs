#[cfg(feature = "clap")]
use clap::ValueEnum;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum GateTimes {
    Time10ms,
    Time20ms,
    Time30ms,
    Time40ms,
    Time50ms,
    Time60ms,
    Time70ms,
    Time80ms,
    Time90ms,
    Time100ms,
    Time110ms,
    Time120ms,
    Time130ms,
    Time140ms,
    Time150ms,
    Time160ms,
    Time170ms,
    Time180ms,
    Time190ms,
    Time200ms,
    Time250ms,
    Time300ms,
    Time350ms,
    Time400ms,
    Time450ms,
    Time500ms,
    Time550ms,
    Time600ms,
    Time650ms,
    Time700ms,
    Time750ms,
    Time800ms,
    Time850ms,
    Time900ms,
    Time950ms,
    Time1000ms,
    Time1100ms,
    Time1200ms,
    Time1300ms,
    Time1400ms,
    Time1500ms,
    Time1600ms,
    Time1700ms,
    Time1800ms,
    Time1900ms,
    Time2000ms,
}
