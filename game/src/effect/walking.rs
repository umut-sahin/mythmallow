use crate::prelude::*;


/// Gap between spawned walking particles.
pub const WALKING_EFFECT_GAP: f32 = 50.00;

/// Walking effect offset to be randomized.
pub const WALKING_EFFECT_SPREAD: f32 = 10.00;


/// Walking effect color at the start.
pub const WALKING_START_COLOR: Vec4 = Vec4::new(0.00, 0.00, 0.00, 1.00);

/// Walking effect color at the end.
pub const WALKING_END_COLOR: Vec4 = Vec4::new(0.20, 0.20, 0.20, 0.50);


/// Tag component for walking particles.
#[derive(Component, Debug, Default, Reflect)]
pub struct WalkingParticles;


/// Component for distance between particle spawns.
#[derive(Clone, Component, Copy, Debug, Deref, Reflect)]
pub struct LastWalkingParticlePosition(pub Vec2);

impl Default for LastWalkingParticlePosition {
    fn default() -> LastWalkingParticlePosition {
        LastWalkingParticlePosition(Vec2::ZERO)
    }
}


/// Particle effect for walking.
pub fn spawn_walking_effect(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let spawner = Spawner::once(CpuValue::Single(1.00), false);

    let writer = ExprWriter::new();
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.00, WALKING_START_COLOR);
    color_gradient.add_key(1.00, WALKING_END_COLOR);

    let size = (writer.rand(ScalarType::Float) * writer.lit(3.00) + writer.lit(12.00)).expr();
    let init_size = SetAttributeModifier::new(Attribute::SIZE, size);

    let age = writer.lit(0.00).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.40).uniform(writer.lit(0.60)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(2.00).uniform(writer.lit(6.00)).expr(),
        dimension: ShapeDimension::Surface,
    };

    let mut module = writer.finish();

    let round = RoundModifier::constant(&mut module, 1.00);

    let walking_effect = effects.add(
        EffectAsset::new(vec![32768], spawner, module)
            .init(init_pos)
            .init(init_age)
            .init(init_lifetime)
            .init(init_size)
            .render(ColorOverLifetimeModifier { gradient: color_gradient })
            .render(round),
    );


    commands.spawn((
        Name::new("Walking Particles"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(walking_effect),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
        WalkingParticles,
    ));
}

/// Despawns player particles.
pub fn despawn_walking_particles(
    mut commands: Commands,
    particle_query: Query<Entity, With<WalkingParticles>>,
) {
    if let Ok(entity) = particle_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
