#![expect(unsafe_code)]

use embassy_rp::block::ImageDef;
// Safety:
// This is where tooling expects this.
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Program metadata for `picotool info`
// Safety:
// `.bl_entry` is where picotool expects this.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Ariel OS"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_description!(c"test"),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
