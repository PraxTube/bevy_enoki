use self::prelude::{
    Particle2dEffect, Particle2dMaterial, ParticleEffectInstance, ParticleSpawnerState,
    ParticleStore,
};
use crate::particles_2d::color::ColorParticle2dMaterial;
use bevy::{asset::load_internal_asset, prelude::*};

mod color;
mod loader;
mod material;
mod update;

#[allow(unused)]
pub mod prelude {
    pub use super::color::ColorParticle2dMaterial;
    pub use super::loader::{EmissionShape, Particle2dEffect, ParticleEffectLoader};
    pub use super::material::{Particle2dMaterial, Particle2dMaterialPlugin};
    pub use super::update::{OneShot, ParticleEffectInstance, ParticleSpawnerState, ParticleStore};
    pub use super::{ParticleSpawnerBundle, DEFAULT_MATERIAL};
}

pub(crate) const PARTICLE_VERTEX_OUT: Handle<Shader> =
    Handle::weak_from_u128(97641680653231235698756524351080664231);
pub(crate) const PARTICLE_VERTEX: Handle<Shader> =
    Handle::weak_from_u128(47641680653221245698756524350619564219);
pub(crate) const PARTICLE_DEFAULT_FRAG: Handle<Shader> =
    Handle::weak_from_u128(27641685611347896254665674658656433339);
pub(crate) const PARTICLE_SPRITE_FRAG: Handle<Shader> =
    Handle::weak_from_u128(52323345476969163624641232313030656999);
pub const DEFAULT_MATERIAL: Handle<ColorParticle2dMaterial> =
    Handle::weak_from_u128(96423345474653413361641232813030656919);

pub struct Particles2dPlugin;
impl Plugin for Particles2dPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PARTICLE_VERTEX_OUT,
            "particle_vertex_out.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PARTICLE_VERTEX,
            "particle_vertex.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PARTICLE_DEFAULT_FRAG,
            "particle_default_frag.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PARTICLE_SPRITE_FRAG,
            "particle_sprite_frag.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(material::Particle2dMaterialPlugin::<ColorParticle2dMaterial>::default());

        app.register_type::<update::ParticleStore>();
        app.register_type::<update::ParticleSpawnerState>();
        app.register_type::<update::ParticleSpawnerState>();
        app.register_type::<update::Particle>();

        app.world
            .resource_mut::<Assets<ColorParticle2dMaterial>>()
            .insert(DEFAULT_MATERIAL, ColorParticle2dMaterial::default());

        app.init_asset::<Particle2dEffect>();
        app.init_asset_loader::<loader::ParticleEffectLoader>();
        app.init_asset_loader::<color::ColorParticle2dAssetLoader>();

        app.add_systems(
            First,
            loader::on_asset_loaded.run_if(on_event::<AssetEvent<Particle2dEffect>>()),
        );

        app.add_systems(
            Update,
            (
                loader::reload_effect,
                update::clone_effect,
                update::remove_finished_spawner,
            ),
        );

        app.add_systems(
            PostUpdate,
            update::update_spawner.after(bevy::render::view::VisibilitySystems::CheckVisibility),
        );
    }
}

/// Everything required to create a particle spawner
#[derive(Bundle)]
pub struct ParticleSpawnerBundle<M: Particle2dMaterial> {
    /// holds the spawner state
    pub state: ParticleSpawnerState,
    /// particle effect handle
    pub effect: Handle<Particle2dEffect>,
    /// the spawners unique effect value,
    /// these can be modified at runtime,
    /// hot reloading the asset resets them to the original state
    pub effect_instance: ParticleEffectInstance,
    /// hold the particle data
    pub particle_store: ParticleStore,
    /// provided particle material
    pub material: Handle<M>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl<M: Particle2dMaterial + Default> Default for ParticleSpawnerBundle<M> {
    fn default() -> Self {
        Self {
            state: ParticleSpawnerState::default(),
            effect: Handle::default(),
            effect_instance: ParticleEffectInstance::default(),
            particle_store: ParticleStore::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            view_visibility: ViewVisibility::default(),
            material: Default::default(),
        }
    }
}
