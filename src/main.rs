slint::include_modules!();



fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let mess: String = string.trim().parse().unwrap();
 
        let result = format!("Messenge", mess);
        ui.set_results(result.into());
    });

    ui.run()
}