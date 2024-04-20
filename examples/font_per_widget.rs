#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_cosmic_edit::*;
use util::{bevy_color_to_cosmic, change_active_editor_ui, deselect_editor_on_esc};

fn setup(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
    commands.spawn(Camera2dBundle::default());
    let root = commands
        .spawn(NodeBundle {
            style: bevy::prelude::Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();

    let attrs = Attrs::new();
    let serif_attrs = attrs.family(Family::Serif);
    let mono_attrs = attrs.family(Family::Monospace);
    let comic_attrs = attrs.family(Family::Name("Comic Neue"));
    let lines = vec![
        ("B", attrs.weight(FontWeight::BOLD)),
        ("old ", attrs),
        ("I", attrs.style(FontStyle::Italic)),
        ("talic ", attrs),
        ("f", attrs),
        ("i ", attrs),
        ("f", attrs.weight(FontWeight::BOLD)),
        ("i ", attrs),
        ("f", attrs.style(FontStyle::Italic)),
        ("i ", attrs),
        ("Sans-Serif Normal ", attrs),
        ("Sans-Serif Bold ", attrs.weight(FontWeight::BOLD)),
        ("Sans-Serif Italic ", attrs.style(FontStyle::Italic)),
        (
            "Sans-Serif Bold Italic",
            attrs.weight(FontWeight::BOLD).style(FontStyle::Italic),
        ),
        ("Serif Normal ", serif_attrs),
        ("Serif Bold ", serif_attrs.weight(FontWeight::BOLD)),
        ("Serif Italic ", serif_attrs.style(FontStyle::Italic)),
        (
            "Serif Bold Italic",
            serif_attrs
                .weight(FontWeight::BOLD)
                .style(FontStyle::Italic),
        ),
        ("\n", attrs),
        ("Mono Normal ", mono_attrs),
        ("Mono Bold ", mono_attrs.weight(FontWeight::BOLD)),
        ("Mono Italic ", mono_attrs.style(FontStyle::Italic)),
        (
            "Mono Bold Italic",
            mono_attrs.weight(FontWeight::BOLD).style(FontStyle::Italic),
        ),
        ("Comic Normal ", comic_attrs),
        ("Comic Bold ", comic_attrs.weight(FontWeight::BOLD)),
        ("Comic Italic ", comic_attrs.style(FontStyle::Italic)),
        (
            "Comic Bold Italic",
            comic_attrs
                .weight(FontWeight::BOLD)
                .style(FontStyle::Italic),
        ),
        ("\n", attrs),
        ("R", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.0, 0.0)))),
        ("A", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.65, 0.0)))),
        ("I", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 1.0, 0.0)))),
        ("N", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.0, 1.0, 0.0)))),
        ("B", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.0, 0.0, 1.0)))),
        ("O", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.29, 0.0, 0.51)))),
        ("W ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.5, 0.0, 0.5)))),
        ("Red ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.0, 0.0)))),
        ("Orange ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.65, 0.0)))),
        ("Yellow ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 1.0, 0.0)))),
        ("Green ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.0, 1.0, 0.0)))),
        ("Blue ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.0, 0.0, 1.0)))),
        ("Indigo ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.29, 0.0, 0.51)))),
        ("Violet ", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.5, 0.0, 0.5)))),
        ("U", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.5, 0.0, 0.5)))),
        ("N", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.29, 0.0, 0.51)))),
        ("I", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.0, 0.0, 1.0)))),
        ("C", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(0.0, 1.0, 0.0)))),
        ("O", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 1.0, 0.0)))),
        ("R", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.65, 0.0)))),
        ("N", attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.0, 0.0)))),
        (
            "生活,삶,जिंदगी 😀 FPS",
            attrs.color(bevy_color_to_cosmic(Color::linear_rgb(1.0, 0.0, 0.0))),
        ),
    ];

    let cosmic_edit_1 = commands
        .spawn(CosmicEditBundle {
            buffer: CosmicBuffer::new(&mut font_system, Metrics::new(18., 22.)).with_rich_text(
                &mut font_system,
                lines,
                attrs,
            ),
            ..default()
        })
        .id();

    let mut attrs_2 = Attrs::new();
    attrs_2 = attrs_2.family(Family::Name("Times New Roman"));
    attrs_2.color_opt = Some(bevy_color_to_cosmic(Color::linear_rgb(0.5, 0.0, 0.5)));

    let cosmic_edit_2 = commands
        .spawn(CosmicEditBundle {
            buffer: CosmicBuffer::new(&mut font_system, Metrics::new(28., 36.)).with_text(
                &mut font_system,
                "Widget 2.\nClick on me =>",
                attrs_2,
            ),
            ..default()
        })
        .id();

    // Spawn the CosmicEditBundles as children of root
    commands.entity(root).with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            })
            .insert(CosmicSource(cosmic_edit_1));

        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            })
            .insert(CosmicSource(cosmic_edit_2));
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin { ..default() })
        .add_systems(Startup, setup)
        .add_systems(Update, (change_active_editor_ui, deselect_editor_on_esc))
        .run();
}
