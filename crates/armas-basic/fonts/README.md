# Inter Fonts

This directory contains the Inter font family for use in Armas examples.

## License

Inter font family by Rasmus Andersson
Licensed under the SIL Open Font License 1.1
https://github.com/rsms/inter

## Included Weights

- **Inter-Regular.otf** - 400 (Regular)
- **Inter-Medium.otf** - 500 (Medium)
- **Inter-SemiBold.otf** - 600 (SemiBold)
- **Inter-Bold.otf** - 700 (Bold)

## Usage

See the `aceternity_buttons` example for how to load these fonts:

```rust
use armas::fonts;

fn setup_fonts(ctx: &egui::Context) {
    let inter_regular = include_bytes!("../fonts/Inter-Regular.otf");
    let inter_medium = include_bytes!("../fonts/Inter-Medium.otf");
    let inter_semibold = include_bytes!("../fonts/Inter-SemiBold.otf");
    let inter_bold = include_bytes!("../fonts/Inter-Bold.otf");

    fonts::load_font_family(
        ctx,
        "Inter",
        inter_regular,
        Some(inter_medium),
        Some(inter_semibold),
        Some(inter_bold),
    );

    // Set as default font (optional)
    fonts::set_default_font(ctx, "Inter");
}
```

## Download

If you need to download Inter yourself:
- **Official website**: https://rsms.me/inter/
- **GitHub**: https://github.com/rsms/inter/releases
