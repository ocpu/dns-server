mod class;
mod domain_name;
mod header;
mod label;
mod question;
mod resource;
mod types;
mod packet;
mod macros;

pub use self::class::{Class, QClass};
pub use self::domain_name::DomainName;
pub use self::header::{
    HeaderView, HeaderViewError, HeaderViewValidated, Opcode, PacketType, ResponseCode,
    UnknownOpcodeError, UnknownResponseCodeError,
};
pub use self::label::{Label, LabelError};
pub use self::question::{Question, QuestionError};
pub use self::resource::{Resource, ResourceError};
pub use self::types::{QType, Type};
pub use self::packet::{Packet, PacketError};

use std::fmt;

pub(self) struct DebugList<F, I>(F) where F: Fn() -> I, I: Iterator, I::Item: fmt::Debug;
impl<F, I> fmt::Debug for DebugList<F, I> where F: Fn() -> I, I: Iterator, I::Item: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0()).finish()
    }
}

pub trait FromPacketBytes<'data>: Sized {
    type Error;

    fn parse(bytes: &'data [u8], offset: usize) -> Result<Option<Self>, Self::Error>;
}