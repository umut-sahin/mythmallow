#import bevy_ui::ui_vertex_output::UiVertexOutput

struct HealthBarMaterial {
    foreground_color: vec4<f32>,
    background_color: vec4<f32>,
    percent: f32,
    border_x: f32,
    border_y: f32,
};

@group(1) @binding(0) var<uniform> health_bar: HealthBarMaterial;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
	if (mesh.uv.x <= health_bar.border_x || mesh.uv.x >= (1 - health_bar.border_x)) {
		return health_bar.background_color;
	}
	if (mesh.uv.y <= health_bar.border_y || mesh.uv.y >= (1 - health_bar.border_y)) {
    	return health_bar.background_color;
    }

    if (mesh.uv.x <= health_bar.percent) {
        return health_bar.foreground_color;
    } else {
        return health_bar.background_color;
    }
}
