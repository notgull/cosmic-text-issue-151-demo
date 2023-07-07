// MIT/Apache2 License.
//! Reproduction of cosmic text issue #151
//! 
//! This version uses cosmic_text v0.8.0, which works and does not justify text.

use cosmic_text::{self as ct, Buffer, BufferLine, Color, FontSystem};

static TEXT: &str = r#"Philosophers often behave like little children who scribble some marks on a piece of paper at random and then ask the grown-up "What's that?" — It happened like this: the grown-up had drawn pictures for the child several times and said "this is a man," "this is a house," etc. And then the child makes some marks too and asks: what's this then?"#;

const RED: Color = Color::rgb(255, 0, 0);
const BLUE: Color = Color::rgb(0, 0, 255);

fn main() {
    tracing_subscriber::fmt::init();

    let mut fs = FontSystem::new();
    let size = 18.0;
    let mut buffer = Buffer::new(&mut fs, ct::Metrics::new(size, size));

    // Construct the attributes list.
    let default_attrs = ct::Attrs {
        color_opt: Some(RED),
        family: ct::Family::Name("Courier New"),
        style: ct::Style::Italic,
        weight: ct::Weight::BOLD,
        ..ct::Attrs::new()
    };

    // Set up attribute ranges.
    let mut list = ct::AttrsList::new(default_attrs);
    list.add_span(
        0..10,
        ct::Attrs {
            color_opt: Some(BLUE),
            ..default_attrs
        },
    );
    list.add_span(
        10..60,
        ct::Attrs {
            color_opt: Some(BLUE),
            weight: ct::Weight::NORMAL,
            ..default_attrs
        },
    );
    list.add_span(
        60..100,
        ct::Attrs {
            color_opt: Some(BLUE),
            weight: ct::Weight::NORMAL,
            style: ct::Style::Normal,
            ..default_attrs
        },
    );
    list.add_span(
        100..140,
        ct::Attrs {
            color_opt: Some(BLUE),
            style: ct::Style::Normal,
            ..default_attrs
        },
    );
    list.add_span(
        140..160,
        ct::Attrs {
            color_opt: Some(BLUE),
            style: ct::Style::Normal,
            weight: ct::Weight::NORMAL,
            ..default_attrs
        },
    );
    list.add_span(
        160..200,
        ct::Attrs {
            color_opt: Some(BLUE),
            weight: ct::Weight::NORMAL,
            ..default_attrs
        },
    );
    list.add_span(
        200..220,
        ct::Attrs {
            color_opt: Some(RED),
            weight: ct::Weight::NORMAL,
            ..default_attrs
        },
    );
    list.add_span(
        220..240,
        ct::Attrs {
            color_opt: Some(RED),
            ..default_attrs
        },
    );
    list.add_span(
        240..346,
        ct::Attrs {
            color_opt: Some(RED),
            family: ct::Family::Monospace,
            ..default_attrs
        },
    );

    // Create and format the text.
    let line = BufferLine::new(TEXT, list);
    buffer.lines = vec![line];
    buffer.set_wrap(&mut fs, ct::Wrap::Word);
    buffer.set_size(&mut fs, 250.0, 300.0);
    buffer.shape_until_scroll(&mut fs);

    // Create an image to draw to.
    let mut image = tiny_skia::Pixmap::new(250, 300).unwrap();
    image.fill(tiny_skia::Color::WHITE);

    // Draw the text.
    let mut cache = ct::SwashCache::new();
    buffer.draw(
        &mut fs,
        &mut cache,
        ct::Color::rgb(0xFF, 0xFF, 0xFF),
        |x, y, w, h, clr| {
            image.fill_rect(
                tiny_skia::Rect::from_xywh(x as f32, y as f32, w as f32, h as f32).unwrap(),
                &tiny_skia::Paint {
                    shader: tiny_skia::Shader::SolidColor({
                        let [r, g, b, a] = [clr.r(), clr.g(), clr.b(), clr.a()];
                        tiny_skia::Color::from_rgba8(r, g, b, a)
                    }),
                    ..Default::default()
                },
                tiny_skia::Transform::identity(),
                None,
            );
        },
    );

    // Save the image to a file.
    image.save_png(std::env::args_os().nth(1).unwrap()).unwrap();
}
