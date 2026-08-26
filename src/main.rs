use bezier_curve_viewer::App;

fn main() -> Result<(), iced::Error> {
    iced::application(App::new, App::update, App::view)
        .title("Bézier Curve Viewer 0.1.0")
        .run()
}
