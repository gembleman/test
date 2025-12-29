use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        CopyBufferToImageInfo, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    format::Format,
    image::{
        sampler::{Sampler, SamplerCreateInfo, Filter, SamplerAddressMode},
        view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, Subpass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
        CompositeAlpha,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent, KeyEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
    keyboard::{KeyCode, PhysicalKey},
};
use fontdue::{Font, FontSettings};
use glam::{Mat4, Vec3};

// 정점 구조체
#[derive(BufferContents, Vertex, Clone, Copy)]
#[repr(C)]
struct TextVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
    #[format(R32G32_SFLOAT)]
    tex_coords: [f32; 2],
}

// Push Constants (투명도와 효과 설정)
#[derive(BufferContents, Clone, Copy)]
#[repr(C)]
struct PushConstants {
    opacity: f32,
    effect_type: i32, // 0: normal, 1: outline, 2: shadow, 3: glow
    outline_width: f32,
    shadow_offset: [f32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TextEffect {
    Normal,
    Outline,
    Shadow,
    Glow,
}

impl TextEffect {
    fn to_i32(&self) -> i32 {
        match self {
            TextEffect::Normal => 0,
            TextEffect::Outline => 1,
            TextEffect::Shadow => 2,
            TextEffect::Glow => 3,
        }
    }

    fn next(&self) -> Self {
        match self {
            TextEffect::Normal => TextEffect::Outline,
            TextEffect::Outline => TextEffect::Shadow,
            TextEffect::Shadow => TextEffect::Glow,
            TextEffect::Glow => TextEffect::Normal,
        }
    }

    fn name(&self) -> &str {
        match self {
            TextEffect::Normal => "일반",
            TextEffect::Outline => "외곽선",
            TextEffect::Shadow => "그림자",
            TextEffect::Glow => "발광",
        }
    }
}

fn main() {
    // Vulkan 초기화
    let library = VulkanLibrary::new().expect("Vulkan 라이브러리 로드 실패");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
    .expect("Instance 생성 실패");

    // 투명한 윈도우 생성
    let event_loop = EventLoop::new();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("투명 텍스트 렌더러 (Vulkan)")
            .with_transparent(true) // 투명 윈도우 설정
            .with_decorations(true)
            .build(&event_loop)
            .unwrap(),
    );

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    // Device 설정
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .expect("Physical device 열거 실패")
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("사용 가능한 device 없음");

    println!(
        "사용 중인 GPU: {} ({:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type
    );

    let (device, mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions,
            ..Default::default()
        },
    )
    .expect("Device 생성 실패");

    let queue = queues.next().unwrap();

    // Swapchain 생성 (투명도 지원)
    let (mut swapchain, images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .expect("Surface capabilities 가져오기 실패");

        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        // 투명도를 위한 CompositeAlpha 설정
        let composite_alpha = surface_capabilities
            .supported_composite_alpha
            .into_iter()
            .find(|&alpha| alpha == CompositeAlpha::PreMultiplied || alpha == CompositeAlpha::PostMultiplied)
            .or_else(|| surface_capabilities.supported_composite_alpha.into_iter().next())
            .unwrap();

        println!("Composite Alpha: {:?}", composite_alpha);

        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap()
    };

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    // 폰트 로드 및 텍스트 렌더링
    let font_data = include_bytes!("../NotoSansKR-Regular.ttf");
    let font = Font::from_bytes(font_data as &[u8], FontSettings::default())
        .expect("폰트 로드 실패");

    let text = "GPU 가속 투명 텍스트\n투명도: 100%\n효과: 일반";
    let font_size = 48.0;

    // 텍스트를 이미지로 렌더링
    let (texture_image, texture_width, texture_height) = create_text_texture(
        &font,
        text,
        font_size,
        device.clone(),
        memory_allocator.clone(),
        queue.clone(),
    );

    let texture_image_view = ImageView::new_default(texture_image.clone()).unwrap();

    // Sampler 생성
    let sampler = Sampler::new(
        device.clone(),
        SamplerCreateInfo {
            mag_filter: Filter::Linear,
            min_filter: Filter::Linear,
            address_mode: [SamplerAddressMode::ClampToEdge; 3],
            ..Default::default()
        },
    )
    .unwrap();

