use thiserror::Error;

#[derive(Error, Debug)]
pub enum EthereumHanshakeError {
    #[error("failed to parse node id format")]
    FailedToParseNodeIDFormat,

    #[error("failed to parse node id invalid scheme")]
    FailedToParseNodeScheme,

    #[error("no host found")]
    NoHostFound,

    #[error("failed to lookup host")]
    FailedToLookupHost,

    #[error("No socket address found for {0}:{1}")]
    NoSocketAddrFound(String, u16),
}

impl From<url::ParseError> for EthereumHanshakeError {
    fn from(_: url::ParseError) -> Self {
        EthereumHanshakeError::FailedToParseNodeIDFormat
    }
}

impl From<std::io::Error> for EthereumHanshakeError {
    fn from(_: std::io::Error) -> Self {
        EthereumHanshakeError::FailedToParseNodeIDFormat
    }
}
