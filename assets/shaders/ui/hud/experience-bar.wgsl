#import bevy_ui::ui_vertex_output::UiVertexOutput

struct ExperienceBarMaterial {
    foreground_color: vec4<f32>,
    background_color: vec4<f32>,
    percent: f32,
    border_x: f32,
    border_y: f32,
};

@group(1) @binding(0) var<uniform> experience_bar: ExperienceBarMaterial;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
	if (mesh.uv.x <= experience_bar.border_x || mesh.uv.x >= (1 - experience_bar.border_x)) {
		return experience_bar.background_color;
	}
	if (mesh.uv.y <= experience_bar.border_y || mesh.uv.y >= (1 - experience_bar.border_y)) {
    	return experience_bar.background_color;
    }

    if (mesh.uv.x <= experience_bar.percent) {
        return experience_bar.foreground_color;
    } else {
        return experience_bar.background_color;
    }
}