    // Vertex Buffer 생성 (화면 중앙에 텍스트 배치)
    let aspect_ratio = window.inner_size().width as f32 / window.inner_size().height as f32;
    let text_scale = 0.5;
    let vertices = [
        TextVertex {
            position: [-text_scale * aspect_ratio, -text_scale],
            tex_coords: [0.0, 0.0],
        },
        TextVertex {
            position: [text_scale * aspect_ratio, -text_scale],
            tex_coords: [1.0, 0.0],
        },
        TextVertex {
            position: [-text_scale * aspect_ratio, text_scale],
            tex_coords: [0.0, 1.0],
        },
        TextVertex {
            position: [text_scale * aspect_ratio, text_scale],
            tex_coords: [1.0, 1.0],
        },
    ];

    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vertices,
    )
    .unwrap();

    // 셰이더 정의
    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: r"
                #version 460

                layout(location = 0) in vec2 position;
                layout(location = 1) in vec2 tex_coords;

                layout(location = 0) out vec2 fragTexCoords;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                    fragTexCoords = tex_coords;
                }
            ",
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: r"
                #version 460

                layout(location = 0) in vec2 fragTexCoords;
                layout(location = 0) out vec4 outColor;

                layout(set = 0, binding = 0) uniform sampler2D texSampler;

                layout(push_constant) uniform PushConstants {
                    float opacity;
                    int effect_type;
                    float outline_width;
                    vec2 shadow_offset;
                } pc;

                void main() {
                    vec4 texColor = texture(texSampler, fragTexCoords);

                    if (pc.effect_type == 0) {
                        // 일반
                        outColor = vec4(texColor.rgb, texColor.a * pc.opacity);
                    } else if (pc.effect_type == 1) {
                        // 외곽선
                        float alpha = texColor.a;
                        vec2 texelSize = 1.0 / textureSize(texSampler, 0);
                        float outline = 0.0;
                        for (int x = -2; x <= 2; x++) {
                            for (int y = -2; y <= 2; y++) {
                                outline = max(outline, texture(texSampler, fragTexCoords + vec2(x, y) * texelSize * pc.outline_width).a);
                            }
                        }
                        vec3 color = mix(vec3(1.0, 1.0, 0.0), texColor.rgb, alpha);
                        outColor = vec4(color, max(alpha, outline * 0.8) * pc.opacity);
                    } else if (pc.effect_type == 2) {
                        // 그림자
                        vec4 shadow = texture(texSampler, fragTexCoords + pc.shadow_offset);
                        vec3 color = mix(shadow.rgb * 0.3, texColor.rgb, texColor.a);
                        float alpha = max(texColor.a, shadow.a * 0.6);
                        outColor = vec4(color, alpha * pc.opacity);
                    } else if (pc.effect_type == 3) {
                        // 발광
                        float glow = 0.0;
                        vec2 texelSize = 1.0 / textureSize(texSampler, 0);
                        for (int x = -3; x <= 3; x++) {
                            for (int y = -3; y <= 3; y++) {
                                float dist = length(vec2(x, y));
                                glow += texture(texSampler, fragTexCoords + vec2(x, y) * texelSize * 2.0).a / (1.0 + dist);
                            }
                        }
                        vec3 glowColor = vec3(0.2, 0.8, 1.0);
                        vec3 color = mix(glowColor * glow * 0.5, texColor.rgb, texColor.a);
                        float alpha = max(texColor.a, glow * 0.3);
                        outColor = vec4(color, alpha * pc.opacity);
                    }
                }
            ",
        }
    }

    let vs = vs::load(device.clone()).unwrap().entry_point("main").unwrap();
    let fs = fs::load(device.clone()).unwrap().entry_point("main").unwrap();

    // Render Pass
    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
    .unwrap();

    // Graphics Pipeline
    let pipeline = {
        let vertex_input_state = TextVertex::per_vertex()
            .definition(&vs.info().input_interface)
            .unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .unwrap();

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        // 블렌딩 활성화 (투명도 지원)
        let mut color_blend_state = ColorBlendState::with_attachment_states(
            subpass.num_color_attachments(),
            ColorBlendAttachmentState::default(),
        );
        color_blend_state.attachments[0].blend = Some(vulkano::pipeline::graphics::color_blend::AttachmentBlend::alpha());

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(color_blend_state),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap()
    };

    let mut viewport = Viewport {
        offset: [0.0, 0.0],
        extent: window.inner_size().into(),
        depth_range: 0.0..=1.0,
    };

    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone(), Default::default());
    let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        pipeline.layout().set_layouts().get(0).unwrap().clone(),
        [WriteDescriptorSet::image_view_sampler(
            0,
            texture_image_view.clone(),
            sampler.clone(),
        )],
        [],
    )
    .unwrap();

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    // 상태 변수
    let mut opacity = 1.0f32;
    let mut current_effect = TextEffect::Normal;

    println!("\n=== 컨트롤 ===");
    println!("1-9: 투명도 조절 (10% - 90%)");
    println!("0: 투명도 100%");
    println!("E: 텍스트 효과 전환");
    println!("ESC: 종료\n");

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(key_code),
                    ..
                },
                ..
            },
            ..
        } => {
            match key_code {
                KeyCode::Escape => *control_flow = ControlFlow::Exit,
                KeyCode::Digit1 => {
                    opacity = 0.1;
                    println!("투명도: 10%");
                }
                KeyCode::Digit2 => {
                    opacity = 0.2;
                    println!("투명도: 20%");
                }
                KeyCode::Digit3 => {
                    opacity = 0.3;
                    println!("투명도: 30%");
                }
                KeyCode::Digit4 => {
                    opacity = 0.4;
                    println!("투명도: 40%");
                }
                KeyCode::Digit5 => {
                    opacity = 0.5;
                    println!("투명도: 50%");
                }
                KeyCode::Digit6 => {
                    opacity = 0.6;
                    println!("투명도: 60%");
                }
                KeyCode::Digit7 => {
                    opacity = 0.7;
                    println!("투명도: 70%");
                }
                KeyCode::Digit8 => {
                    opacity = 0.8;
                    println!("투명도: 80%");
                }
                KeyCode::Digit9 => {
                    opacity = 0.9;
                    println!("투명도: 90%");
                }
                KeyCode::Digit0 => {
                    opacity = 1.0;
                    println!("투명도: 100%");
                }
                KeyCode::KeyE => {
                    current_effect = current_effect.next();
                    println!("효과: {}", current_effect.name());
                }
                _ => {}
            }
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            recreate_swapchain = true;
        }
        Event::RedrawEventsCleared => {
            let image_extent: [u32; 2] = window.inner_size().into();
            if image_extent.contains(&0) {
                return;
            }

            previous_frame_end.as_mut().unwrap().cleanup_finished();

            if recreate_swapchain {
                let (new_swapchain, new_images) = swapchain
                    .recreate(SwapchainCreateInfo {
                        image_extent,
                        ..swapchain.create_info()
                    })
                    .expect("Swapchain 재생성 실패");

                swapchain = new_swapchain;
                framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut viewport);
                recreate_swapchain = false;
            }

            let (image_index, suboptimal, acquire_future) =
                match acquire_next_image(swapchain.clone(), None).map_err(Validated::unwrap) {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("이미지 획득 실패: {e}"),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary(
                &command_buffer_allocator,
                queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            let push_constants = PushConstants {
                opacity,
                effect_type: current_effect.to_i32(),
                outline_width: 2.0,
                shadow_offset: [0.005, 0.005],
            };

            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.0, 0.0, 0.0, 0.0].into())], // 투명 배경
                        ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
                    },
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                )
                .unwrap()
                .set_viewport(0, [viewport.clone()].into_iter().collect())
                .unwrap()
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    descriptor_set.clone(),
                )
                .unwrap()
                .push_constants(pipeline.layout().clone(), 0, push_constants)
                .unwrap()
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .unwrap()
                .draw(vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap()
                .end_render_pass(Default::default())
                .unwrap();

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                )
                .then_signal_fence_and_flush();

            match future.map_err(Validated::unwrap) {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(VulkanError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    println!("렌더링 실패: {e}");
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }
        }
        _ => (),
    });
}

