use std::{num::NonZeroU64, sync::Arc, time::Duration};

use wgpu::{Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BufferBinding, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, DeviceDescriptor, Features, Maintain, MapMode, PipelineLayoutDescriptor, PowerPreference, RequestAdapterOptionsBase, ShaderStages, include_spirv};

fn main() {
    pollster::block_on(run()).unwrap();
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let instance = wgpu::Instance::new(Backends::PRIMARY);
    let adapter = instance.request_adapter(&RequestAdapterOptionsBase {
        power_preference: PowerPreference::LowPower,
        force_fallback_adapter: false,
        compatible_surface: None,
    }).await.expect("couldn't find adapter");
    let (device, queue) = adapter.request_device(&DeviceDescriptor {
        label: None,
        features: Features::CLEAR_COMMANDS,
        limits: Default::default(),
    }, None).await?;

    dbg!();

    let shader_desc = include_spirv!("../target/spirv-builder/spirv-unknown-vulkan1.1/release/deps/shader.spv.dir/module");
    let shader = device.create_shader_module(&shader_desc);
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(16),
            },
            count: None,
        }],
    });
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main_cs",
    });

    dbg!();

    let buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: 16,
        usage: BufferUsages::STORAGE | BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    dbg!();

    let mut command_encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: None,
    });
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    dbg!();
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&ComputePassDescriptor { label: None });
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.dispatch(1, 1, 1);
    }

    dbg!();
    command_encoder.clear_buffer(&buffer, 0, None);
    let command_buffer = command_encoder.finish();

    dbg!();

    queue.submit([command_buffer]);

    dbg!();

    let buffer_slice = buffer.slice(..);
    let buffer_map_future = buffer_slice.map_async(MapMode::Read);
    let device2 = Arc::new(device);

    dbg!();
    std::thread::spawn(move || loop {
        println!("polling...");
        device2.poll(Maintain::Poll);
        std::thread::sleep(Duration::from_millis(2000));
    });
    buffer_map_future.await?;

    dbg!();
    let buffer_view = buffer_slice.get_mapped_range();
    println!("{:?}", &buffer_view[..]);

    Ok(())
}
