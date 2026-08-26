use bezier_curve_viewer::App;

fn main() -> Result<(), iced::Error> {
    iced::application(App::new, App::update, App::view).run()
}
