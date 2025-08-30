slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    // Handle menu button clicks
    app.on_file_clicked(|| {
        println!("File menu clicked!");
    });

    app.on_edit_clicked(|| {
        println!("Edit menu clicked!");
    });

    app.on_format_clicked(|| {
        println!("Format menu clicked!");
    });

    app.run()
}