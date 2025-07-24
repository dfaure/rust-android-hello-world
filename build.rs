fn main() {
    let config = slint_build::CompilerConfiguration::new()
    .with_style("fluent-light".into());
    //.with_style("material-light".into());
    //.with_style("material-dark".into());
    //.with_style("cupertino-dark".into());
    //.with_style("cosmic-light".into());

    // material-light: too big
    // fluent-light: clean, very square, blue highlight below the lineedit, blue selection
    // cupertino-light: even smaller, clean too, blue highlight around the lineedit
    // cosmic-light: too gray

    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
