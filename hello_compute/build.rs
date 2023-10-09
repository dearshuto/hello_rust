use std::io::Write;

fn main()
{
	let source = include_str!("resources/write_buffer.glsl");
	let mut compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();

    let binary_result = compiler
        .compile_into_spirv(source, shaderc::ShaderKind::Compute, "shader.glsl", "main", Some(&options))
        .unwrap();

    let mut file = std::fs::File::create("src/write_buffer.spv").unwrap();
    let _result = file.write_all(binary_result.as_binary_u8());
}
