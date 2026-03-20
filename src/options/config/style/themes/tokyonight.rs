use tui::{
    style::{Color, Modifier},
    widgets::BorderType,
};

use super::{color, hex};
use crate::options::config::style::{Styles, themes::hex_colour};

impl Styles {
    pub(crate) fn tokyonight_palette() -> Self {
        Self {
            ram_style: hex!("#9ece6a"), // Green
            #[cfg(not(target_os = "windows"))]
            cache_style: hex!("#bb9af7"), // Magenta
            swap_style: hex!("#ff9e64"), // Orange
            #[cfg(feature = "zfs")]
            arc_style: hex!("#7dcfff"), // Cyan
            #[cfg(feature = "gpu")]
            gpu_colours: vec![
                hex!("#7aa2f7"), // Blue
                hex!("#f7768e"), // Red
                hex!("#9ece6a"), // Green
                hex!("#e0af68"), // Yellow
                hex!("#bb9af7"), // Magenta
                hex!("#ff9e64"), // Orange
            ],
            rx_style: hex!("#9ece6a"),
            tx_style: hex!("#e0af68"),
            total_rx_style: hex!("#7dcfff"),
            total_tx_style: hex!("#7aa2f7"),
            all_cpu_colour: hex!("#9ece6a"),
            avg_cpu_colour: hex!("#f7768e"),
            cpu_colour_styles: vec![
                hex!("#7aa2f7"),
                hex!("#f7768e"),
                hex!("#9ece6a"),
                hex!("#e0af68"),
                hex!("#bb9af7"),
                hex!("#ff9e64"),
                hex!("#7dcfff"),
            ],
            border_style: hex!("#292e42"), // Surface
            highlighted_border_style: hex!("#bb9af7"), // Magenta
            text_style: hex!("#c0caf5"), // Text
            selected_text_style: hex!("#1a1b26").bg(hex_colour!("#bb9af7")), // Base on Magenta
            table_header_style: hex!("#7aa2f7").add_modifier(Modifier::BOLD),
            widget_title_style: hex!("#c0caf5"),
            graph_style: hex!("#292e42"), // Surface
            graph_legend_style: hex!("#565f89"), // Comment
            high_battery: hex!("#9ece6a"),
            medium_battery: hex!("#e0af68"),
            low_battery: hex!("#f7768e"),
            invalid_query_style: color!(Color::Red),
            disabled_text_style: hex!("#565f89"), // Comment
            border_type: BorderType::Plain,
            #[cfg(target_os = "linux")]
            thread_text_style: hex!("#7aa2f7"),
        }
    }

    pub(crate) fn tokyonight_light_palette() -> Self {
        Self {
            ram_style: hex!("#485e30"), // Green
            #[cfg(not(target_os = "windows"))]
            cache_style: hex!("#9854f1"), // Magenta
            swap_style: hex!("#ff966c"), // Orange
            #[cfg(feature = "zfs")]
            arc_style: hex!("#166775"), // Cyan
            #[cfg(feature = "gpu")]
            gpu_colours: vec![
                hex!("#34548a"), // Blue
                hex!("#f52a65"), // Red
                hex!("#485e30"), // Green
                hex!("#8c6c3e"), // Yellow
                hex!("#9854f1"), // Magenta
                hex!("#ff966c"), // Orange
            ],
            rx_style: hex!("#485e30"),
            tx_style: hex!("#8c6c3e"),
            total_rx_style: hex!("#166775"),
            total_tx_style: hex!("#34548a"),
            all_cpu_colour: hex!("#485e30"),
            avg_cpu_colour: hex!("#f52a65"),
            cpu_colour_styles: vec![
                hex!("#34548a"),
                hex!("#f52a65"),
                hex!("#485e30"),
                hex!("#8c6c3e"),
                hex!("#9854f1"),
                hex!("#ff966c"),
                hex!("#166775"),
            ],
            border_style: hex!("#b4b5b9"), // Surface
            highlighted_border_style: hex!("#9854f1"), // Magenta
            text_style: hex!("#3760bf"), // Text
            selected_text_style: hex!("#d5d6db").bg(hex_colour!("#9854f1")), // Base on Magenta
            table_header_style: hex!("#34548a").add_modifier(Modifier::BOLD),
            widget_title_style: hex!("#3760bf"),
            graph_style: hex!("#b4b5b9"),
            graph_legend_style: hex!("#848cb5"), // Comment
            high_battery: hex!("#485e30"),
            medium_battery: hex!("#8c6c3e"),
            low_battery: hex!("#f52a65"),
            invalid_query_style: color!(Color::Red),
            disabled_text_style: hex!("#848cb5"), // Comment
            border_type: BorderType::Plain,
            #[cfg(target_os = "linux")]
            thread_text_style: hex!("#34548a"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tokyonight_palettes_valid() {
        let _ = super::Styles::tokyonight_palette();
        let _ = super::Styles::tokyonight_light_palette();
    }
}