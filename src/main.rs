use wgpu_bootstrap::{
    window::Window,
    frame::Frame,
    cgmath,
    application::Application,
    context::Context,
    camera::Camera,
    default:: {Vertex, Particle},
    wgpu,
    geometry::icosphere,
};
use rand::Rng;

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    let mut sum: [f32; 3] = [0.0, 0.0, 0.0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        sum[i] = aval + bval;
    }
    sum
}
fn multiply(a: [f32; 3], b: f32) -> [f32; 3] {
    let mut mult = a;
    for elem in mult.iter_mut() {
        *elem = *elem * b;
    }
    mult
}
fn check_collisions(particle: &mut Particle) {
    if (particle.position[0] > 29.0) & (particle.velocity[0] > 0.0) {
        particle.velocity[0] = -particle.velocity[0];
    }
    else if (particle.position[0] < 1.0) & (particle.velocity[0] < 0.0) {
        particle.velocity[0] = -particle.velocity[0];
    }
    if (particle.position[1] > 29.0) & (particle.velocity[1] > 0.0) {
        particle.velocity[1] = -particle.velocity[1];
    }
    else if (particle.position[1] < 1.0) & (particle.velocity[1] < 0.0) {
        particle.velocity[1] = -particle.velocity[1];
    }
    if (particle.position[2] > 29.0) & (particle.velocity[2] > 0.0) {
        particle.velocity[2] = -particle.velocity[2];
    }
    else if (particle.position[2] < 1.0) & (particle.velocity[2] < 0.0) {
        particle.velocity[2] = -particle.velocity[2];
    }
}
//----- BOX -----
const BOX_VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.0, 30.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [30.0, 0.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [30.0, 30.0, 0.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.0, 0.0, 30.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.0, 30.0, 30.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [30.0, 0.0, 30.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [30.0, 30.0, 30.0], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
];

const BOX_INDICES: &[u16] = &[
    0, 1,
    0, 2,
    0, 4,
    1, 3,
    1, 5,
    2, 3,
    2, 6,
    3, 7,
    4, 5,
    4, 6,
    5, 7,
    6, 7,
];

//----- APP -----
struct MyApp {
    camera_bind_group: wgpu::BindGroup,
    box_pipeline: wgpu::RenderPipeline,
    particle_pipeline: wgpu::RenderPipeline,
    box_vertex_buffer: wgpu::Buffer,
    box_index_buffer: wgpu::Buffer,
    particle_vertex_buffer: wgpu::Buffer,
    particle_index_buffer: wgpu::Buffer,
    particles: Vec<Particle>,
    instance_buffer: wgpu::Buffer,
    nb_indices: usize,
}

impl MyApp {
    fn new(context: &Context) -> Self {
        //----- GRAPHICS -----
        let (particle_vertices, particle_indices) = icosphere(2);

        let camera = Camera {
            eye: (27.0, 27.0, 67.0).into(),
            target: (9.0, 7.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let (_camera_buffer, camera_bind_group) = camera.create_camera_bind_group(context);
    
        //----- PIPELINES -----
        let box_pipeline = context.create_render_pipeline(
            "Box Render Pipeline",
            include_str!("blue.wgsl"),
            &[Vertex::desc()],
            &[
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::LineList,
        );

        let particle_pipeline = context.create_render_pipeline(
            "Particle Render Pipeline",
            include_str!("shader_instances.wgsl"),
            &[Vertex::desc(), Particle::desc()],
            &[
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList
        );

        //----- INSTANCES -----
        let particles = (0..50).map(|_index| {
            let position = [rand::thread_rng().gen_range(5..25) as f32, rand::thread_rng().gen_range(5..25) as f32, rand::thread_rng().gen_range(5..25) as f32];
            let velocity = [rand::thread_rng().gen_range(-5..5) as f32, rand::thread_rng().gen_range(-5..5) as f32, rand::thread_rng().gen_range(-5..5) as f32];
            Particle {
                position, velocity,
            }
        }).collect::<Vec<_>>();

        //----- BUFFERS -----
        let box_vertex_buffer = context.create_buffer(BOX_VERTICES, wgpu::BufferUsages::VERTEX);
        let box_index_buffer = context.create_buffer(BOX_INDICES, wgpu::BufferUsages::INDEX);

        let particle_vertex_buffer = context.create_buffer(particle_vertices.as_slice(), wgpu::BufferUsages::VERTEX);
        let particle_index_buffer = context.create_buffer(particle_indices.as_slice(), wgpu::BufferUsages::INDEX);

        let instance_buffer = context.create_buffer(particles.as_slice(), wgpu::BufferUsages::VERTEX);

        Self {
            camera_bind_group,
            box_pipeline,
            particle_pipeline,
            box_vertex_buffer,
            box_index_buffer,
            particle_vertex_buffer,
            particle_index_buffer,
            particles,
            instance_buffer,
            nb_indices: particle_indices.len(),
        }
    }
}

impl Application for MyApp {
    fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        let mut frame = Frame::new(context)?;
        {
            let mut render_pass = frame.begin_render_pass(wgpu::Color {r: 0.15, g: 0.15, b: 0.2, a: 1.0});

            render_pass.set_pipeline(&self.box_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.box_vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.box_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(BOX_INDICES.len() as u32), 0, 0..1);

            render_pass.set_pipeline(&self.particle_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.particle_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.particle_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.nb_indices as u32), 0, 0..self.particles.len() as _);
        }

        frame.present();

        Ok(())
    }

    fn update(&mut self, context: &Context, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            check_collisions(particle);
            particle.velocity = add(particle.velocity, multiply([0.0, -5.0, 0.0], delta_time));
            particle.position = add(particle.position, multiply(particle.velocity, delta_time));
        }

        context.update_buffer(&self.instance_buffer, self.particles.as_slice());
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();

    let my_app = MyApp::new(context);

    window.run(my_app);
}