use tui::{
    style::{Color, Modifier},
    widgets::BorderType,
};

use super::{color, hex};
use crate::options::config::style::{Styles, themes::hex_colour};

impl Styles {
    pub(crate) fn catppuccin_palette() -> Self {
        Self {
            ram_style: hex!("#a6e3a1"), // Green
            #[cfg(not(target_os = "windows"))]
            cache_style: hex!("#cba6f7"), // Mauve
            swap_style: hex!("#fab387"), // Peach
            #[cfg(feature = "zfs")]
            arc_style: hex!("#94e2d5"), // Teal
            #[cfg(feature = "gpu")]
            gpu_colours: vec![
                hex!("#89b4fa"), // Blue
                hex!("#f38ba8"), // Red
                hex!("#a6e3a1"), // Green
                hex!("#f9e2af"), // Yellow
                hex!("#cba6f7"), // Mauve
                hex!("#fab387"), // Peach
            ],
            rx_style: hex!("#a6e3a1"),
            tx_style: hex!("#f9e2af"),
            total_rx_style: hex!("#94e2d5"),
            total_tx_style: hex!("#89b4fa"),
            all_cpu_colour: hex!("#a6e3a1"),
            avg_cpu_colour: hex!("#f38ba8"),
            cpu_colour_styles: vec![
                hex!("#89b4fa"),
                hex!("#f38ba8"),
                hex!("#a6e3a1"),
                hex!("#f9e2af"),
                hex!("#cba6f7"),
                hex!("#fab387"),
                hex!("#94e2d5"),
                hex!("#89dceb"), // Sky
            ],
            border_style: hex!("#585b70"), // Surface 2
            highlighted_border_style: hex!("#cba6f7"), // Mauve
            text_style: hex!("#cdd6f4"), // Text
            selected_text_style: hex!("#1e1e2e").bg(hex_colour!("#cba6f7")), // Base on Mauve
            table_header_style: hex!("#89b4fa").add_modifier(Modifier::BOLD),
            widget_title_style: hex!("#cdd6f4"),
            graph_style: hex!("#585b70"), // Surface 2
            graph_legend_style: hex!("#a6adc8"), // Subtext 0
            high_battery: hex!("#a6e3a1"),
            medium_battery: hex!("#f9e2af"),
            low_battery: hex!("#f38ba8"),
            invalid_query_style: color!(Color::Red),
            disabled_text_style: hex!("#6c7086"), // Overlay 0
            border_type: BorderType::Plain,
            #[cfg(target_os = "linux")]
            thread_text_style: hex!("#89b4fa"),
        }
    }

    pub(crate) fn catppuccin_light_palette() -> Self {
        Self {
            ram_style: hex!("#40a02b"), // Green
            #[cfg(not(target_os = "windows"))]
            cache_style: hex!("#8839ef"), // Mauve
            swap_style: hex!("#fe640b"), // Peach
            #[cfg(feature = "zfs")]
            arc_style: hex!("#179299"), // Teal
            #[cfg(feature = "gpu")]
            gpu_colours: vec![
                hex!("#1e66f5"), // Blue
                hex!("#d20f39"), // Red
                hex!("#40a02b"), // Green
                hex!("#df8e1d"), // Yellow
                hex!("#8839ef"), // Mauve
                hex!("#fe640b"), // Peach
            ],
            rx_style: hex!("#40a02b"),
            tx_style: hex!("#df8e1d"),
            total_rx_style: hex!("#179299"),
            total_tx_style: hex!("#1e66f5"),
            all_cpu_colour: hex!("#40a02b"),
            avg_cpu_colour: hex!("#d20f39"),
            cpu_colour_styles: vec![
                hex!("#1e66f5"),
                hex!("#d20f39"),
                hex!("#40a02b"),
                hex!("#df8e1d"),
                hex!("#8839ef"),
                hex!("#fe640b"),
                hex!("#179299"),
                hex!("#04a5e5"), // Sky
            ],
            border_style: hex!("#9ca0b0"), // Surface 2
            highlighted_border_style: hex!("#8839ef"), // Mauve
            text_style: hex!("#4c4f69"), // Text
            selected_text_style: hex!("#eff1f5").bg(hex_colour!("#8839ef")), // Base on Mauve
            table_header_style: hex!("#1e66f5").add_modifier(Modifier::BOLD),
            widget_title_style: hex!("#4c4f69"),
            graph_style: hex!("#9ca0b0"),
            graph_legend_style: hex!("#6c6f85"), // Subtext 0
            high_battery: hex!("#40a02b"),
            medium_battery: hex!("#df8e1d"),
            low_battery: hex!("#d20f39"),
            invalid_query_style: color!(Color::Red),
            disabled_text_style: hex!("#8c8fa1"), // Overlay 0
            border_type: BorderType::Plain,
            #[cfg(target_os = "linux")]
            thread_text_style: hex!("#1e66f5"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn catppuccin_palettes_valid() {
        let _ = super::Styles::catppuccin_palette();
        let _ = super::Styles::catppuccin_light_palette();
    }
}