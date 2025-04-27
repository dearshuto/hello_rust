fn main() {
    let source = r"
void main()
{
}
";

    let options = naga::front::glsl::Options::from(naga::ShaderStage::Mesh);
    let module = naga::front::glsl::Frontend::default()
        .parse(&options, source)
        .unwrap();

    let module_info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();

    // wgsl
    naga::back::wgsl::write_string(&module, &module_info, naga::back::wgsl::WriterFlags::all())
        .unwrap();

    // spv
    let options = naga::back::spv::Options::default();
    let _ = naga::back::spv::write_vec(&module, &module_info, &options, None).unwrap();
}
