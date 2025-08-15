use clap::{Args, Parser};
use std::fmt::{self, Display};
use std::io::{Error, ErrorKind};
use std::ops::{Add, Sub};
use winsafe::{RECT, SIZE};

#[derive(Debug, Clone, Args)]
pub struct TargetInformation {
    #[arg(short, long, help = "Filter by binary path")]
    pub path_endswith: Vec<String>,
    #[arg(short, long, help = "Filter by title")]
    pub title_contains: Vec<String>,
    #[arg(short, long, value_parser = parse_size, default_value_t = Size::default(), help = "Additional offset for window")]
    pub offset: Size,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(flatten)]
    pub target: TargetInformation,
    #[arg(value_parser = parse_size, help = "Set the window size if it's set.")]
    pub size: Option<Size>,
}

#[derive(Debug, Copy, Clone, Parser, Default)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Add for Size {
    type Output = Size;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<RECT> for Size {
    fn from(v: RECT) -> Self {
        Self {
            x: (v.right - v.left) as usize,
            y: (v.bottom - v.top) as usize,
        }
    }
}

impl From<Size> for SIZE {
    fn from(v: Size) -> Self {
        Self {
            cx: v.x as i32,
            cy: v.y as i32,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

fn get_resolution_by_name(name: &str) -> Option<Size> {
    match name.to_ascii_lowercase().as_str() {
        "vga" => Some(Size { x: 640, y: 480 }),
        "svga" => Some(Size { x: 800, y: 600 }),
        "xga" => Some(Size { x: 1024, y: 768 }),
        "sxga" => Some(Size { x: 1280, y: 1024 }),
        "uxga" => Some(Size { x: 1600, y: 1200 }),
        "hd" => Some(Size { x: 1280, y: 720 }),
        "fhd" => Some(Size { x: 1920, y: 1080 }),
        "wqhd" => Some(Size { x: 2560, y: 1440 }),
        "4k" | "uhd" => Some(Size { x: 3840, y: 2160 }),
        "8k" => Some(Size { x: 7680, y: 4320 }),
        "wxga" => Some(Size { x: 1280, y: 800 }),
        "wsxga+" | "wsxga" => Some(Size { x: 1680, y: 1050 }),
        "wuxga" => Some(Size { x: 1920, y: 1200 }),
        "wqxga" => Some(Size { x: 2560, y: 1600 }),
        "wquxga" => Some(Size { x: 3840, y: 2400 }),
        "uwfhd" | "ultrawide" => Some(Size { x: 2560, y: 1080 }),
        "uwqhd" | "ultrawide1440" => Some(Size { x: 3440, y: 1440 }),
        "uw4k" | "ultrawide4k" => Some(Size { x: 5120, y: 2160 }),
        "dci4k" => Some(Size { x: 4096, y: 2160 }),
        _ => None,
    }
}

fn parse_size(arg: &str) -> Result<Size, Error> {
    if let Some(size) = get_resolution_by_name(arg) {
        return Ok(size);
    }

    if arg.ends_with('p') || arg.ends_with('P') {
        let height_str = &arg[..arg.len() - 1];
        if let Ok(height) = height_str.parse::<usize>() {
            let width = (height * 16 + 8) / 9;
            return Ok(Size { x: width, y: height });
        }
    }

    let mut res = arg.split('x');

    let Some(x) = res.next() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected Input"));
    };

    let Some(y) = res.next() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected Input"));
    };

    let None = res.next() else {
        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected Input"));
    };

    Ok(Size {
        x: x.parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Unexpected Input"))?,
        y: y.parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Unexpected Input"))?,
    })
}
