use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() {
    SpirvBuilder::new(".", "spirv-unknown-vulkan1.2")
        .print_metadata(MetadataPrintout::None)
        .multimodule(true)
        .build()
        .unwrap();
}
