use anyhow::anyhow;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DataError {
    Anyhow(anyhow::Error),
    Str(String),
    Io(std::io::Error),
    RollBack,
    Reset,
    AnyNotSupported,
    IgnoredAnyNotSupported
}

impl Display for DataError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::Io(err) => Display::fmt(err, f),
            DataError::Str(err) => {
                write!(f, "DataError Str:{}", err)
            }
            DataError::Anyhow(err) => Display::fmt(err, f),
            DataError::RollBack=> write!(f, "RollBack"),
            DataError::Reset=> write!(f, "Reset"),
            DataError::AnyNotSupported=> write!(f, "Deserialize any not supported"),
            DataError::IgnoredAnyNotSupported => write!(f, "Deserialize ignored any not supported"),
        }
    }
}

impl std::error::Error for DataError {}

impl serde::ser::Error for DataError {
    #[inline]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DataError::Anyhow(anyhow!("{}", msg))
    }
}

impl serde::de::Error for DataError {
    #[inline]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DataError::Anyhow(anyhow!("{}", msg))
    }
}

impl From<anyhow::Error> for DataError {
    #[inline]
    fn from(err: anyhow::Error) -> Self {
        DataError::Anyhow(err)
    }
}

impl From<std::io::Error> for DataError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        DataError::Io(err)
    }
}

