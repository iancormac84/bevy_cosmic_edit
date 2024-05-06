#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_cosmic_edit::*;

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
        ("R", attrs.color(Color::srgb(1.0, 0.0, 0.0).to_cosmic())),
        ("A", attrs.color(Color::srgb(1.0, 0.65, 0.0).to_cosmic())),
        ("I", attrs.color(Color::srgb(1.0, 1.0, 0.0).to_cosmic())),
        ("N", attrs.color(Color::srgb(0.0, 1.0, 0.0).to_cosmic())),
        ("B", attrs.color(Color::srgb(0.0, 0.0, 1.0).to_cosmic())),
        ("O", attrs.color(Color::srgb(0.29, 0.0, 0.51).to_cosmic())),
        ("W ", attrs.color(Color::srgb(0.5, 0.0, 0.5).to_cosmic())),
        ("Red ", attrs.color(Color::srgb(1.0, 0.0, 0.0).to_cosmic())),
        ("Orange ", attrs.color(Color::srgb(1.0, 0.65, 0.0).to_cosmic())),
        ("Yellow ", attrs.color(Color::srgb(1.0, 1.0, 0.0).to_cosmic())),
        ("Green ", attrs.color(Color::srgb(0.0, 1.0, 0.0).to_cosmic())),
        ("Blue ", attrs.color(Color::srgb(0.0, 0.0, 1.0).to_cosmic())),
        ("Indigo ", attrs.color(Color::srgb(0.29, 0.0, 0.51).to_cosmic())),
        ("Violet ", attrs.color(Color::srgb(0.5, 0.0, 0.5).to_cosmic())),
        ("U", attrs.color(Color::srgb(0.5, 0.0, 0.5).to_cosmic())),
        ("N", attrs.color(Color::srgb(0.29, 0.0, 0.51).to_cosmic())),
        ("I", attrs.color(Color::srgb(0.0, 0.0, 1.0).to_cosmic())),
        ("C", attrs.color(Color::srgb(0.0, 1.0, 0.0).to_cosmic())),
        ("O", attrs.color(Color::srgb(1.0, 1.0, 0.0).to_cosmic())),
        ("R", attrs.color(Color::srgb(1.0, 0.65, 0.0).to_cosmic())),
        ("N", attrs.color(Color::srgb(1.0, 0.0, 0.0).to_cosmic())),
        ("生活,삶,जिंदगी 😀 FPS", attrs.color(Color::srgb(1.0, 0.0, 0.0).to_cosmic())),
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
    attrs_2.color_opt = Some(Color::srgb(0.5, 0.0, 0.5).to_cosmic());

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

    // Spawn the CosmicEditUiBundles as children of root
    commands.entity(root).with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    ..default()
                },
                image: UiImage::default().with_color(Color::WHITE),
                ..default()
            })
            .insert(CosmicSource(cosmic_edit_1));

        parent
            .spawn(ButtonBundle {
                image: UiImage::default().with_color(Color::WHITE.with_alpha(0.8)),
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
