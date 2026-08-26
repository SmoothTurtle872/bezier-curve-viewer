use iced::widget::canvas::{Stroke, Style, stroke};
use iced::widget::{canvas, column, container, row, scrollable};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};
use iced::{mouse, theme};
use iced_aw::NumberInput;

#[derive(Debug, Clone)]
pub struct App {
    curve: Curve,
}

impl App {
    pub fn new() -> Self {
        Self {
            curve: Curve::default(),
        }
    }
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            container(
                canvas(self.curve.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .style(|theme| { container::bordered_box(theme) }),
            container(
                scrollable(
                    row![
                        "Detail: ",
                        NumberInput::new(&self.curve.detail, 1..254, Message::DetailChanged)
                            .width(50),
                        "Start Point: ",
                        container(
                            column![
                                NumberInput::new(&self.curve.start.x, 0.0..1000.0, |value| {
                                    Message::SetStartPoint(value, self.curve.start.y)
                                })
                                .width(50),
                                NumberInput::new(&self.curve.start.y, 0.0..1000.0, |value| {
                                    Message::SetStartPoint(self.curve.start.x, value)
                                })
                                .width(50)
                            ]
                            .spacing(10)
                            .padding(10)
                        )
                        .style(|theme| { container::bordered_box(theme) }),
                        "End Point: ",
                        container(
                            column![
                                NumberInput::new(&self.curve.end.x, 0.0..1000.0, |value| {
                                    Message::SetEndPoint(value, self.curve.end.y)
                                })
                                .width(50),
                                NumberInput::new(&self.curve.end.y, 0.0..1000.0, |value| {
                                    Message::SetEndPoint(self.curve.end.x, value)
                                })
                                .width(50)
                            ]
                            .spacing(10)
                            .padding(10)
                        )
                        .style(|theme| { container::bordered_box(theme) }),
                        "Control Point 1: ",
                        container(
                            column![
                                NumberInput::new(&self.curve.control_one.x, 0.0..1000.0, |value| {
                                    Message::SetControlPointOne(value, self.curve.control_one.y)
                                })
                                .width(50),
                                NumberInput::new(&self.curve.control_one.y, 0.0..1000.0, |value| {
                                    Message::SetControlPointOne(self.curve.control_one.x, value)
                                })
                                .width(50)
                            ]
                            .spacing(10)
                            .padding(10)
                        )
                        .style(|theme| { container::bordered_box(theme) }),
                        "Control Point 2: ",
                        container(
                            column![
                                NumberInput::new(&self.curve.control_two.x, 0.0..1000.0, |value| {
                                    Message::SetControlPointTwo(value, self.curve.control_two.y)
                                })
                                .width(50),
                                NumberInput::new(&self.curve.control_two.y, 0.0..1000.0, |value| {
                                    Message::SetControlPointTwo(self.curve.control_two.x, value)
                                })
                                .width(50)
                            ]
                            .spacing(10)
                            .padding(10)
                        )
                        .style(|theme| { container::bordered_box(theme) })
                    ]
                    .spacing(10)
                    .padding(10)
                )
                .horizontal()
            )
            .style(|theme| { container::bordered_box(theme) })
        ]
        .spacing(10)
        .padding(20)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DetailChanged(detail) => self.curve.detail = detail,
            Message::SetStartPoint(x, y) => self.curve.start = Point::new(x, y),
            Message::SetEndPoint(x, y) => self.curve.end = Point::new(x, y),
            Message::SetControlPointOne(x, y) => self.curve.control_one = Point::new(x, y),
            Message::SetControlPointTwo(x, y) => self.curve.control_two = Point::new(x, y),
        }
    }
}

#[derive(Debug, Clone)]
struct Curve {
    start: Point,
    end: Point,
    control_one: Point,
    control_two: Point,
    detail: u8,
}

impl Default for Curve {
    fn default() -> Self {
        Self {
            start: Point { x: 50.0, y: 50.0 },
            end: Point { x: 150.0, y: 150.0 },
            control_one: Point { x: 50.0, y: 150.0 },
            control_two: Point { x: 150.0, y: 50.0 },
            detail: 100,
        }
    }
}

impl Curve {
    fn get_location(&self, t: f32) -> Point {
        // For a bezier curve, t is always between 0 and one
        let t = if t > 1.0 {
            1.0
        } else if t < 0.0 {
            0.0
        } else {
            t
        };

        // This could be done without the additional variables,
        // and probably will be compiled down wothout them
        // However, it is so much easier to write if I have
        // the same names as the variables in my maths notebook
        let (x0, x1, x2, x3) = (
            self.start.x,
            self.control_one.x,
            self.control_two.x,
            self.end.x,
        );

        let (ax, bx, cx, dx) = (
            x3 - (3.0 * x2) + (3.0 * x1) - x0,
            (3.0 * x2) - (6.0 * x1) + (3.0 * x0),
            (3.0 * x1) - (3.0 * x0),
            x0,
        );

        let (y0, y1, y2, y3) = (
            self.start.y,
            self.control_one.y,
            self.control_two.y,
            self.end.y,
        );

        let (ay, by, cy, dy) = (
            y3 - (3.0 * y2) + (3.0 * y1) - y0,
            (3.0 * y2) - (6.0 * y1) + (3.0 * y0),
            (3.0 * y1) - (3.0 * y0),
            y0,
        );
        Point {
            x: ax * t.powi(3) + bx * t.powi(2) + cx * t + dx,
            y: ay * t.powi(3) + by * t.powi(2) + cy * t + dy,
        }
    }
}

impl<Message> iced::widget::canvas::Program<Message> for Curve {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &iced_renderer::core::Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry<Renderer>> {
        let palette = theme.palette();
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let start = canvas::Path::circle(self.start, 4.0);
        let end = canvas::Path::circle(self.end, 4.0);
        let control_one = canvas::Path::circle(self.control_one, 4.0);
        let control_two = canvas::Path::circle(self.control_two, 4.0);

        frame.fill(&start, palette.primary);
        frame.fill(&end, palette.primary);
        frame.fill(&control_one, palette.success);
        frame.fill(&control_two, palette.success);

        for segment in 0..self.detail {
            let start = self.get_location(segment as f32 / self.detail as f32);
            let end = self.get_location((segment + 1) as f32 / self.detail as f32);
            let line = canvas::Path::line(start, end);
            let stroke = Stroke {
                style: Style::Solid(palette.primary),
                width: 2.0,
                line_cap: stroke::LineCap::Round,
                ..Stroke::default()
            };
            frame.stroke(&line, stroke);
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DetailChanged(u8),
    SetStartPoint(f32, f32),
    SetEndPoint(f32, f32),
    SetControlPointOne(f32, f32),
    SetControlPointTwo(f32, f32),
}
