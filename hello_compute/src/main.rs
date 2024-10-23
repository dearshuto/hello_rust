// GPU で等差数列を計算するプログラム
#[tokio::main]
async fn main() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .await
        .unwrap();

    let shader_source = wgpu::util::make_spirv(include_bytes!("write_buffer.spv"));
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: shader_source,
    });

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (std::mem::size_of::<i32>() * 64) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_groyp_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_groyp_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_groyp_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(1, 1, 1);
    }
    queue.submit(Some(command_encoder.finish()));

    let buffer_slice = buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});

    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Write, move |result| {
            // if result.is_ok() {
            //     let mut view = capturable.slice(..).get_mapped_range_mut();
            //     let floats: &mut [f32] = bytemuck::cast_slice_mut(&mut view);
            //     floats.fill(42.0);
            //     drop(view);
            //     capturable.unmap();
            // }
        });

    device.poll(wgpu::Maintain::Wait);
    // let result: std::vec::Vec<i32> = bytemuck::cast_slice(&data).to_vec();
    // for index in 0..result.len() {
    //     println!("index {}: {}", index, result[index]);
    // }
}
