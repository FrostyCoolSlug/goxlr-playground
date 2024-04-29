pub mod channel;
pub mod fader;
pub mod input;
pub mod output;
pub mod sub_mix;
pub mod volume;

/// There are cases where Type X cannot be converted to Type Y without a panic! triggering,
/// for example several 'FaderChannel's don't support SubMixes, so cannot be converted into a
/// 'SubMixChannel'. Rather than having loads of checks all over the place, this trait allows
/// for an authoritative Top Level check which can be called in advance.
pub trait CanFrom<T> {
    fn can_from(value: T) -> bool;
}
