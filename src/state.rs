use cgmath::num_traits::ToPrimitive;
use winit::event::WindowEvent;
use winit::window::Window;

#[cfg(debug_assertions)]
use perf_meter::PerfMeter;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub backend: wgpu::Backend,
    cursor_position: winit::dpi::PhysicalPosition<f64>,
    update_counter: u32,
    #[cfg(debug_assertions)]
    perf_meter: PerfMeter,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let backend = adapter.get_info().backend;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let cursor_position = winit::dpi::PhysicalPosition { x: 0_f64 ,y: 0_f64 };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let update_counter = 0;

        #[cfg(debug_assertions)]
        let perf_meter = PerfMeter::new(1);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            backend,
            cursor_position,
            update_counter,
            #[cfg(debug_assertions)]
            perf_meter,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            println!("new size w: {}, h: {}", new_size.width, new_size.height)
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        match _event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = *position;
            },
            _ => {},
        }
        false
    }

    pub fn update(&mut self) {
        self.update_counter += 1;
        #[cfg(debug_assertions)]
        self.perf_meter.tick();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let texture = self.surface.get_current_texture()?;
        let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: self.cursor_position.x / self.size.width.to_f64().unwrap_or(self.cursor_position.x),
                                    g: self.cursor_position.y / self.size.height.to_f64().unwrap_or(self.cursor_position.y),
                                    b: 0.0,
                                    a: 1.0,
                                },
                            ),
                            store: true,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();

        Ok(())
    }
}