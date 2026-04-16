#![expect(unsafe_code)]

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
