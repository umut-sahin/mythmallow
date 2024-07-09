use crate::prelude::*;


/// Material for the health bar.
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct HealthBarMaterial {
    #[uniform(0)]
    pub foreground_color: Vec4,

    #[uniform(0)]
    pub background_color: Vec4,

    #[uniform(0)]
    pub percent: f32,

    #[uniform(0)]
    pub border_x: f32,

    #[uniform(0)]
    pub border_y: f32,
}

impl Default for HealthBarMaterial {
    fn default() -> HealthBarMaterial {
        HealthBarMaterial {
            foreground_color: Vec4::new(1.00, 0.00, 0.00, 1.00),
            background_color: Vec4::new(0.00, 0.00, 0.00, 1.00),
            percent: 1.00,
            border_x: 0.025,
            border_y: 0.125,
        }
    }
}

impl UiMaterial for HealthBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ui/hud/health-bar.wgsl".into()
    }
}


/// Material for the experience bar.
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct ExperienceBarMaterial {
    #[uniform(0)]
    pub foreground_color: Vec4,

    #[uniform(0)]
    pub background_color: Vec4,

    #[uniform(0)]
    pub percent: f32,

    #[uniform(0)]
    pub border_x: f32,

    #[uniform(0)]
    pub border_y: f32,
}

impl Default for ExperienceBarMaterial {
    fn default() -> ExperienceBarMaterial {
        ExperienceBarMaterial {
            foreground_color: Vec4::new(0.00, 1.00, 0.00, 1.00),
            background_color: Vec4::new(0.00, 0.00, 0.00, 1.00),
            percent: 0.00,
            border_x: 0.025,
            border_y: 0.125,
        }
    }
}

impl UiMaterial for ExperienceBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ui/hud/experience-bar.wgsl".into()
    }
}
