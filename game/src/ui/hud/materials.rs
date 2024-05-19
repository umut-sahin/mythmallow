use crate::prelude::*;


/// Material for the health bar.
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct HealthBarMaterial {
    #[uniform(0)]
    pub foreground_color: Color,

    #[uniform(0)]
    pub background_color: Color,

    #[uniform(0)]
    pub percent: f32,
}

impl Default for HealthBarMaterial {
    fn default() -> HealthBarMaterial {
        HealthBarMaterial {
            foreground_color: Color::RED,
            background_color: Color::BLACK,
            percent: 1.00,
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
    pub foreground_color: Color,

    #[uniform(0)]
    pub background_color: Color,

    #[uniform(0)]
    pub percent: f32,
}

impl Default for ExperienceBarMaterial {
    fn default() -> ExperienceBarMaterial {
        ExperienceBarMaterial {
            foreground_color: Color::GREEN,
            background_color: Color::BLACK,
            percent: 0.00,
        }
    }
}

impl UiMaterial for ExperienceBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ui/hud/experience-bar.wgsl".into()
    }
}
