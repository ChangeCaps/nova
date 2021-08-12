struct VertexInput {
	[[location(0)]] position: vec3<f32>;
	[[location(1)]] uv: vec2<f32>;
	[[location(2)]] color: vec4<f32>;
};

struct VertexOutput {
	[[builtin(position)]] position: vec4<f32>;
	[[location(0)]] color: vec4<f32>;
};

[[block]]
struct Transform {
	matrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> transform: Transform;

[[block]]
struct Camera {
	view_proj: mat4x4<f32>;
};

[[group(0), binding(1)]]
var<uniform> camera: Camera;

[[stage(vertex)]]
fn main(in: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.position = camera.view_proj * transform.matrix * vec4<f32>(in.position, 1.0);
	out.color = in.color;

	return out;
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	var out: vec4<f32>;

	out = in.color;

	return out;
}