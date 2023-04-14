use std::io::{Read, Write};

pub(crate) trait ExodusSerializable {
    /// The current version of the file serialization format, as byte.
    /// Other (older) versions are to be defined outside of the trait.
    const CURRENT_VERSION: u8;
    /// The returned error type if serializing or deserializing was unsuccessful.
    type ParseError;
    /// Serialize this instance using the newest available version of the binary
    /// (bincode) data format.
    /// Calling parse() on a serialized instance should return an instance exactly the same as
    /// the one that was serialized.
    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), Self::ParseError>;
    /// Parse the given stream and fill the instance with parsed data.
    /// This function should be able to handle older file format versions,
    /// according to the specification of the respective file format.
    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError>;
    /// Parse a file with the current version.
    /// The read position must be already behind the version byte, i.e. this function should be
    /// called internally after parsing the version identifier.
    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError>;
}
