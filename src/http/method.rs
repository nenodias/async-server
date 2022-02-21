use std::convert::TryFrom;

pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Method for TryFrom {
    type MethodError;

    /// Performs the conversion.
    #[stable(feature = "try_from", since = "1.34.0")]
    fn try_from(value: T) -> Result<Self, Self::Error>;
}