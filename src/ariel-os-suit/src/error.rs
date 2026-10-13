// #[derive(Debug, Copy, Clone, PartialEq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub enum Error {
//     AuthenticationFailure,
//     InvalidManifestStructure,
//     InvalidManifestSequence { position: usize },
//     ConditionMatchFail { position: usize },
//     MissingCommandSection { section: i16 },
//     ManifestProcessingError,
//     MissingParameter { position: usize },
//     InvalidDirective { identifier: u32 },
//     UnsupportedDigestAlgorithm { algorithm: i32 },
//     UnsupportedComponent { identifier: i64 },
//     UnsupportedParameter { parameter: i32 },
//     TransportError,
//     Other,
// }

// impl core::fmt::Display for Error {
//     fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             _ => todo!(),
//         }
//     }
// }

// impl core::error::Error for Error {}

// impl From<dress_up::error::Error> for Error {
//     fn from(value: dress_up::error::Error) -> Self {
//         match value {
//             dress_up::error::Error::AuthenticationFailure => Self::AuthenticationFailure,
//             dress_up::error::Error::CapacityError => Self::ManifestProcessingError,
//             dress_up::error::Error::ConditionMatchFail { position } => {
//                 Self::ConditionMatchFail { position }
//             }
//             dress_up::error::Error::TryEachFail { position } => {
//                 Self::ConditionMatchFail { position }
//             }
//             dress_up::error::Error::EndOfInput => Self::ManifestProcessingError,
//             dress_up::error::Error::InvalidAuthenticationStructure => Self::ManifestProcessingError,
//             dress_up::error::Error::InvalidCommandSequence { position } => {
//                 Self::InvalidManifestSequence { position }
//             }
//             dress_up::error::Error::InvalidCommonSection => Self::InvalidManifestStructure,
//             dress_up::error::Error::NoAuthObject => Self::InvalidManifestStructure,
//             dress_up::error::Error::NoCommonSection => Self::InvalidManifestStructure,
//             dress_up::error::Error::NoCommandSection { section } => {
//                 Self::MissingCommandSection { section }
//             }
//             dress_up::error::Error::NoComponentList => Self::InvalidManifestStructure,
//             dress_up::error::Error::NoManifestObject => Self::InvalidManifestStructure,
//             dress_up::error::Error::NoManifestVersion => Self::InvalidManifestStructure,
//             dress_up::error::Error::NoSequenceNumber => Self::InvalidManifestStructure,
//             dress_up::error::Error::ParameterNotSet { position } => {
//                 Self::MissingParameter { position }
//             }
//             dress_up::error::Error::SameSourceAndTarget { identifier } => {
//                 Self::InvalidDirective { identifier }
//             }
//             dress_up::error::Error::InvalidSourceComponent { identifier } => {
//                 Self::InvalidDirective { identifier }
//             }
//             dress_up::error::Error::UnexpectedCbor { .. } => Self::InvalidManifestStructure,
//             dress_up::error::Error::UnexpectedIndefiniteLength { .. } => {
//                 Self::InvalidManifestStructure
//             }
//             dress_up::error::Error::UnsupportedCommand { command } => Self::InvalidDirective {
//                 identifier: command as u32,
//             },
//             dress_up::error::Error::UnsupportedComponentIdentifier { identifier } => {
//                 Self::UnsupportedComponent { identifier }
//             }
//             dress_up::error::Error::UnsupportedDigestAlgo { algorithm } => {
//                 Self::UnsupportedDigestAlgorithm {
//                     algorithm: algorithm as i32,
//                 }
//             }
//             dress_up::error::Error::UnsupportedManifestVersion => Self::InvalidManifestStructure,
//             dress_up::error::Error::UnsupportedParameter { parameter } => {
//                 Self::UnsupportedParameter { parameter }
//             }
//             dress_up::error::Error::Utf8Error { .. } => Self::InvalidManifestStructure,
//             _ => Self::Other
//         }
//     }
// }
