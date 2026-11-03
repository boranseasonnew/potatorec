use super::{Page, Theme};
use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, RichText, Stroke, Vec2};

pub const PURPLE: Color32 = Color32::from_rgb(174, 153, 255);
pub const MINT: Color32 = Color32::from_rgb(108, 224, 192);
#[derive(Clone, Copy)]
pub struct Palette {
    pub background: Color32,
    pub sidebar: Color32,
    pub surface: Color32,
    pub raised: Color32,
    pub border: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub tint: Color32,
}
pub fn palette(theme: Theme) -> Palette {
    if theme == Theme::Dark {
        Palette {
            background: Color32::from_rgb(15, 17, 24),
            sidebar: Color32::from_rgb(19, 21, 30),
            surface: Color32::from_rgb(24, 27, 38),
            raised: Color32::from_rgb(30, 33, 46),
            border: Color32::from_rgb(42, 45, 61),
            text: Color32::from_rgb(236, 237, 246),
            muted: Color32::from_rgb(130, 137, 160),
            tint: Color32::from_rgb(38, 32, 59),
        }
    } else {
        Palette {
            background: Color32::from_rgb(244, 244, 250),
            sidebar: Color32::from_rgb(253, 253, 255),
            surface: Color32::WHITE,
            raised: Color32::from_rgb(239, 238, 248),
            border: Color32::from_rgb(226, 225, 239),
            text: Color32::from_rgb(33, 31, 51),
            muted: Color32::from_rgb(122, 120, 143),
            tint: Color32::from_rgb(235, 229, 252),
        }
    }
}
pub fn display(text: impl Into<String>, size: f32) -> RichText {
    RichText::new(text).font(FontId::new(size, FontFamily::Name("Display".into())))
}
pub fn eyebrow(ui: &mut egui::Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .size(10.)
            .extra_letter_spacing(1.7)
            .color(ui.visuals().weak_text_color()),
    );
}
pub fn badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    egui::Frame::new()
        .fill(color.gamma_multiply(0.10))
        .stroke(Stroke::new(1_f32, color.gamma_multiply(0.22)))
        .corner_radius(20.)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(11.).color(color));
        });
}
pub fn potato(p: &egui::Painter, r: Rect) {
    let c = r.center();
    p.rect_filled(r, 11., Color32::from_rgb(161, 136, 245));
    let body = Rect::from_center_size(c, Vec2::new(r.width() * 0.48, r.height() * 0.62));
    p.add(egui::epaint::EllipseShape::filled(
        body.center(),
        body.size() * 0.5,
        Color32::from_rgb(255, 217, 154),
    ));
    let eye = Color32::from_rgb(79, 53, 67);
    p.circle_filled(c + Vec2::new(-3.5, -1.), 1.6, eye);
    p.circle_filled(c + Vec2::new(3.5, -1.), 1.6, eye);
    p.line_segment(
        [c + Vec2::new(-2., 5.), c + Vec2::new(2., 5.)],
        Stroke::new(1.3_f32, eye),
    );
    p.circle_filled(
        c + Vec2::new(-4., -7.),
        1.,
        Color32::from_rgb(217, 165, 109),
    );
}
pub fn icon(p: &egui::Painter, r: Rect, page: Page, color: Color32) {
    let c = r.center();
    let s = Stroke::new(1.6_f32, color);
    let boxr = Rect::from_center_size(c, Vec2::splat(16.));
    match page {
        Page::Dashboard => {
            for (x, y) in [(-5., -5.), (5., -5.), (-5., 5.), (5., 5.)] {
                p.rect_stroke(
                    Rect::from_center_size(c + Vec2::new(x, y), Vec2::splat(6.)),
                    1.5,
                    s,
                    egui::StrokeKind::Inside,
                );
            }
        }
        Page::Recording => {
            p.circle_stroke(c, 8., s);
            p.circle_filled(c, 3., color);
        }
        Page::Video => {
            p.rect_stroke(
                Rect::from_center_size(c - Vec2::new(0., 2.), Vec2::new(20., 13.)),
                2.,
                s,
                egui::StrokeKind::Inside,
            );
            p.line_segment([c + Vec2::new(0., 5.), c + Vec2::new(0., 9.)], s);
            p.line_segment([c + Vec2::new(-5., 9.), c + Vec2::new(5., 9.)], s);
        }
        Page::Audio => {
            for (x, h) in [(-8., 3.), (-4., 6.), (0., 10.), (4., 6.), (8., 3.)] {
                p.line_segment([c + Vec2::new(x, -h), c + Vec2::new(x, h)], s);
            }
        }
        Page::Performance => {
            p.add(egui::Shape::line(
                vec![
                    c + Vec2::new(-10., 6.),
                    c + Vec2::new(-4., -1.),
                    c + Vec2::new(1., 3.),
                    c + Vec2::new(9., -8.),
                ],
                s,
            ));
            p.line_segment([c + Vec2::new(3., -8.), c + Vec2::new(9., -8.)], s);
            p.line_segment([c + Vec2::new(9., -8.), c + Vec2::new(9., -2.)], s);
        }
        Page::Hotkeys => {
            p.rect_stroke(
                Rect::from_center_size(c, Vec2::new(22., 15.)),
                3.,
                s,
                egui::StrokeKind::Inside,
            );
            for x in [-6., 0., 6.] {
                for y in [-3., 1.] {
                    p.circle_filled(c + Vec2::new(x, y), 1., color);
                }
            }
            p.line_segment([c + Vec2::new(-5., 5.), c + Vec2::new(5., 5.)], s);
        }
        Page::Recordings => {
            p.rect_stroke(
                boxr.expand2(Vec2::new(3., 0.)),
                3.,
                s,
                egui::StrokeKind::Inside,
            );
            p.line_segment([c + Vec2::new(-8., -4.), c + Vec2::new(-1., -4.)], s);
            p.line_segment([c + Vec2::new(-1., -4.), c + Vec2::new(2., -7.)], s);
        }
        Page::Settings => {
            p.circle_stroke(c, 6., s);
            p.circle_stroke(c, 2., s);
            for i in 0..6 {
                let a = i as f32 * std::f32::consts::TAU / 6.;
                let d = Vec2::angled(a);
                p.line_segment([c + d * 7., c + d * 10.], s);
            }
        }
    }
}
pub fn nav(
    ui: &mut egui::Ui,
    page: Page,
    label: &str,
    selected: bool,
    colors: Palette,
) -> egui::Response {
    let (r, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 43.), egui::Sense::click());
    let p = ui.painter();
    if selected || response.hovered() {
        p.rect_filled(r, 9., if selected { colors.tint } else { colors.raised });
    }
    if selected {
        p.rect_filled(
            Rect::from_min_size(r.left_center() - Vec2::new(0., 9.), Vec2::new(3., 18.)),
            2.,
            PURPLE,
        );
    }
    let color = if selected { PURPLE } else { colors.muted };
    icon(
        p,
        Rect::from_center_size(r.left_center() + Vec2::new(24., 0.), Vec2::splat(24.)),
        page,
        color,
    );
    p.text(
        r.left_center() + Vec2::new(47., 0.),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(14.),
        if selected { colors.text } else { colors.muted },
    );
    if selected {
        p.circle_filled(r.right_center() - Vec2::new(17., 0.), 2.5, PURPLE);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
pub fn toggle(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let (r, response) = ui.allocate_exact_size(Vec2::new(34., 20.), egui::Sense::click());
    if response.clicked() {
        *on = !*on;
    }
    let p = ui.painter();
    p.rect_filled(
        r,
        10.,
        if *on {
            Color32::from_rgb(155, 128, 239)
        } else {
            ui.visuals().widgets.inactive.bg_fill
        },
    );
    p.circle_filled(
        Pos2::new(
            if *on { r.right() - 10. } else { r.left() + 10. },
            r.center().y,
        ),
        7.,
        Color32::from_rgb(246, 244, 255),
    );
    response
}
pub fn source_art(ui: &mut egui::Ui, window: bool, colors: Palette) {
    let (r, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 143.), egui::Sense::hover());
    let p = ui.painter();
    let c = r.center() - Vec2::new(0., 7.);
    for i in (0..5).rev() {
        let pad = i as f32 * 9.;
        p.rect_filled(
            Rect::from_center_size(c, Vec2::new(195. + pad * 2., 99. + pad)),
            18.,
            Color32::from_rgba_unmultiplied(127, 93, 215, 3 + i * 2),
        );
    }
    for i in 0..=12 {
        let x = r.left() + i as f32 * r.width() / 12.;
        for j in 0..=4 {
            p.circle_filled(
                Pos2::new(x, r.top() + j as f32 * 32.),
                0.8,
                colors.muted.gamma_multiply(0.15),
            );
        }
    }
    let screen = Rect::from_center_size(c, Vec2::new(196., 109.));
    p.rect_filled(screen, 10., colors.background);
    p.rect_stroke(
        screen,
        10.,
        Stroke::new(1.5_f32, Color32::from_rgb(106, 92, 151)),
        egui::StrokeKind::Inside,
    );
    let inner = screen.shrink(6.);
    p.rect_filled(inner, 5., colors.tint);
    // Static source illustration, not a live preview or simulated capture.
    for i in 0..5 {
        let y = inner.bottom() - i as f32 * 14.;
        let points = vec![
            Pos2::new(inner.left(), y),
            Pos2::new(inner.left() + 56., y - 29.),
            Pos2::new(inner.left() + 105., y - 4.),
            Pos2::new(inner.right(), y - 47.),
        ];
        p.add(egui::Shape::line(
            points,
            Stroke::new(1.0_f32, PURPLE.gamma_multiply(0.12 + i as f32 * 0.07)),
        ));
    }
    if window {
        p.line_segment(
            [
                inner.left_top() + Vec2::new(0., 13.),
                inner.right_top() + Vec2::new(0., 13.),
            ],
            Stroke::new(1.0_f32, PURPLE.gamma_multiply(0.4)),
        );
        for i in 0..3 {
            p.circle_filled(
                inner.left_top() + Vec2::new(8. + i as f32 * 7., 7.),
                1.4,
                PURPLE,
            );
        }
    } else {
        p.line_segment(
            [
                screen.center_bottom(),
                screen.center_bottom() + Vec2::new(0., 10.),
            ],
            Stroke::new(2.0_f32, colors.muted),
        );
        p.line_segment(
            [
                screen.center_bottom() + Vec2::new(-21., 11.),
                screen.center_bottom() + Vec2::new(21., 11.),
            ],
            Stroke::new(2.0_f32, colors.muted),
        );
    }
}
pub fn titlebar(ctx: &egui::Context, colors: Palette) {
    egui::TopBottomPanel::top("custom_chrome")
        .exact_height(42.)
        .frame(
            egui::Frame::new()
                .fill(colors.sidebar)
                .stroke(Stroke::new(1_f32, colors.border)),
        )
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let drag = ui.interact(
                Rect::from_min_max(rect.min, rect.max - Vec2::new(154., 0.)),
                ui.id().with("window-drag"),
                egui::Sense::click_and_drag(),
            );
            if drag.drag_started() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            if drag.double_clicked() {
                let max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
            }
            ui.horizontal_centered(|ui| {
                ui.add_space(20.);
                ui.label(
                    RichText::new("POTATO RECORDER")
                        .size(10.)
                        .extra_letter_spacing(2.)
                        .color(colors.muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    for (symbol, cmd) in [
                        ("×", egui::ViewportCommand::Close),
                        (
                            "□",
                            egui::ViewportCommand::Maximized(
                                !ctx.input(|i| i.viewport().maximized.unwrap_or(false)),
                            ),
                        ),
                        ("—", egui::ViewportCommand::Minimized(true)),
                    ] {
                        let button =
                            egui::Button::new(RichText::new(symbol).size(19.).color(colors.muted))
                                .frame(false)
                                .min_size(Vec2::new(42., 32.));
                        if ui.add(button).clicked() {
                            ctx.send_viewport_cmd(cmd);
                        }
                    }
                });
            });
        });
}
