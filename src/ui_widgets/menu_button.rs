use bevy::{
    color::palettes::basic::*,
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    reflect::Is,
    ui::{InteractionDisabled, Pressed},
    ui_widgets::Button,
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
const CHECKBOX_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);
const CHECKBOX_CHECK: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn menu_button_plugin(app: &mut App) {
    app.add_observer(button_on_interaction::<Add, Pressed>)
        .add_observer(button_on_interaction::<Remove, Pressed>)
        .add_observer(button_on_interaction::<Add, InteractionDisabled>)
        .add_observer(button_on_interaction::<Remove, InteractionDisabled>)
        .add_observer(button_on_interaction::<Insert, Hovered>);
}

/// Marker which identifies buttons with a particular style, in this case the "Demo style".
#[derive(Component)]
struct MenuButton;

pub fn menu_button(asset_server: &AssetServer, label: String) -> impl Bundle {
    (
        Node {
            width: px(300),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MenuButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(label),
            TextFont {
                //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

fn button_on_interaction<E: EntityEvent, C: Component>(
    event: On<E, C>,
    mut buttons: Query<
        (
            &Hovered,
            Has<InteractionDisabled>,
            Has<Pressed>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<MenuButton>,
    >,
    mut text_query: Query<&mut Text>,
) {
    if let Ok((hovered, disabled, pressed, mut color, mut border_color, children)) =
        buttons.get_mut(event.event_target())
    {
        if children.is_empty() {
            return;
        }
        let Ok(mut text) = text_query.get_mut(children[0]) else {
            return;
        };
        let hovered = hovered.get();
        // These "removal event checks" exist because the `Remove` event is triggered _before_ the component is actually
        // removed, meaning it still shows up in the query. We're investigating the best way to improve this scenario.
        let pressed = pressed && !(E::is::<Remove>() && C::is::<Pressed>());
        let disabled = disabled && !(E::is::<Remove>() && C::is::<InteractionDisabled>());
        match (disabled, hovered, pressed) {
            // Disabled button
            (true, _, _) => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(GRAY);
            }

            // Pressed and hovered button
            (false, true, true) => {
                *color = PRESSED_BUTTON.into();
                border_color.set_all(RED);
            }

            // Hovered, unpressed button
            (false, true, false) => {
                *color = HOVERED_BUTTON.into();
                border_color.set_all(WHITE);
            }

            // Unhovered button (either pressed or not).
            (false, false, _) => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(BLACK);
            }
        }
    }
}
