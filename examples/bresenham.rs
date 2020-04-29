use maths::bresenham::*;
use nannou::prelude::*;

struct Model {
    _window: window::Id,
    line_points: Vec<Vec<Point>>,
}

fn model(app: &App) -> Model {
    Model {
        _window: app.new_window().size(800, 600).view(view).build().unwrap(),
        line_points: vec![
            // a line for each octant
            rasterize([0, 0].into(), [200, 160].into()),
            rasterize([0, 0].into(), [160, 200].into()),
            rasterize([0, 0].into(), [-200, 160].into()),
            rasterize([0, 0].into(), [-160, 200].into()),
            rasterize([0, 0].into(), [200, -160].into()),
            rasterize([0, 0].into(), [160, -200].into()),
            rasterize([0, 0].into(), [-200, -160].into()),
            rasterize([0, 0].into(), [-160, -200].into()),
            // Outmost shape (x1.5 bigger then the middle one)
            rasterize([-150, 50].into(), [0, 160].into()),
            rasterize([150, 50].into(), [0, 160].into()),
            rasterize([-150, 50].into(), [-110, -160].into()),
            rasterize([150, 50].into(), [110, -160].into()),
            rasterize([110, -160].into(), [-110, -160].into()),
            // Middle shape
            rasterize([-100, 33].into(), [0, 106].into()),
            rasterize([100, 33].into(), [0, 106].into()),
            rasterize([-100, 33].into(), [-73, -106].into()),
            rasterize([100, 33].into(), [73, -106].into()),
            rasterize([73, -106].into(), [-73, -106].into()),
            // Innermost shape (x0.8 smaller then the middle one)
            rasterize([-80, 26].into(), [0, 85].into()),
            rasterize([80, 26].into(), [0, 85].into()),
            rasterize([-80, 26].into(), [-58, -85].into()),
            rasterize([80, 26].into(), [58, -85].into()),
            rasterize([58, -85].into(), [-58, -85].into()),
            // Outmost & Innermost connections
            rasterize([0, 85].into(), [0, 160].into()),
            rasterize([80, 26].into(), [150, 50].into()),
            rasterize([-80, 26].into(), [-150, 50].into()),
            rasterize([58, -85].into(), [110, -160].into()),
            rasterize([-58, -85].into(), [-110, -160].into()),
        ],
    }
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}

fn draw_interactive_grid(app: &App, draw: &Draw) {
    let window = app.main_window();
    let win = window.rect();
    // 100-step and 10-step grids.
    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);
    // Ellipse at mouse.
    draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

    // Mouse position text.
    let mouse = app.mouse.position();
    let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
    draw.text(&pos)
        .xy(mouse + vec2(0.0, 20.0))
        .font_size(14)
        .color(WHITE);

    // Crosshair.
    let crosshair_color = gray(0.5);
    let ends = [
        win.mid_top(),
        win.mid_right(),
        win.mid_bottom(),
        win.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }

    // Crosshair text.
    let top = format!("{:.1}", win.top());
    let bottom = format!("{:.1}", win.bottom());
    let left = format!("{:.1}", win.left());
    let right = format!("{:.1}", win.right());
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);
    draw.text(&top)
        .h(win.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);

    // Ellipse at mouse.
    draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.11, 0.12, 0.13);
    draw_interactive_grid(&app, &draw);

    // Mouse position text.
    let mouse = app.mouse.position();
    let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
    draw.text(&pos)
        .xy(mouse + vec2(0.0, 20.0))
        .font_size(14)
        .color(WHITE);

    for points in &m.line_points {
        draw.line()
            .start(pt2(points[0].x as f32, points[0].y as f32))
            .end(pt2(
                points[points.len() - 1].x as f32,
                points[points.len() - 1].y as f32,
            ))
            .weight(2.0)
            .color(RED);

        for pixel in points {
            draw.ellipse()
                .x_y(pixel.x as f32, pixel.y as f32)
                .w_h(2.0, 2.0)
                .color(WHITE);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).run()
}
