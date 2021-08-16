struct VertexInput {
	[[location(0)]] position: vec3<f32>;
	[[location(1)]] normal: vec3<f32>;
	[[location(2)]] uv: vec2<f32>;
	[[location(3)]] color: vec4<f32>;
	[[location(4)]] model_0: vec4<f32>;
	[[location(5)]] model_1: vec4<f32>;
	[[location(6)]] model_2: vec4<f32>;
	[[location(7)]] model_3: vec4<f32>;
};

struct VertexOutput {
	[[builtin(position)]] position: vec4<f32>;
	[[location(0)]] w_position: vec3<f32>;
	[[location(1)]] w_normal: vec3<f32>;
	[[location(2)]] color: vec4<f32>;
};

[[block]]
struct Camera {
	view_proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: Camera;

struct PointLight {
	position: vec3<f32>;
	intensity: f32;
	color: vec4<f32>;
};

[[block]]
struct Lights {
	ambient_color: vec4<f32>;
	ambient_intensity: f32;
	point_lights_len: u32;
	point_lights: array<PointLight, 64>;
};

[[group(0), binding(1)]]
var<uniform> lights: Lights;

[[stage(vertex)]]
fn main(in: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	let model = mat4x4<f32>(
		in.model_0,
		in.model_1,
		in.model_2,
		in.model_3,
	);

	out.position = model * vec4<f32>(in.position, 1.0);
	out.w_position = out.position.xyz;
	out.position = camera.view_proj * out.position;

	out.w_normal = (model * vec4<f32>(in.normal, 0.0)).xyz;
	out.color = in.color;

	return out;
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	var light: vec3<f32>;

	light = lights.ambient_color.rgb * lights.ambient_intensity;

	var i: u32 = 0u32;
	loop {
		if (i >= lights.point_lights_len) { break; }

		let point_light: PointLight = lights.point_lights[i];

		let pos_to_light = point_light.position - in.w_position;
		let dist = length(pos_to_light);
		let light_dir = normalize(pos_to_light);
		
		let diffuse = max(dot(light_dir, in.w_normal), 0.0);

		let intensity = 1.0 / pow(dist, 2.0) * point_light.intensity;

		light = light + point_light.color.rgb * diffuse * intensity;

		i = i + 1u32;
	}

	let color = vec4<f32>(light, 1.0) * in.color;

	return color;
}