macro_rules! include_spirv {
    ($p:literal) => {
        include_bytes!(concat!(
            "../../target/spirv-unknown-vulkan1.2/release/deps/shader.spvs/",
            $p
        ))
    };
}

fn main() {
    const FS_SHADER: &[u8] = include_spirv!("main_fs.spv");
    const VS_SHADER: &[u8] = include_spirv!("vertex-main_vs.spv");
    println!("{}", FS_SHADER.len());
    println!("{}", VS_SHADER.len());
}
