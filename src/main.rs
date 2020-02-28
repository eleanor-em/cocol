use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::device::QueuesIter;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use simple_error::SimpleError;

use std::sync::Arc;
use std::error::Error;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::sync::GpuFuture;

#[inline]
fn fail(s: &str) -> SimpleError {
    SimpleError::new(s)
}

/// Perform dirty initialisation work for Vulkan.
fn init() -> Result<(Arc<Device>, QueuesIter), Box<dyn Error>> {
    // Create instance
    let instance = Instance::new(None, &InstanceExtensions::none(), None)?;

    // Choose a physical device
    let mut devices = PhysicalDevice::enumerate(&instance);
    for device in devices.clone() {
        println!("Detected: {} (type: {:?})", device.name(), device.ty());
    }
    let physical_device = devices.next().ok_or(fail("No physical device available."))?;
    println!("Using: {}", physical_device.name());

    // Load relevant queue families
    let mut queue_families = physical_device.queue_families()
        .filter(|&q| q.supports_compute());
    let count = queue_families.clone().count();
    let queue_family = queue_families.next().ok_or(fail("No compute queues available."))?;
    println!("Found {} compute queue{}.", count, if count == 1 { "" } else { "s" });

    let (device, queues) = Device::new(physical_device,
         &Features::none(),
         &DeviceExtensions::none(),
         [(queue_family, 0.5)].iter().cloned())?;
    
    Ok((device, queues))
}

fn main() {
    let (device, mut queues) = init().expect("Failed to initialise Vulkan");
    let queue = queues.next().unwrap();

    let data_iter = 0..65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), data_iter)
        .expect("Failed to create buffer");

    let shader = cs::Shader::load(device.clone())
        .expect("Failed to create shader module");

    let pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                                .expect("Failed to create pipeline"));

    let set = Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
        .add_buffer(data_buffer.clone()).unwrap()
        .build().unwrap());

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(),
        queue.family()).unwrap()
        .dispatch([1024, 1, 1], pipeline.clone(), set.clone(), ()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Done.");
}

mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        src:"\
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}
        "
    }
}