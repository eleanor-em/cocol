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
use vulkano::descriptor::PipelineLayoutAbstract;

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
         &DeviceExtensions{khr_storage_buffer_storage_class:true, ..DeviceExtensions::none()},
         [(queue_family, 0.5)].iter().cloned())?;
    
    Ok((device, queues))
}

fn main() {
    let (device, mut queues) = init().expect("Failed to initialise Vulkan");
    let queue = queues.next().unwrap();

    // 16341
    let capacity = 10000;
    let values = [795u32, 435, 499, 56, 268, 958, 1495, 425, 1340, 512, 126, 1210, 97, 1281, 922, 915, 557, 709, 1524, 81, 186, 1288, 1075, 1007, 714];
    let weights = [424u32, 876, 248, 1279, 829, 286, 1066, 1371, 384, 315, 762, 182, 289, 914, 419, 997, 1492, 736, 1069, 978, 513, 624, 1146, 482, 224];

    // below is correct
//    let capacity = 1000;
//    let values = [104u32, 84, 28, 111, 69, 113, 60, 52, 36, 8, 57, 70, 93, 0, 57, 37, 24, 110, 79, 1, 28, 9, 113, 68, 68, 89, 41, 54, 8, 111, 93, 27, 47, 104, 69];
//    let weights = [38u32, 0, 53, 52, 49, 90, 91, 5, 42, 78, 38, 8, 113, 4, 102, 24, 9, 33, 35, 0, 56, 70, 106, 103, 28, 87, 43, 110, 64, 20, 89, 82, 34, 9, 31];

    let num_items = values.len() as u32;
    let result_iter = (0..((capacity + 1) * (num_items + 1))).map(|_| 0);

    let value_buffer = CpuAccessibleBuffer::from_iter(device.clone(),
                                                      BufferUsage {
                                                          transfer_source: true,
                                                          transfer_destination: true,
                                                          uniform_texel_buffer: false,
                                                          storage_texel_buffer: false,
                                                          uniform_buffer: false,
                                                          storage_buffer: true,
                                                          index_buffer: false,
                                                          vertex_buffer: false,
                                                          indirect_buffer: false
                                                      },
                                                      false,
                                                      values.iter().cloned())
        .expect("Failed to create value buffer");
    let weight_buffer = CpuAccessibleBuffer::from_iter(device.clone(),
                                                       BufferUsage {
                                                           transfer_source: true,
                                                           transfer_destination: true,
                                                           uniform_texel_buffer: false,
                                                           storage_texel_buffer: false,
                                                           uniform_buffer: false,
                                                           storage_buffer: true,
                                                           index_buffer: false,
                                                           vertex_buffer: false,
                                                           indirect_buffer: false
                                                       },
                                                       false,
                                                       weights.iter().cloned())
        .expect("Failed to create weight buffer");
    let result_buffer = CpuAccessibleBuffer::from_iter(device.clone(),
                                                       BufferUsage {
                                                           transfer_source: true,
                                                           transfer_destination: true,
                                                           uniform_texel_buffer: false,
                                                           storage_texel_buffer: false,
                                                           uniform_buffer: false,
                                                           storage_buffer: true,
                                                           index_buffer: false,
                                                           vertex_buffer: false,
                                                           indirect_buffer: false
                                                       },
                                                       false,
                                                       result_iter)
        .expect("Failed to create result buffer");

    let shader = cs::Shader::load(device.clone())
        .expect("Failed to create shader module");

    let pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                                .expect("Failed to create pipeline"));

    let layout = pipeline.layout().descriptor_set_layout(0)
        .expect("Failed to create descriptor set layout");
    let set = Arc::new(PersistentDescriptorSet::start(layout.clone())
        .add_buffer(value_buffer.clone()).unwrap()
        .add_buffer(weight_buffer.clone()).unwrap()
        .add_buffer(result_buffer.clone()).unwrap()
        .build().unwrap());

    let size = capacity / 32 + 1;

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(),
        queue.family()).unwrap()
        .dispatch([size, 1, 1], pipeline.clone(), set.clone(), [capacity, num_items]).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let result = result_buffer.read().unwrap();

    println!("Max value: {}", result.last().unwrap());
    println!("Done.");
}

mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        path: "knapsack.shader"
    }
}