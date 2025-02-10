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
pub struct BevyTimeScriptingPlugin;
#[script_bindings(remote, name = "fixed")]
impl bevy::time::prelude::Fixed {
    fn clone(_self: Ref<bevy::time::prelude::Fixed>) {
        let output: Val<bevy::time::prelude::Fixed> = <bevy::time::prelude::Fixed as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
}
#[script_bindings(remote, name = "real")]
impl bevy::time::prelude::Real {
    fn clone(_self: Ref<bevy::time::prelude::Real>) {
        let output: Val<bevy::time::prelude::Real> = <bevy::time::prelude::Real as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
}
#[script_bindings(remote, name = "timer")]
impl bevy::time::prelude::Timer {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::time::prelude::Timer>) {
        let output: () = <bevy::time::prelude::Timer as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn clone(_self: Ref<bevy::time::prelude::Timer>) {
        let output: Val<bevy::time::prelude::Timer> = <bevy::time::prelude::Timer as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn duration(_self: Ref<bevy::time::prelude::Timer>) {
        let output: Val<bevy::utils::Duration> = bevy::time::prelude::Timer::duration(
                &_self,
            )
            .into();
        output
    }
    fn elapsed(_self: Ref<bevy::time::prelude::Timer>) {
        let output: Val<bevy::utils::Duration> = bevy::time::prelude::Timer::elapsed(
                &_self,
            )
            .into();
        output
    }
    fn elapsed_secs(_self: Ref<bevy::time::prelude::Timer>) {
        let output: f32 = bevy::time::prelude::Timer::elapsed_secs(&_self).into();
        output
    }
    fn elapsed_secs_f64(_self: Ref<bevy::time::prelude::Timer>) {
        let output: f64 = bevy::time::prelude::Timer::elapsed_secs_f64(&_self).into();
        output
    }
    fn eq(
        _self: Ref<bevy::time::prelude::Timer>,
        other: Ref<bevy::time::prelude::Timer>,
    ) {
        let output: bool = <bevy::time::prelude::Timer as std::cmp::PartialEq<
            bevy::time::prelude::Timer,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn finished(_self: Ref<bevy::time::prelude::Timer>) {
        let output: bool = bevy::time::prelude::Timer::finished(&_self).into();
        output
    }
    fn fraction(_self: Ref<bevy::time::prelude::Timer>) {
        let output: f32 = bevy::time::prelude::Timer::fraction(&_self).into();
        output
    }
    fn fraction_remaining(_self: Ref<bevy::time::prelude::Timer>) {
        let output: f32 = bevy::time::prelude::Timer::fraction_remaining(&_self).into();
        output
    }
    fn from_seconds(duration: f32, mode: Val<bevy::time::prelude::TimerMode>) {
        let output: Val<bevy::time::prelude::Timer> = bevy::time::prelude::Timer::from_seconds(
                duration,
                mode.into_inner(),
            )
            .into();
        output
    }
    fn just_finished(_self: Ref<bevy::time::prelude::Timer>) {
        let output: bool = bevy::time::prelude::Timer::just_finished(&_self).into();
        output
    }
    fn mode(_self: Ref<bevy::time::prelude::Timer>) {
        let output: Val<bevy::time::prelude::TimerMode> = bevy::time::prelude::Timer::mode(
                &_self,
            )
            .into();
        output
    }
    fn new(
        duration: Val<bevy::utils::Duration>,
        mode: Val<bevy::time::prelude::TimerMode>,
    ) {
        let output: Val<bevy::time::prelude::Timer> = bevy::time::prelude::Timer::new(
                duration.into_inner(),
                mode.into_inner(),
            )
            .into();
        output
    }
    fn pause(mut _self: Mut<bevy::time::prelude::Timer>) {
        let output: () = bevy::time::prelude::Timer::pause(&mut _self).into();
        output
    }
    fn paused(_self: Ref<bevy::time::prelude::Timer>) {
        let output: bool = bevy::time::prelude::Timer::paused(&_self).into();
        output
    }
    fn remaining(_self: Ref<bevy::time::prelude::Timer>) {
        let output: Val<bevy::utils::Duration> = bevy::time::prelude::Timer::remaining(
                &_self,
            )
            .into();
        output
    }
    fn remaining_secs(_self: Ref<bevy::time::prelude::Timer>) {
        let output: f32 = bevy::time::prelude::Timer::remaining_secs(&_self).into();
        output
    }
    fn reset(mut _self: Mut<bevy::time::prelude::Timer>) {
        let output: () = bevy::time::prelude::Timer::reset(&mut _self).into();
        output
    }
    fn set_duration(
        mut _self: Mut<bevy::time::prelude::Timer>,
        duration: Val<bevy::utils::Duration>,
    ) {
        let output: () = bevy::time::prelude::Timer::set_duration(
                &mut _self,
                duration.into_inner(),
            )
            .into();
        output
    }
    fn set_elapsed(
        mut _self: Mut<bevy::time::prelude::Timer>,
        time: Val<bevy::utils::Duration>,
    ) {
        let output: () = bevy::time::prelude::Timer::set_elapsed(
                &mut _self,
                time.into_inner(),
            )
            .into();
        output
    }
    fn set_mode(
        mut _self: Mut<bevy::time::prelude::Timer>,
        mode: Val<bevy::time::prelude::TimerMode>,
    ) {
        let output: () = bevy::time::prelude::Timer::set_mode(
                &mut _self,
                mode.into_inner(),
            )
            .into();
        output
    }
    fn times_finished_this_tick(_self: Ref<bevy::time::prelude::Timer>) {
        let output: u32 = bevy::time::prelude::Timer::times_finished_this_tick(&_self)
            .into();
        output
    }
    fn unpause(mut _self: Mut<bevy::time::prelude::Timer>) {
        let output: () = bevy::time::prelude::Timer::unpause(&mut _self).into();
        output
    }
}
#[script_bindings(remote, name = "timer_mode")]
impl bevy::time::prelude::TimerMode {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::time::prelude::TimerMode>) {
        let output: () = <bevy::time::prelude::TimerMode as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn clone(_self: Ref<bevy::time::prelude::TimerMode>) {
        let output: Val<bevy::time::prelude::TimerMode> = <bevy::time::prelude::TimerMode as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn eq(
        _self: Ref<bevy::time::prelude::TimerMode>,
        other: Ref<bevy::time::prelude::TimerMode>,
    ) {
        let output: bool = <bevy::time::prelude::TimerMode as std::cmp::PartialEq<
            bevy::time::prelude::TimerMode,
        >>::eq(&_self, &other)
            .into();
        output
    }
}
#[script_bindings(remote, name = "virtual")]
impl bevy::time::prelude::Virtual {
    fn clone(_self: Ref<bevy::time::prelude::Virtual>) {
        let output: Val<bevy::time::prelude::Virtual> = <bevy::time::prelude::Virtual as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
}
#[script_bindings(remote, name = "stopwatch")]
impl bevy::time::Stopwatch {
    fn assert_receiver_is_total_eq(_self: Ref<bevy::time::Stopwatch>) {
        let output: () = <bevy::time::Stopwatch as std::cmp::Eq>::assert_receiver_is_total_eq(
                &_self,
            )
            .into();
        output
    }
    fn clone(_self: Ref<bevy::time::Stopwatch>) {
        let output: Val<bevy::time::Stopwatch> = <bevy::time::Stopwatch as std::clone::Clone>::clone(
                &_self,
            )
            .into();
        output
    }
    fn elapsed(_self: Ref<bevy::time::Stopwatch>) {
        let output: Val<bevy::utils::Duration> = bevy::time::Stopwatch::elapsed(&_self)
            .into();
        output
    }
    fn elapsed_secs(_self: Ref<bevy::time::Stopwatch>) {
        let output: f32 = bevy::time::Stopwatch::elapsed_secs(&_self).into();
        output
    }
    fn elapsed_secs_f64(_self: Ref<bevy::time::Stopwatch>) {
        let output: f64 = bevy::time::Stopwatch::elapsed_secs_f64(&_self).into();
        output
    }
    fn eq(_self: Ref<bevy::time::Stopwatch>, other: Ref<bevy::time::Stopwatch>) {
        let output: bool = <bevy::time::Stopwatch as std::cmp::PartialEq<
            bevy::time::Stopwatch,
        >>::eq(&_self, &other)
            .into();
        output
    }
    fn is_paused(_self: Ref<bevy::time::Stopwatch>) {
        let output: bool = bevy::time::Stopwatch::is_paused(&_self).into();
        output
    }
    fn new() {
        let output: Val<bevy::time::Stopwatch> = bevy::time::Stopwatch::new().into();
        output
    }
    fn pause(mut _self: Mut<bevy::time::Stopwatch>) {
        let output: () = bevy::time::Stopwatch::pause(&mut _self).into();
        output
    }
    fn reset(mut _self: Mut<bevy::time::Stopwatch>) {
        let output: () = bevy::time::Stopwatch::reset(&mut _self).into();
        output
    }
    fn set_elapsed(
        mut _self: Mut<bevy::time::Stopwatch>,
        time: Val<bevy::utils::Duration>,
    ) {
        let output: () = bevy::time::Stopwatch::set_elapsed(
                &mut _self,
                time.into_inner(),
            )
            .into();
        output
    }
    fn unpause(mut _self: Mut<bevy::time::Stopwatch>) {
        let output: () = bevy::time::Stopwatch::unpause(&mut _self).into();
        output
    }
}
impl ::bevy::app::Plugin for BevyTimeScriptingPlugin {
    fn build(&self, app: &mut ::bevy::prelude::App) {
        let mut world = app.world_mut();
        register_fixed(&mut world);
        register_real(&mut world);
        register_timer(&mut world);
        register_timer_mode(&mut world);
        register_virtual(&mut world);
        register_stopwatch(&mut world);
    }
}
