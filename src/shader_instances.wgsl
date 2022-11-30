// Vertex shader

struct CameraUniform {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> matrices: CameraUniform;

struct ParticleInput {
    @location(5) position: vec3<f32>,
    @location(6) velocity: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    particle: ParticleInput,
) -> VertexOutput {
    let model_matrix = mat2x3<f32>(
        particle.position,
        particle.velocity,
    );
    var out: VertexOutput;
    out.clip_position = matrices.proj * matrices.view * vec4<f32>(model.position+particle.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(0.7, 0.2, 0.2, 1.0);
}