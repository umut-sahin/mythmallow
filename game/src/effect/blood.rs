use crate::prelude::*;


/// Blood effect offset to be randomized.
pub const BLOOD_EFFECT_SPREAD: f32 = 7.00;


/// Blood effect color at the start.
pub const BLOOD_START_COLOR: Vec4 = Vec4::new(4.00, 0.00, 0.00, 1.00);

/// Blood effect color at the end.
pub const BLOOD_END_COLOR: Vec4 = Vec4::new(4.00, 0.20, 0.20, 1.00);

/// Tag component for blood particles.
#[derive(Component, Debug, Default, Reflect)]
pub struct BloodParticles;


/// Particle effect for blood.
pub fn spawn_blood_particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.00, BLOOD_START_COLOR);
    color_gradient.add_key(0.40, BLOOD_START_COLOR);
    color_gradient.add_key(1.00, BLOOD_END_COLOR);

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.00, Vec2::splat(12.00));
    size_gradient.add_key(1.00, Vec2::splat(0.00));

    let writer = ExprWriter::new();

    let age = writer.lit(0.00).uniform(writer.lit(0.20)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.60).uniform(writer.lit(0.80)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * -150.00).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(2.00).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(4.00).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(60.00) + writer.lit(25.00)).expr(),
    };

    let spawner = Spawner::once(CpuValue::Uniform((7.00, 11.00)), false);
    let mut module = writer.finish();

    let round = RoundModifier::constant(&mut module, 1.00);

    let blood_effect = effects.add(
        EffectAsset::new(vec![32768], spawner, module)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(update_accel)
            .render(ColorOverLifetimeModifier { gradient: color_gradient })
            .render(SizeOverLifetimeModifier { gradient: size_gradient, screen_space_size: false })
            .render(round),
    );


    commands.spawn((
        Name::new("Blood Particles"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(blood_effect).with_z_layer_2d(Some(Depth::Effect.z())),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
        BloodParticles,
    ));
}

/// Despawns blood particles.
pub fn despawn_blood_particles(
    mut commands: Commands,
    particle_query: Query<Entity, With<BloodParticles>>,
) {
    if let Ok(entity) = particle_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
