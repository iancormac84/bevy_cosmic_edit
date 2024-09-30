// This will all be rewritten soon, looking toward per-widget cursor control
// Rewrite should address issue #93 too

use crate::*;
use bevy::{input::mouse::MouseMotion, prelude::*, render::view::cursor::CursorIcon, window::PrimaryWindow};

/// System set for mouse cursor systems. Runs in [`Update`]
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CursorSet;

pub struct CursorPlugin;

/// Unit resource whose existence in the world disables the cursor plugin systems.
#[derive(Resource)]
pub struct CursorPluginDisabled;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((hover_sprites, hover_ui), change_cursor)
                .chain()
                .run_if(not(resource_exists::<CursorPluginDisabled>)),
        )
        .add_event::<TextHoverIn>()
        .add_event::<TextHoverOut>();
    }
}

/// For use with custom cursor control
/// Event is emitted when cursor enters a text widget
#[derive(Event)]
pub struct TextHoverIn;

/// For use with custom cursor control
/// Event is emitted when cursor leaves a text widget
#[derive(Event)]
pub struct TextHoverOut;

pub(crate) fn change_cursor(
    mut commands: Commands,
    mut evr_hover_in: EventReader<TextHoverIn>,
    evr_hover_out: EventReader<TextHoverOut>,
    evr_text_changed: EventReader<CosmicTextChanged>,
    evr_mouse_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    cursor: Query<&CursorIcon, With<Window>>,
) {
    if windows.iter().len() == 0 {
        return;
    }
    let (window_entity, mut window) = windows.single_mut();

    if let Some(_ev) = evr_hover_in.read().last() {
        if let Err(_) = cursor.get_single() {
            commands.entity(window_entity).insert();
        }
    } else if !evr_hover_out.is_empty() {
        commands.entity(window_entity).remove::<CursorIcon>();
    }

    if !evr_text_changed.is_empty() {
        window.cursor_options.visible = false;
    }

    if mouse_buttons.get_just_pressed().len() != 0 || !evr_mouse_motion.is_empty() {
        window.cursor_options.visible = true;
    }
}

#[cfg(feature = "multicam")]
type CameraQuery<'a, 'b, 'c, 'd> =
    Query<'a, 'b, (&'c Camera, &'d GlobalTransform), With<CosmicPrimaryCamera>>;

#[cfg(not(feature = "multicam"))]
type CameraQuery<'a, 'b, 'c, 'd> = Query<'a, 'b, (&'c Camera, &'d GlobalTransform)>;

pub(crate) fn hover_sprites(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_edit_query: Query<
        (&mut Sprite, &Visibility, &GlobalTransform, &CursorIcon),
        With<CosmicBuffer>,
    >,
    camera_q: CameraQuery,
    mut hovered: Local<bool>,
    mut last_hovered: Local<bool>,
    mut evw_hover_in: EventWriter<TextHoverIn>,
    mut evw_hover_out: EventWriter<TextHoverOut>,
) {
    *hovered = false;
    if windows.iter().len() == 0 {
        return;
    }
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    for (sprite, visibility, node_transform, _hover) in &mut cosmic_edit_query.iter_mut() {
        if visibility == Visibility::Hidden {
            continue;
        }

        let size = sprite.custom_size.unwrap_or(Vec2::ONE);
        let x_min = node_transform.affine().translation.x - size.x / 2.;
        let y_min = node_transform.affine().translation.y - size.y / 2.;
        let x_max = node_transform.affine().translation.x + size.x / 2.;
        let y_max = node_transform.affine().translation.y + size.y / 2.;
        if let Some(pos) = window.cursor_position() {
            if let Ok(pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                if x_min < pos.x && pos.x < x_max && y_min < pos.y && pos.y < y_max {
                    *hovered = true;
                }
            }
        }
    }

    if *last_hovered != *hovered {
        if *hovered {
            evw_hover_in.send(TextHoverIn);
        } else {
            evw_hover_out.send(TextHoverOut);
        }
    }

    *last_hovered = *hovered;
}

pub(crate) fn hover_ui(
    interaction_query: Query<(&Interaction, &CosmicSource), Changed<Interaction>>,
    cosmic_query: Query<&CursorIcon, With<CosmicBuffer>>,
    mut evw_hover_in: EventWriter<TextHoverIn>,
    mut evw_hover_out: EventWriter<TextHoverOut>,
) {
    for (interaction, source) in interaction_query.iter() {
        match interaction {
            Interaction::None => {
                evw_hover_out.send(TextHoverOut);
            }
            Interaction::Hovered => {
                if let Ok(_hover) = cosmic_query.get(source.0) {
                    evw_hover_in.send(TextHoverIn);
                }
            }
            _ => {}
        }
    }
}
