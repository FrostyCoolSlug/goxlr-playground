use std::fmt::{Debug, Display, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FirmwareVersions {
    pub firmware: VersionNumber,
    pub dice: VersionNumber,
    pub fpga_count: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VersionNumber(pub u32, pub u32, pub Option<u32>, pub Option<u32>);

impl Display for VersionNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(patch) = self.2 {
            if let Some(build) = self.3 {
                return write!(f, "{}.{}.{}.{}", self.0, self.1, patch, build);
            }
            return write!(f, "{}.{}.{}", self.0, self.1, patch);
        }

        write!(f, "{}.{}", self.0, self.1)
    }
}

impl Debug for VersionNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
