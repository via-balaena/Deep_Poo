use bevy::prelude::*;
use bevy::ui::{
    AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection, JustifyContent,
    PositionType, UiRect, Val,
};

use crate::polyp::PolypTelemetry;
use crate::probe::TipSense;
use sim_core::controls::ControlParams;
use sim_core::recorder_types::RecorderState;
use vision_core::overlay::normalize_box;
use vision_runtime::{
    BurnInferenceState, DetectionOverlayState, DetectorHandle, DetectorKind, FrontCameraState,
};

#[derive(Component)]
pub struct ControlText;
#[derive(Component)]
pub struct DetectionOverlayRoot;
#[derive(Component)]
pub struct DetectionBoxUI;
#[derive(Component)]
pub struct DetectionLabelUI;
#[derive(Component)]
pub struct FallbackBanner;

pub fn spawn_controls_ui(mut commands: Commands) {
    let bg = Color::srgba(0.04, 0.08, 0.14, 0.82);
    let border = Color::srgba(0.0, 0.8, 0.75, 0.85);
    let accent = Color::srgba(0.28, 0.9, 1.0, 0.95);
    let line = Color::srgba(0.75, 0.88, 1.0, 0.9);
    let soft = Color::srgba(0.5, 0.74, 1.0, 0.85);

    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.5)),
            width: Val::Px(360.0),
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        BorderRadius::all(Val::Px(10.0)),
        Text::new("AUX // PROBE HUD\n───────────────\n"),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        TextColor(accent),
        ControlText,
        children![
            (
                TextSpan::from("TNS :: 0.50 [ [ ] ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("STF :: 500 [ ; ' ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("DMP :: 20 [ , . ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("THR :: 40 [ 1 2 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("SPD :: 1.20 [ 3 4 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("LIN :: 0.20 [ 5 6 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("FRI :: 1.20 [ 7 8 ]\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("TIP PRESS R/U/F: 0.0 / 0.0 / 0.0\n"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("TIP STEER >> 0.0 0.0 0.0 (0.0)"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(accent),
            ),
            (
                TextSpan::from("POLYPS :: 0/0\n"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(line),
            ),
            (
                TextSpan::from("NEAREST :: -- m"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("VISION :: cam=OFF burn=--\n"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(soft),
            ),
            (
                TextSpan::from("CONSENSUS :: hold"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(accent),
            ),
            (
                TextSpan::from("REC :: off\n"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgba(0.9, 0.2, 0.2, 0.95)),
            ),
            (
                TextSpan::from("REMOVAL :: idle"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(accent),
            ),
        ],
    ));
}

pub fn update_detection_overlay_ui(
    mut commands: Commands,
    overlay: Res<DetectionOverlayState>,
    root_q: Query<Entity, With<DetectionOverlayRoot>>,
    boxes_q: Query<Entity, With<DetectionBoxUI>>,
    labels_q: Query<Entity, With<DetectionLabelUI>>,
    mut fallback_q: Query<(&mut Node, &mut Text), With<FallbackBanner>>,
) {
    if overlay.is_changed() {
        for e in boxes_q.iter() {
            commands.entity(e).despawn();
        }
        for e in labels_q.iter() {
            commands.entity(e).despawn();
        }
        let Some(root) = root_q.iter().next() else {
            return;
        };
        let boxes = overlay
            .boxes
            .iter()
            .filter_map(|b| normalize_box(*b, overlay.size))
            .collect::<Vec<_>>();
        let mut labels = Vec::new();
        for (i, score) in overlay.scores.iter().enumerate() {
            labels.push((i, *score));
        }
        labels.sort_by(|a, b| b.1.total_cmp(&a.1));
        for (idx, score) in labels.iter().take(4) {
            let Some(px) = boxes.get(*idx) else {
                continue;
            };
            commands.entity(root).with_children(|parent| {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(px[1] as f32),
                        left: Val::Px(px[0] as f32),
                        width: Val::Px((px[2] - px[0]) as f32),
                        height: Val::Px((px[3] - px[1]) as f32),
                        border: UiRect::all(Val::Px(1.5)),
                        ..default()
                    },
                    BorderColor::all(Color::srgba(0.6, 1.0, 0.7, 0.8)),
                    BorderRadius::all(Val::Px(3.0)),
                    BackgroundColor(Color::srgba(0.1, 0.22, 0.1, 0.08)),
                    ZIndex(1),
                    DetectionBoxUI,
                ));
            });
            commands.entity(root).with_children(|parent| {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(px[1] as f32 - 22.0),
                        left: Val::Px(px[0] as f32),
                        padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgba(0.6, 1.0, 0.7, 0.8)),
                    BorderRadius::all(Val::Px(2.0)),
                    BackgroundColor(Color::srgba(0.08, 0.2, 0.08, 0.95)),
                    Text::new(format!("polyp {:.2}", score)),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.86, 0.95, 0.92)),
                    ZIndex(2),
                    DetectionLabelUI,
                ));
            });
        }
    }

    // Fallback banner if heuristic detector is active or no camera feed.
    let Ok((mut node, mut text)) = fallback_q.single_mut() else {
        return;
    };
    if let Some(fb) = &overlay.fallback {
        node.display = Display::Flex;
        text.0 = fb.clone().into();
    } else {
        node.display = Display::None;
    }
}

