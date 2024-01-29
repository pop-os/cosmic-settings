/// Normalize the labeling of displays across settings pages.
pub fn display_name(name: &str, physical: (u32, u32)) -> String {
    let inches = ((physical.0.pow(2) + physical.1.pow(2)) as f32).sqrt() * 0.039_370_1;
    let inches_string = format!("{inches:.1}\"");

    match name {
        "eDP-1" | "LVDS1" => {
            fl!("display", "laptop", size = inches_string.as_str())
        }
        output => fl!(
            "display",
            "external",
            size = inches_string.as_str(),
            output = output
        ),
    }
}
