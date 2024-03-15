use std::{fs::File, io::Write};

fn main() {
    let mut frontend = naga::front::glsl::Frontend::default();
    let option = naga::front::glsl::Options::from(naga::ShaderStage::Compute);
    let module = frontend
        .parse(&option, include_str!("res/compute.glsl"))
        .unwrap();

    let module_info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();
    let options = naga::back::spv::Options::default();
    let bytes = naga::back::spv::write_vec(&module, &module_info, &options, None).unwrap();

    let mut file = File::create("src/compute.spv").unwrap();
    let bytes = bytemuck::cast_slice(&bytes);
    file.write_all(&bytes).unwrap();
}
