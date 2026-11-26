//! GPUI Theme Definitions for CosmicOS
//!
//! Implementación de los estándares visuales de CosmicOS:
//! - Liquid Glass: Efectos de difuminado, transparencia y elevación.
//! - Material You 3: Paleta de colores dinámica y formas orgánicas.

pub enum ThemeType {
    LiquidGlass,
    MaterialYou3,
}

pub struct ThemeConfig {
    pub name: String,
    pub theme_type: ThemeType,
    pub primary_color: String,
    pub secondary_color: String,
    pub blur_radius: f32,
    pub corner_radius: f32,
}

pub fn get_default_themes() -> Vec<ThemeConfig> {
    vec![
        ThemeConfig {
            name: "Liquid Glass".to_string(),
            theme_type: ThemeType::LiquidGlass,
            primary_color: "#FFFFFF".to_string(),
            secondary_color: "#000000".to_string(),
            blur_radius: 20.0,
            corner_radius: 12.0,
        },
        ThemeConfig {
            name: "Material You 3".to_string(),
            theme_type: ThemeType::MaterialYou3,
            primary_color: "#D0BCFF".to_string(),
            secondary_color: "#49454F".to_string(),
            blur_radius: 0.0,
            corner_radius: 28.0,
        },
    ]
}
