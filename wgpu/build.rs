use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() {
    SpirvBuilder::new("shader", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build().unwrap();
}
