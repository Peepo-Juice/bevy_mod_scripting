// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use bevy_mod_scripting_core::bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use crate::*;
pub struct BevyHierarchyScriptingPlugin;
#[script_bindings(remote, name = "children")]
impl bevy::hierarchy::prelude::Children {
    fn swap(
        mut _self: Mut<bevy::hierarchy::prelude::Children>,
        a_index: usize,
        b_index: usize,
    ) {
        let output: () = bevy::hierarchy::prelude::Children::swap(
                &mut _self,
                a_index,
                b_index,
            )
            .into();
        output
    }
}
#[script_bindings(remote, name = "parent")]
impl bevy::hierarchy::prelude::Parent {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::hierarchy::prelude::Parent>) {
        let output: () = <bevy::hierarchy::prelude::Parent as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn eq(
        _self: Ref<bevy::hierarchy::prelude::Parent>,
        other: Ref<bevy::hierarchy::prelude::Parent>,
    ) {
        let output: bool = <bevy::hierarchy::prelude::Parent as std::cmp::PartialEq<
            bevy::hierarchy::prelude::Parent,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn get(_self: Ref<bevy::hierarchy::prelude::Parent>) {
        let output: Val<bevy::ecs::entity::Entity> = bevy::hierarchy::prelude::Parent::get(
                &_self,
            )
            .into();
        output
    }
}
#[script_bindings(remote, name = "hierarchy_event")]
impl bevy::hierarchy::HierarchyEvent {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::hierarchy::HierarchyEvent>) {
        let output: () = <bevy::hierarchy::HierarchyEvent as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn clone(_self: Ref<bevy::hierarchy::HierarchyEvent>) {
        let output: Val<bevy::hierarchy::HierarchyEvent> = <bevy::hierarchy::HierarchyEvent as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn eq(
        _self: Ref<bevy::hierarchy::HierarchyEvent>,
        other: Ref<bevy::hierarchy::HierarchyEvent>,
    ) {
        let output: bool = <bevy::hierarchy::HierarchyEvent as std::cmp::PartialEq<
            bevy::hierarchy::HierarchyEvent,
        >>::eq(&_self, &other)
            .into();
        output
    }
}
impl ::bevy::app::Plugin for BevyHierarchyScriptingPlugin {
    fn build(&self, app: &mut ::bevy::prelude::App) {
        let mut world = app.world_mut();
        register_children(&mut world);
        register_parent(&mut world);
        register_hierarchy_event(&mut world);
    }
}
