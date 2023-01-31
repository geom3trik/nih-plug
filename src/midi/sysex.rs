//! Traits for working with MIDI SysEx data.

use std::fmt::Debug;

/// A type that can be converted to and from byte buffers containing MIDI SysEx messages.
pub trait SysExMessage: Debug + Clone + PartialEq + Send + Sync {
    /// The maximum SysEx message size, in bytes.
    const MAX_BUFFER_SIZE: usize;

    // TODO: Conversion functions
}

/// A default implementation plugins that don't need SysEx support can use.
impl SysExMessage for () {
    const MAX_BUFFER_SIZE: usize = 0;
}