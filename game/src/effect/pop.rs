use crate::prelude::*;


/// Pop effect offset to be randomized.
pub const POP_EFFECT_SPREAD: f32 = 7.00;


/// Pop effect color at the start.
pub const POP_START_COLOR: Vec4 = Vec4::new(1.00, 1.00, 1.00, 1.00);

/// Pop effect color at the end.
pub const POP_END_COLOR: Vec4 = Vec4::new(0.20, 0.20, 0.20, 1.00);


/// Tag component for pop particles.
#[derive(Component, Debug, Default, Reflect)]
pub struct PopParticles;


/// Sets up pop particles.
pub fn spawn_pop_particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.00, POP_START_COLOR);
    color_gradient.add_key(1.00, POP_END_COLOR);

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.00, Vec2::splat(14.00));
    size_gradient.add_key(1.00, Vec2::splat(0.00));

    let writer = ExprWriter::new();

    let age = writer.lit(0.00).uniform(writer.lit(0.20)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.30).uniform(writer.lit(0.50)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(3.00).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(200.00).expr(),
    };

    let spawner = Spawner::once(CpuValue::Uniform((3.00, 4.00)), false);
    let mut module = writer.finish();

    let round = RoundModifier::constant(&mut module, 1.00);

    let pop_effect = effects.add(
        EffectAsset::new(vec![32768], spawner, module)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier { gradient: color_gradient })
            .render(SizeOverLifetimeModifier { gradient: size_gradient, screen_space_size: false })
            .render(round),
    );

    commands.spawn((
        Name::new("Pop Particles"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(pop_effect).with_z_layer_2d(Some(Depth::Effect.z())),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
        PopParticles,
    ));
}

/// Despawns pop particles.
pub fn despawn_pop_particles(
    mut commands: Commands,
    particle_query: Query<Entity, With<PopParticles>>,
) {
    if let Ok(entity) = particle_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
