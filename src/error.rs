//! The errors of the `coap` module.

use core::fmt;

#[derive(Debug)]
pub enum MessageError {
    InvalidHeader,
    InvalidPacketLength,
    InvalidTokenLength,
    InvalidOptionDelta,
    InvalidOptionLength,
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageError::InvalidHeader => {
                write!(f, "CoAP error: invalid header")
            }
            MessageError::InvalidPacketLength => {
                write!(f, "CoAP error: invalid packet length")
            }
            MessageError::InvalidTokenLength => {
                write!(f, "CoAP error: invalid token length")
            }
            MessageError::InvalidOptionDelta => {
                write!(f, "CoAP error: invalid option delta")
            }
            MessageError::InvalidOptionLength => {
                write!(f, "CoAP error: invalid option length")
            }
        }
    }
}

#[derive(Debug)]
pub struct InvalidOption;

impl fmt::Display for InvalidOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CoAP error: invalid option number")
    }
}

#[derive(Debug)]
pub struct InvalidContentFormat;

impl fmt::Display for InvalidContentFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CoAP error: invalid content-format number")
    }
}

#[derive(Debug)]
pub struct InvalidObserve;

impl fmt::Display for InvalidObserve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CoAP error: invalid observe option number")
    }
}