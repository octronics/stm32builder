/// Available Random Access Memory (ram) (in Kb) on a device
#[derive(Debug, Clone, PartialEq)]
pub struct RamSize(pub u32);

/// Identify a part availble on a device line.
#[derive(Debug, Clone, PartialEq)]
pub struct Part(pub String);