fn create_text_texture(
    font: &Font,
    text: &str,
    font_size: f32,
    device: Arc<Device>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    queue: Arc<vulkano::device::Queue>,
) -> (Arc<Image>, u32, u32) {
    use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        max_width: Some(800.0),
        max_height: Some(600.0),
        ..LayoutSettings::default()
    });
    layout.append(&[font], &TextStyle::new(text, font_size, 0));

    let width = 512;
    let height = 256;
    let mut buffer = vec![0u8; width * height];

    for glyph in layout.glyphs() {
        let (metrics, bitmap) = font.rasterize_config(glyph.key);
        let x_pos = glyph.x as i32;
        let y_pos = glyph.y as i32;

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let px = x_pos + x as i32;
                let py = y_pos + y as i32;

                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    let idx = (py * width as i32 + px) as usize;
                    let glyph_idx = y * metrics.width + x;
                    buffer[idx] = bitmap[glyph_idx];
                }
            }
        }
    }

    // RGBA 변환
    let rgba_buffer: Vec<u8> = buffer
        .iter()
        .flat_map(|&a| [255u8, 255u8, 255u8, a])
        .collect();

    let upload_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        rgba_buffer,
    )
    .unwrap();

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [width as u32, height as u32, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
    )
    .unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
            upload_buffer,
            image.clone(),
        ))
        .unwrap();

    let command_buffer = builder.build().unwrap();
    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    (image, width as u32, height as u32)
}

fn window_size_dependent_setup(
    images: &[Arc<Image>],
    render_pass: Arc<vulkano::render_pass::RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
