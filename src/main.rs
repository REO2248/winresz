use clap::Parser;
use winsafe::{
    co::{PROCESS, PROCESS_NAME, SWP},
    EnumWindows, HwndPlace, HPROCESS, HWND, POINT, SIZE,
};
use windows_sys::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWM_WINDOW_CORNER_PREFERENCE, DWMWCP_DEFAULT, DWMWCP_DONOTROUND, DWMWCP_ROUND, DWMWCP_ROUNDSMALL, DWMWA_BORDER_COLOR};
use winsafe::co::{WS, WS_EX};

mod model;
use model::*;

fn filter_target_windows(hwnd: &HWND, q: &TargetInformation) -> bool {
    if !q.title_contains.is_empty() {
        let Ok(title) = hwnd.GetWindowText() else {
            return false;
        };

        if q.title_contains
            .iter()
            .all(|s| !title.to_ascii_lowercase().contains(&s.to_ascii_lowercase()))
        {
            return false;
        }
    }

    if !q.path_endswith.is_empty() {
        let (_, pid) = hwnd.GetWindowThreadProcessId();

        let Ok(proc) = HPROCESS::OpenProcess(PROCESS::QUERY_LIMITED_INFORMATION, false, pid) else {
            return false;
        };

        let Ok(path) = proc.QueryFullProcessImageName(PROCESS_NAME::WIN32) else {
            return false;
        };

        if q.path_endswith
            .iter()
            .all(|s| !path.to_ascii_lowercase().ends_with(&s.to_ascii_lowercase()))
        {
            return false;
        }
    }

    true
}

fn window_callback(hwnd: HWND, op: &Cli) {
    if !filter_target_windows(&hwnd, &op.target) {
        return;
    }

    // Toggle border (style) if requested
    if let Some(border_on) = op.border {
        unsafe {
            // Read current styles
            let mut style = WS::from_raw(hwnd.GetWindowLongPtr(winsafe::co::GWLP::STYLE) as _);
            let mut exstyle = WS_EX::from_raw(hwnd.GetWindowLongPtr(winsafe::co::GWLP::EXSTYLE) as _);

            if border_on {
                // Ensure border styles are present
                style |= WS::BORDER | WS::THICKFRAME | WS::CAPTION;
            } else {
                // Remove border-related styles (frameless)
                style &= !(WS::BORDER | WS::THICKFRAME | WS::CAPTION);
                exstyle &= !WS_EX::WINDOWEDGE;
            }

            hwnd.SetWindowLongPtr(winsafe::co::GWLP::STYLE, style.raw() as _);
            hwnd.SetWindowLongPtr(winsafe::co::GWLP::EXSTYLE, exstyle.raw() as _);

            // Apply style changes by forcing a frame change
            hwnd.SetWindowPos(
                HwndPlace::None,
                POINT::with(0, 0),
                SIZE::default(),
                SWP::NOMOVE | SWP::NOSIZE | SWP::NOZORDER | SWP::FRAMECHANGED,
            )
            .unwrap();
        }
    }

    // Set border color if requested (Windows 11)
    if let Some(color) = op.border_color {
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd.ptr() as _,
                DWMWA_BORDER_COLOR as u32,
                &color as *const _ as _,
                std::mem::size_of::<u32>() as u32,
            );
        }
    }

    // Set corner preference if requested (Windows 11)
    if let Some(corner) = op.corner.as_ref() {
        let pref: DWM_WINDOW_CORNER_PREFERENCE = match corner {
            CornerPreference::Default => DWMWCP_DEFAULT,
            CornerPreference::DoNotRound => DWMWCP_DONOTROUND,
            CornerPreference::Round => DWMWCP_ROUND,
            CornerPreference::RoundSmall => DWMWCP_ROUNDSMALL,
        } as _;

        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd.ptr() as _,
                DWMWA_WINDOW_CORNER_PREFERENCE as u32,
                &pref as *const _ as _,
                std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
            );
        }
    }

    // Handle window sizing (after all style changes are applied)
    let client_size = Size::from(hwnd.GetClientRect().unwrap());
    
    let Some(size) = op.size else {
        println!("{}\t", client_size - op.target.offset);
        return;
    };

    // Recalculate border size after potential style changes
    let window_size = Size::from(hwnd.GetWindowRect().unwrap());
    let border = window_size - client_size;

    hwnd.SetWindowPos(
        HwndPlace::None,
        POINT::with(0, 0),
        (size + op.target.offset + border).into(),
        SWP::NOMOVE,
    )
    .unwrap();
}

fn main() {
    let c = Cli::parse();
    EnumWindows(|hwnd: HWND| -> bool {
        window_callback(hwnd, &c);
        true
    })
    .unwrap();
}
