//! # Display Output
//!
//! Tipos de conectores e outputs de display.

/// Tipo de conector de display.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum ConnectorType {
    /// Desconhecido.
    #[default]
    Unknown = 0,
    /// VGA (D-Sub 15).
    VGA = 1,
    /// DVI-I.
    DVII = 2,
    /// DVI-D.
    DVID = 3,
    /// DVI-A.
    DVIA = 4,
    /// Composite.
    Composite = 5,
    /// S-Video.
    SVideo = 6,
    /// LVDS (laptop panel).
    LVDS = 7,
    /// Component.
    Component = 8,
    /// DisplayPort.
    DisplayPort = 9,
    /// HDMI Type A.
    HDMIA = 10,
    /// HDMI Type B.
    HDMIB = 11,
    /// TV.
    TV = 12,
    /// eDP (embedded DisplayPort).
    EDP = 13,
    /// Virtual.
    Virtual = 14,
    /// DSI (Display Serial Interface).
    DSI = 15,
    /// USB-C DisplayPort Alt Mode.
    USBC = 16,
}

impl ConnectorType {
    /// Converte de u8.
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Unknown),
            1 => Some(Self::VGA),
            2 => Some(Self::DVII),
            3 => Some(Self::DVID),
            4 => Some(Self::DVIA),
            5 => Some(Self::Composite),
            6 => Some(Self::SVideo),
            7 => Some(Self::LVDS),
            8 => Some(Self::Component),
            9 => Some(Self::DisplayPort),
            10 => Some(Self::HDMIA),
            11 => Some(Self::HDMIB),
            12 => Some(Self::TV),
            13 => Some(Self::EDP),
            14 => Some(Self::Virtual),
            15 => Some(Self::DSI),
            16 => Some(Self::USBC),
            _ => None,
        }
    }

    /// Nome do conector.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::VGA => "VGA",
            Self::DVII => "DVI-I",
            Self::DVID => "DVI-D",
            Self::DVIA => "DVI-A",
            Self::Composite => "Composite",
            Self::SVideo => "S-Video",
            Self::LVDS => "LVDS",
            Self::Component => "Component",
            Self::DisplayPort => "DisplayPort",
            Self::HDMIA => "HDMI-A",
            Self::HDMIB => "HDMI-B",
            Self::TV => "TV",
            Self::EDP => "eDP",
            Self::Virtual => "Virtual",
            Self::DSI => "DSI",
            Self::USBC => "USB-C",
        }
    }

    /// Verifica se é digital.
    #[inline]
    pub const fn is_digital(&self) -> bool {
        matches!(
            self,
            Self::DVID
                | Self::DisplayPort
                | Self::HDMIA
                | Self::HDMIB
                | Self::EDP
                | Self::DSI
                | Self::USBC
        )
    }

    /// Verifica se suporta áudio.
    #[inline]
    pub const fn supports_audio(&self) -> bool {
        matches!(
            self,
            Self::HDMIA | Self::HDMIB | Self::DisplayPort | Self::EDP | Self::USBC
        )
    }
}

/// Informações de um output/conector.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OutputInfo {
    /// ID do output.
    pub id: u32,
    /// Tipo de conector.
    pub connector: ConnectorType,
    /// Está conectado?
    pub connected: bool,
    /// Largura física em mm (0 se desconhecido).
    pub width_mm: u32,
    /// Altura física em mm (0 se desconhecido).
    pub height_mm: u32,
}

impl OutputInfo {
    /// Calcula DPI horizontal (se dimensões conhecidas).
    #[inline]
    pub fn dpi_x(&self, width_px: u32) -> Option<f32> {
        if self.width_mm > 0 {
            Some(width_px as f32 / (self.width_mm as f32 / 25.4))
        } else {
            None
        }
    }

    /// Calcula DPI vertical (se dimensões conhecidas).
    #[inline]
    pub fn dpi_y(&self, height_px: u32) -> Option<f32> {
        if self.height_mm > 0 {
            Some(height_px as f32 / (self.height_mm as f32 / 25.4))
        } else {
            None
        }
    }

    /// Calcula DPI médio.
    #[inline]
    pub fn dpi(&self, width_px: u32, height_px: u32) -> Option<f32> {
        match (self.dpi_x(width_px), self.dpi_y(height_px)) {
            (Some(x), Some(y)) => Some((x + y) / 2.0),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}
