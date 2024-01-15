slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |name, message| {
        let ui = ui_handle.unwrap();
        let formatted_message = format!("{}: {}", name.trim(), message.trim());
        ui.set_results(formatted_message.into());
    });

    ui.run()
}
