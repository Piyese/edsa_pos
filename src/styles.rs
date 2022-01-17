use eframe::egui::{Frame, Vec2, epaint::Shadow, Color32, Stroke, Visuals, style::{WidgetVisuals, Selection, Widgets, Spacing, Interaction, DebugOptions}, Style, TextStyle};

pub fn get_frame() -> Frame {
    let vc = Vec2::new(5., 5.);
    let shdw = Shadow::small_light();
    let strk = Stroke::new(1., Color32::BLACK);

    let frme = Frame {
        margin: vc,
        corner_radius: 1.0,
        shadow: shdw,
        fill: Color32::from_rgb (235, 235, 235),
        stroke: strk
    };
    frme
}

pub fn get_style()->Style{
    let spacing = Spacing::default();
    let interaction = Interaction::default();
    let visuals = Visuals::light();
    let debug = DebugOptions::default();

    Style{ 
        body_text_style: TextStyle::Small , 
        override_text_style: None, 
        wrap: None, 
        spacing, 
        interaction, 
        visuals, 
        animation_time: 3., 
        debug, 
        explanation_tooltips: true, 
    }
}

pub fn light() -> Visuals {
    Visuals {
        dark_mode: false,
        override_text_color: Some(Color32::BLACK),
        widgets: widget_light(),
        selection: Selection::default(),   //
        hyperlink_color: Color32::from_rgb(90, 170, 255),
        faint_bg_color: Color32::GRAY,
        extreme_bg_color: Color32::GRAY,
        code_bg_color: Color32::GRAY,
        window_corner_radius: 6.0,
        window_shadow: Shadow::big_light(),
        popup_shadow: Shadow::small_light(),
        resize_corner_size: 12.0,
        text_cursor_width: 2.0,
        text_cursor_preview: true,
        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
    }
}

pub fn widget_light() -> Widgets {
    Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: Color32::WHITE, // window background
            bg_stroke: Stroke::new(1.0, Color32::BLACK), // separators, indentation lines, windows outlines
            fg_stroke: Stroke::new(1.0, Color32::from_gray(100)), // normal text color
            corner_radius: 2.0,
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            bg_fill: Color32::LIGHT_GREEN, // button background
            bg_stroke: Default::default(),
            fg_stroke: Stroke::new(1.0, Color32::BLACK), // button text
            corner_radius: 2.0,
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            bg_fill: Color32::LIGHT_GREEN,
            bg_stroke: Stroke::new(2.0, Color32::from_gray(105)), // e.g. hover over window edge or button
            fg_stroke: Stroke::new(2.0, Color32::BLACK),
            corner_radius: 3.0,
            expansion: 2.,
        },
        active: WidgetVisuals {
            bg_fill: Color32::LIGHT_BLUE,
            bg_stroke: Stroke::new(4.0, Color32::BLACK),
            fg_stroke: Stroke::new(4.0, Color32::BLACK),
            corner_radius: 2.0,
            expansion: 1.5,
        },
        open: WidgetVisuals {
            bg_fill: Color32::from_gray(220),
            bg_stroke: Stroke::new(1.0, Color32::from_gray(160)),
            fg_stroke: Stroke::new(1.0, Color32::BLACK),
            corner_radius: 2.0,
            expansion: 0.0,
        },
    }
}
