// Copied from
// <https://github.com/bergzand/Ariel-os/blob/0a44aaa8178083588f82b3f4187051ab4ec75365/examples/rustweek-demo/src/coap_ext.rs>.

use coap_message::MinimalWritableMessage;
use coap_numbers::option;

fn block_opt_val(szx: u8, num: u32) -> u32 {
    num << 4 | szx as u32
}

pub trait OptionMessageWriter: MinimalWritableMessage {
    fn add_option_uri_path(&mut self, path: &str) -> Result<(), Self::AddOptionError>;
    fn add_option_uri_query(&mut self, path: &str) -> Result<(), Self::AddOptionError>;
    fn add_option_content_format(
        &mut self,
        content_format: u16,
    ) -> Result<(), Self::AddOptionError>;

    fn add_option_block2(&mut self, szx: u8, num: u32) -> Result<(), Self::AddOptionError>;

    #[allow(unused)]
    fn add_option_block1(&mut self, szx: u8, num: u32) -> Result<(), Self::AddOptionError>;
}

impl<T: MinimalWritableMessage> OptionMessageWriter for T
where
    <T as MinimalWritableMessage>::OptionNumber: From<u16>,
{
    fn add_option_uri_path(&mut self, path: &str) -> Result<(), Self::AddOptionError> {
        for component in path.split('/') {
            self.add_option_str(option::URI_PATH.into(), component)?;
        }
        Ok(())
    }

    fn add_option_block2(&mut self, szx: u8, num: u32) -> Result<(), Self::AddOptionError> {
        let val: u32 = block_opt_val(szx, num);
        self.add_option_uint(option::BLOCK2.into(), val)
    }

    fn add_option_block1(&mut self, szx: u8, num: u32) -> Result<(), Self::AddOptionError> {
        let val: u32 = block_opt_val(szx, num);
        self.add_option_uint(option::BLOCK1.into(), val)
    }

    fn add_option_uri_query(&mut self, query: &str) -> Result<(), Self::AddOptionError> {
        self.add_option_str(option::URI_QUERY.into(), query)
    }

    fn add_option_content_format(
        &mut self,
        content_format: u16,
    ) -> Result<(), Self::AddOptionError> {
        self.add_option_uint(option::CONTENT_FORMAT.into(), content_format)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block2Opt(u32);

pub trait Block2RequestDataExt {
    fn has_more(&self) -> bool;

    fn blocknum(&self) -> u32;
}

impl Block2Opt {
    pub fn to_option_value(self) -> u32 {
        self.0
    }

    pub fn szx(self) -> u8 {
        (self.to_option_value() & 0x7u32) as u8
    }

    pub fn size(self) -> u16 {
        1u16 << 4 + self.szx()
    }
}

impl From<u32> for Block2Opt {
    fn from(value: u32) -> Self {
        Block2Opt(value)
    }
}

impl Block2RequestDataExt for Block2Opt {
    fn has_more(&self) -> bool {
        (self.to_option_value() & 0x08) == 0x08
    }

    fn blocknum(&self) -> u32 {
        self.to_option_value() >> 4
    }
}
