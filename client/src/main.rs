use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audition Clone".to_string(),
                resolution: (800, 600).into(),
                resizable: false, 
                ..default()
            }),
            ..default()
        }))
        // Le decimos a Bevy que ejecute esta función al arrancar
        .add_systems(Startup, configurar_pantalla_inicio)
        .run(); 
}

// "Commands" es la herramienta de Bevy para crear (spawn) o destruir entidades
fn configurar_pantalla_inicio(mut commands: Commands) {
    // 1. Instanciamos la cámara 2D
    commands.spawn(Camera2d);

    // 2. Creamos un contenedor global (pantalla completa) para centrar todo
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    })
    .with_children(|padre| {
        // 3. Creamos el recuadro gris oscuro del Login
        padre.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(250.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderRadius::MAX, // Bordes redondeados
        ))
        .with_children(|caja| {
            // 4. Texto del título
            caja.spawn((
                Text::new("Ingreso a Audition"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Aquí añadiremos el input de texto y el botón más adelante
            caja.spawn((
                Text::new("[ Espacio para Input Usuario ]"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));

            caja.spawn((
                Text::new("[ Espacio para Select Genero ]"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
    });
}