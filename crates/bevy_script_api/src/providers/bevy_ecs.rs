// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_reflect::*;
extern crate self as bevy_script_api;
use bevy_script_api::{
    lua::RegisterForeignLuaType, ReflectedValue, common::bevy::GetWorld,
};
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::ecs::entity::Entity",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &entity::Entity) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::ecs::entity::Entity;

"#,
    r#"
/// Creates a new entity ID with the specified `index` and a generation of 1.
/// # Note
/// Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
/// [`Commands::spawn`](crate::system::Commands::spawn). This method should generally
/// only be used for sharing entities across apps, and only when they have a scheme
/// worked out to share an index space (which doesn't happen by default).
/// In general, one should not try to synchronize the ECS by attempting to ensure that
/// `Entity` lines up between instances, but instead insert a secondary identifier as
/// a component.

    #[lua(kind = "Function", output(proxy))]
    fn from_raw(index: u32) -> bevy::ecs::entity::Entity;

"#,
    r#"
/// Convert to a form convenient for passing outside of rust.
/// Only useful for identifying entities within the same instance of an application. Do not use
/// for serialization between runs.
/// No particular structure is guaranteed for the returned bits.

    #[lua(kind = "Method")]
    fn to_bits(self) -> u64;

"#,
    r#"
/// Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
/// Only useful when applied to results from `to_bits` in the same instance of an application.
/// # Panics
/// This method will likely panic if given `u64` values that did not come from [`Entity::to_bits`].

    #[lua(kind = "Function", output(proxy))]
    fn from_bits(bits: u64) -> bevy::ecs::entity::Entity;

"#,
    r#"
/// Return a transiently unique identifier.
/// No two simultaneously-live entities share the same index, but dead entities' indices may collide
/// with both live and dead entities. Useful for compactly representing entities within a
/// specific snapshot of the world, such as when serializing.

    #[lua(kind = "Method")]
    fn index(self) -> u32;

"#,
    r#"
/// Returns the generation of this Entity's index. The generation is incremented each time an
/// entity with a given index is despawned. This serves as a "count" of the number of times a
/// given index has been reused (index, generation) pairs uniquely identify a given Entity.

    #[lua(kind = "Method")]
    fn generation(self) -> u32;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Entity {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::ecs::component::ComponentId",
    functions[r#"
/// Creates a new [`ComponentId`].
/// The `index` is a unique value associated with each type of component in a given world.
/// Usually, this value is taken from a counter incremented for each type of component registered with the world.

    #[lua(kind = "Function", output(proxy))]
    fn new(index: usize) -> bevy::ecs::component::ComponentId;

"#,
    r#"
/// Returns the index of the current component.

    #[lua(kind = "Method")]
    fn index(self) -> usize;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &component::ComponentId) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::ecs::component::ComponentId;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct ComponentId();
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::ecs::component::Tick",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::ecs::component::Tick;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a new [`Tick`] wrapping the given value.

    #[lua(kind = "Function", output(proxy))]
    fn new(tick: u32) -> bevy::ecs::component::Tick;

"#,
    r#"
/// Gets the value of this change tick.

    #[lua(kind = "Method")]
    fn get(self) -> u32;

"#,
    r#"
/// Sets the value of this change tick.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, tick: u32) -> ();

"#,
    r#"
/// Returns `true` if this `Tick` occurred since the system's `last_run`.
/// `this_run` is the current tick of the system, used as a reference to help deal with wraparound.

    #[lua(kind = "Method")]
    fn is_newer_than(
        self,
        #[proxy]
        last_run: bevy::ecs::component::Tick,
        #[proxy]
        this_run: bevy::ecs::component::Tick,
    ) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &component::Tick) -> bool;

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Tick {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::ecs::component::ComponentTicks",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::ecs::component::ComponentTicks;

"#,
    r#"
/// Returns `true` if the component or resource was added after the system last ran.

    #[lua(kind = "Method")]
    fn is_added(
        &self,
        #[proxy]
        last_run: bevy::ecs::component::Tick,
        #[proxy]
        this_run: bevy::ecs::component::Tick,
    ) -> bool;

"#,
    r#"
/// Returns `true` if the component or resource was added or mutably dereferenced after the system last ran.

    #[lua(kind = "Method")]
    fn is_changed(
        &self,
        #[proxy]
        last_run: bevy::ecs::component::Tick,
        #[proxy]
        this_run: bevy::ecs::component::Tick,
    ) -> bool;

"#,
    r#"
/// Returns the tick recording the time this component or resource was most recently changed.

    #[lua(kind = "Method", output(proxy))]
    fn last_changed_tick(&self) -> bevy::ecs::component::Tick;

"#,
    r#"
/// Returns the tick recording the time this component or resource was added.

    #[lua(kind = "Method", output(proxy))]
    fn added_tick(&self) -> bevy::ecs::component::Tick;

"#,
    r#"
/// Manually sets the change tick.
/// This is normally done automatically via the [`DerefMut`](std::ops::DerefMut) implementation
/// on [`Mut<T>`](crate::change_detection::Mut), [`ResMut<T>`](crate::change_detection::ResMut), etc.
/// However, components and resources that make use of interior mutability might require manual updates.
/// # Example
/// ```no_run
/// # use bevy_ecs::{world::World, component::ComponentTicks};
/// let world: World = unimplemented!();
/// let component_ticks: ComponentTicks = unimplemented!();
/// component_ticks.set_changed(world.read_change_tick());
/// ```

    #[lua(kind = "MutatingMethod")]
    fn set_changed(&mut self, #[proxy] change_tick: bevy::ecs::component::Tick) -> ();

"#,
    r#"
#[lua(kind="MetaMethod", metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct ComponentTicks {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::ecs::entity::EntityHash",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::ecs::entity::EntityHash;

"#]
)]
pub struct EntityHash {}
#[derive(Default)]
pub(crate) struct Globals;
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<
        'lua,
        T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>,
    >(self, instances: &mut T) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance(
                "Entity",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaEntity>::new,
            )?;
        instances
            .add_instance(
                "ComponentId",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaComponentId>::new,
            )?;
        instances
            .add_instance(
                "Tick",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTick>::new,
            )?;
        Ok(())
    }
}
pub struct BevyEcsAPIProvider;
impl bevy_mod_scripting_core::hosts::APIProvider for BevyEcsAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;
    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(
                e.to_string(),
            ))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(
            bevy_mod_scripting_lua::docs::LuaDocFragment::new(
                "BevyEcsAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaEntity>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaEntity>,
                        >()
                        .process_type::<LuaComponentId>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaComponentId,
                            >,
                        >()
                        .process_type::<LuaTick>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaTick>,
                        >()
                        .process_type::<LuaComponentTicks>()
                        .process_type::<LuaEntityHash>()
                },
            ),
        )
    }
    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::ecs::entity::Entity>();
        app.register_foreign_lua_type::<bevy::ecs::component::ComponentId>();
        app.register_foreign_lua_type::<bevy::ecs::component::Tick>();
        app.register_foreign_lua_type::<bevy::ecs::component::ComponentTicks>();
        app.register_foreign_lua_type::<bevy::ecs::entity::EntityHash>();
    }
}
