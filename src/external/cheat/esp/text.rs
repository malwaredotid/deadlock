use egui::{Align2, Pos2, Rect};

use crate::{
    external::interfaces::entities::Player, settings::structs::{Settings, TextSettings},
};

pub fn draw(g: &egui::Painter, player: &Player, settings: &Settings) {
    let mut offsets = (0., 0., 0., 0.); // left, top, right, bottom
    draw_text(
        g,
        player.rect,
        &settings.esp_players.text_hero,
        format!("{:?}", player.data.hero),
        &mut offsets,
    );
    draw_text(
        g,
        player.rect,
        &settings.esp_players.text_health,
        format!("{}/{}", player.pawn.health, player.pawn.max_health),
        &mut offsets,
    );
}

fn draw_text(
    g: &egui::Painter,
    rect: Rect,
    settings: &TextSettings,
    text: String,
    offsets: &mut (f32, f32, f32, f32),
) {
    let mut pos = Pos2 { x: 0., y: 0. };
    let mut align = Align2::LEFT_TOP;
    if settings.align == Align2::LEFT_TOP {
        align = Align2::RIGHT_TOP;
        offsets.0 += 12.;
        pos = Pos2 {
            x: rect.left(),
            y: rect.top() + offsets.0,
        };
    } else if settings.align == Align2::CENTER_TOP {
        align = Align2::CENTER_BOTTOM;
        offsets.1 -= 12.;
        pos = Pos2 {
            x: rect.left() + rect.width() / 2.,
            y: rect.top() + offsets.1,
        };
    } else if settings.align == Align2::RIGHT_TOP {
        align = Align2::LEFT_TOP;
        offsets.2 += 12.;
        pos = Pos2 {
            x: rect.left() + rect.width(),
            y: rect.top() + offsets.2,
        };
    } else if settings.align == Align2::CENTER_BOTTOM {
        align = Align2::CENTER_TOP;
        offsets.3 += 12.;
        pos = Pos2 {
            x: rect.left() + rect.width() / 2.,
            y: rect.bottom() + offsets.3,
        };
    }
    let mut font = egui::FontId::default();
    font.size = settings.font_size;
    if settings.shadow {
        g.text(
            Pos2 {
                x: pos.x + 2.,
                y: pos.y + 2.,
            },
            align,
            text.to_owned(),
            font.clone(),
            egui::Color32::BLACK,
        );
    }
    let rect = g.text(pos, align, text, font, settings.font_color);
    if settings.align == Align2::LEFT_TOP {
        offsets.0 += rect.height();
    } else if settings.align == Align2::CENTER_TOP {
        offsets.1 -= rect.height();
    } else if settings.align == Align2::RIGHT_TOP {
        offsets.2 += rect.height();
    } else if settings.align == Align2::CENTER_BOTTOM {
        offsets.3 += rect.height();
    }
}