pub fn spawn_detection_overlay(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            width: Val::Px(320.0),
            height: Val::Px(240.0),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
        BorderColor::all(Color::srgba(0.0, 0.8, 0.75, 0.85)),
        BorderRadius::all(Val::Px(10.0)),
        ZIndex(200),
        DetectionOverlayRoot,
        children![(
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                display: Display::None,
                ..default()
            },
            BorderColor::all(Color::srgba(0.9, 0.75, 0.2, 0.85)),
            BorderRadius::all(Val::Px(3.0)),
            BackgroundColor(Color::srgba(0.3, 0.2, 0.05, 0.92)),
            Text::new("Heuristic detector active (Burn unavailable)"),
            TextFont {
                font_size: 13.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 0.92, 0.75, 0.95)),
            FallbackBanner,
        )],
    ));
}

pub fn update_controls_ui(
    control: Res<ControlParams>,
    telemetry: Option<Res<PolypTelemetry>>,
    tip: Option<Res<TipSense>>,
    recorder: Option<Res<RecorderState>>,
    camera: Option<Res<FrontCameraState>>,
    mut node_q: Query<&mut Text, With<ControlText>>,
    burn_state: Option<Res<BurnInferenceState>>,
    handle: Option<Res<DetectorHandle>>,
) {
    let Ok(mut text) = node_q.single_mut() else {
        return;
    };
    let mut lines = Vec::new();

    lines.push(format!(
        "TNS :: {:.2} [ [ ] ]\nSTF :: {:.0} [ ; ' ]\nDMP :: {:.0} [ , . ]",
        control.tension, control.stiffness, control.damping
    ));
    lines.push(format!(
        "THR :: {:.0} [ 1 2 ]\nSPD :: {:.2} [ 3 4 ]\nLIN :: {:.2} [ 5 6 ]\nFRI :: {:.2} [ 7 8 ]",
        control.thrust, control.target_speed, control.linear_damping, control.friction
    ));

    if let Some(t) = tip {
        lines.push(format!(
            "TIP PRESS R/U/F: {:.2} / {:.2} / {:.2}",
            t.pressure_world.x, t.pressure_world.y, t.pressure_world.z
        ));
        lines.push(format!(
            "TIP STEER >> {:.2} {:.2} {:.2} ({:.2})",
            t.steer_dir.x, t.steer_dir.y, t.steer_dir.z, t.steer_strength
        ));
    }

    if let Some(tele) = telemetry {
        lines.push(format!(
            "POLYPS :: {}/{}",
            tele.total.saturating_sub(tele.remaining),
            tele.total
        ));
        let nearest = tele.nearest_distance.unwrap_or(f32::MAX);
        if nearest < f32::MAX {
            lines.push(format!("NEAREST :: {:.2} m", nearest));
        } else {
            lines.push("NEAREST :: -- m".into());
        }
    }

    if let Some(cam) = camera {
        lines.push(format!(
            "VISION :: cam={} burn={}",
            if cam.active { "ON" } else { "OFF" },
            handle
                .as_ref()
                .map(|h| match h.kind {
                    DetectorKind::Burn => "Burn",
                    DetectorKind::Heuristic => "Heuristic",
                })
                .unwrap_or("N/A")
        ));
    }

    if let Some(rec) = recorder {
        lines.push(format!("REC :: {}", if rec.enabled { "on" } else { "off" }));
    }

    if let Some(burn) = burn_state {
        if let Some(r) = &burn.last_result {
            lines.push(format!(
                "CONSENSUS :: {:.2} ({} boxes)",
                r.confidence,
                r.boxes.len()
            ));
        } else {
            lines.push("CONSENSUS :: hold".into());
        }
    }

    text.0 = format!("AUX // PROBE HUD\n───────────────\n{}", lines.join("\n"));
}
