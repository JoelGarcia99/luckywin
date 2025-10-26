pub const INTER_400_TTF: &[u8] = include_bytes!("assets/inter/Inter-400.ttf");
pub const INTER_500_TTF: &[u8] = include_bytes!("assets/inter/Inter-500.ttf");
pub const INTER_600_TTF: &[u8] = include_bytes!("assets/inter/Inter-600.ttf");
pub const INTER_700_TTF: &[u8] = include_bytes!("assets/inter/Inter-700.ttf");
pub const INTER_800_TTF: &[u8] = include_bytes!("assets/inter/Inter-800.ttf");

pub const INTER_FONT_ASSETS: [(&str, &[u8]); 5] = [
    ("assets/inter/Inter-400.ttf", INTER_400_TTF),
    ("assets/inter/Inter-500.ttf", INTER_500_TTF),
    ("assets/inter/Inter-600.ttf", INTER_600_TTF),
    ("assets/inter/Inter-700.ttf", INTER_700_TTF),
    ("assets/inter/Inter-800.ttf", INTER_800_TTF),
];
