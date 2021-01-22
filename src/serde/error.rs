use std::io;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;


#[derive(Debug)]
pub enum DataError{
    Io(io::Error),
    Str(String),
    Other(Box<dyn Error>)
}


impl Display for DataError{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::Io(err)=>{
                Display::fmt(err,f)
            },
            DataError::Str(err)=>{
                write!(f, "DataError Str:{}",err )
            },
            DataError::Other(err)=>{
                Display::fmt(err,f)
            }
        }
    }
}

impl Error for DataError{}


// Parse our own error message that looks like "{} at line {} column {}" to work
// around erased-serde round-tripping the error through de::Error::custom.
#[inline]
fn make_error(mut msg: String) -> DataError {
    let (line, column) = parse_line_col(&mut msg).unwrap_or((0, 0));
    DataError::Str(format!("{} line:{} column:{}",msg,line,column))
}

#[inline]
fn parse_line_col(msg: &mut String) -> Option<(usize, usize)> {
    let start_of_suffix = match msg.rfind(" at line ") {
        Some(index) => index,
        None => return None,
    };

    // Find start and end of line number.
    let start_of_line = start_of_suffix + " at line ".len();
    let mut end_of_line = start_of_line;
    while starts_with_digit(&msg[end_of_line..]) {
        end_of_line += 1;
    }

    if !msg[end_of_line..].starts_with(" column ") {
        return None;
    }

    // Find start and end of column number.
    let start_of_column = end_of_line + " column ".len();
    let mut end_of_column = start_of_column;
    while starts_with_digit(&msg[end_of_column..]) {
        end_of_column += 1;
    }

    if end_of_column < msg.len() {
        return None;
    }

    // Parse numbers.
    let line = match usize::from_str(&msg[start_of_line..end_of_line]) {
        Ok(line) => line,
        Err(_) => return None,
    };
    let column = match usize::from_str(&msg[start_of_column..end_of_column]) {
        Ok(column) => column,
        Err(_) => return None,
    };

    msg.truncate(start_of_suffix);
    Some((line, column))
}

#[inline]
fn starts_with_digit(slice: &str) -> bool {
    match slice.as_bytes().get(0) {
        None => false,
        Some(&byte) => byte >= b'0' && byte <= b'9',
    }
}

impl From<io::Error> for DataError{
    #[cold]
    fn from(err: io::Error) -> Self {
        DataError::Io(err)
    }
}

impl serde::ser::Error for DataError{
    #[cold]
    fn custom<T>(msg: T) -> Self where T: Display {
        make_error(msg.to_string())
    }
}

impl serde::de::Error for DataError{
    #[cold]
    fn custom<T>(msg: T) -> Self where
        T: Display {
        make_error(msg.to_string())
    }
}