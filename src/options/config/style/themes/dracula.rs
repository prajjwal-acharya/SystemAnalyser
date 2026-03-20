use tui::{
    style::{Color, Modifier},
    widgets::BorderType,
};

use super::{color, hex};
use crate::options::config::style::{Styles, themes::hex_colour};

impl Styles {
    pub(crate) fn dracula_palette() -> Self {
        Self {
            ram_style: hex!("#50fa7b"), // Green
            #[cfg(not(target_os = "windows"))]
            cache_style: hex!("#ff79c6"), // Pink
            swap_style: hex!("#ffb86c"), // Orange
            #[cfg(feature = "zfs")]
            arc_style: hex!("#8be9fd"), // Cyan
            #[cfg(feature = "gpu")]
            gpu_colours: vec![
                hex!("#bd93f9"), // Purple
                hex!("#ff5555"), // Red
                hex!("#50fa7b"), // Green
                hex!("#f1fa8c"), // Yellow
                hex!("#ff79c6"), // Pink
                hex!("#ffb86c"), // Orange
            ],
            rx_style: hex!("#50fa7b"),
            tx_style: hex!("#f1fa8c"),
            total_rx_style: hex!("#8be9fd"),
            total_tx_style: hex!("#bd93f9"),
            all_cpu_colour: hex!("#50fa7b"),
            avg_cpu_colour: hex!("#ff5555"),
            cpu_colour_styles: vec![
                hex!("#bd93f9"),
                hex!("#ff5555"),
                hex!("#50fa7b"),
                hex!("#f1fa8c"),
                hex!("#ff79c6"),
                hex!("#ffb86c"),
                hex!("#8be9fd"),
            ],
            border_style: hex!("#6272a4"),             // Comment
            highlighted_border_style: hex!("#ff79c6"), // Pink
            text_style: hex!("#f8f8f2"),               // Foreground
            selected_text_style: hex!("#282a36").bg(hex_colour!("#ff79c6")), // Background on Pink
            table_header_style: hex!("#bd93f9").add_modifier(Modifier::BOLD), // Purple
            widget_title_style: hex!("#f8f8f2"),
            graph_style: hex!("#6272a4"),        // Comment
            graph_legend_style: hex!("#6272a4"), // Comment
            high_battery: hex!("#50fa7b"),
            medium_battery: hex!("#f1fa8c"),
            low_battery: hex!("#ff5555"),
            invalid_query_style: color!(Color::Red),
            disabled_text_style: hex!("#6272a4"), // Comment
            border_type: BorderType::Plain,
            #[cfg(target_os = "linux")]
            thread_text_style: hex!("#bd93f9"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dracula_palettes_valid() {
        let _ = super::Styles::dracula_palette();
    }
}
