use super::constants::*;
use super::points::calc::*;
use crate::STATE;
use skulpin::skia_safe::{Canvas, Color, Paint, PaintStyle, Path};

pub enum GuidelineType {
    Horizontal,
    Vertical,
}

pub struct Guideline {
    pub gtype: GuidelineType,
    pub where_: f32,
    pub selected: bool,
    pub name: Option<String>,
}

pub fn draw_guideline(color: Color, where_: f32, gtype: GuidelineType, canvas: &mut Canvas) {
    let mut paint = Paint::default();
    let mut path = Path::new();
    let factor = STATE.with(|v| v.borrow().factor);
    let offset = STATE.with(|v| v.borrow().offset);
    match gtype {
        GuidelineType::Vertical => {
            STATE.with(|v| {
                path.move_to((where_, -(offset.1 * (1. / factor))));
                path.line_to((
                    where_,
                    v.borrow().winsize.height as f32 * (1. / factor) + -(offset.1 * (1. / factor)),
                ));
            });
        }
        GuidelineType::Horizontal => {
            STATE.with(|v| {
                path.move_to((-(offset.0 * (1. / factor)), where_));
                path.line_to((
                    (v.borrow().winsize.width as f32 * (1. / factor))
                        + (-(offset.0 * (1. / factor))),
                    where_,
                ));
            });
        }
    }
    path.close();
    paint.set_anti_alias(true);
    paint.set_color(color);
    paint.set_stroke_width(GUIDELINE_THICKNESS * (1. / factor));
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&path, &paint);
}

pub fn draw_lbearing(canvas: &mut Canvas) {
    draw_guideline(
        Color::from(LBEARING_STROKE),
        0.,
        GuidelineType::Vertical,
        canvas,
    );
}

pub fn draw_rbearing(width: u64, canvas: &mut Canvas) {
    draw_guideline(
        Color::from(RBEARING_STROKE),
        width as f32,
        GuidelineType::Vertical,
        canvas,
    );
}

pub fn draw_baseline(canvas: &mut Canvas) {
    draw_guideline(
        Color::from(LBEARING_STROKE),
        calc_y(0.),
        GuidelineType::Horizontal,
        canvas,
    );
}

pub fn draw_all(canvas: &mut Canvas) {
    STATE.with(|v| {
        draw_lbearing(canvas);
        draw_rbearing(v.borrow().glyph.as_ref().unwrap().glif.width, canvas);
        draw_baseline(canvas);

        for guideline in &v.borrow().glyph.as_ref().unwrap().guidelines {
            draw_guideline(
                Color::from(LBEARING_STROKE),
                calc_y(guideline.where_),
                GuidelineType::Horizontal,
                canvas,
            );
        }
    });
}
