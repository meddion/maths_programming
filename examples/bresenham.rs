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
            // 4 lines for each octant
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

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

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
                .color(BLACK);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).run()
}
