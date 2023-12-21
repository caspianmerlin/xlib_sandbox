#![allow(non_upper_case_globals)]

use std::ffi::{CStr, CString, c_char};

use x11_dl::xlib::{Xlib, XSizeHints, PPosition, PSize, PMinSize, XWMHints, XClassHint, XTextProperty, True, StateHint, IconPixmapHint, InputHint, ExposureMask, KeyPressMask, ButtonPressMask, StructureNotifyMask, XGCValues, LineOnOffDash, CapRound, JoinRound, XEvent, Expose, _XGC, XFontStruct, _XDisplay, ConfigureNotify, ButtonPress, KeyPress};

static ICON_BYTES: &[u8] = &[
   0xc3, 0xc3, 0x7f, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00,
   0x00, 0x00, 0x80, 0x38, 0x00, 0x40, 0x00, 0x80, 0x24, 0x00, 0x00, 0x00,
   0x80, 0x44, 0x00, 0x00, 0x00, 0x80, 0x44, 0x00, 0x00, 0x00, 0x80, 0x74,
   0x00, 0x0f, 0x0c, 0x00, 0x7c, 0x3e, 0x41, 0x0e, 0x00, 0x44, 0x22, 0x41,
   0x02, 0x00, 0x84, 0x22, 0x46, 0x02, 0x00, 0x9c, 0x26, 0xcc, 0x02, 0x00,
   0x78, 0x3c, 0xcd, 0x36, 0x80, 0x00, 0x20, 0x06, 0x0c, 0x80, 0x01, 0x00,
   0x00, 0x00, 0x80, 0x01, 0x02, 0x40, 0x00, 0x80, 0x01, 0x06, 0x40, 0x00,
   0x80, 0x01, 0x04, 0x20, 0x00, 0x80, 0x01, 0x04, 0x20, 0x01, 0x80, 0x01,
   0x04, 0x20, 0x00, 0x80, 0x01, 0x04, 0x22, 0x00, 0x80, 0x01, 0x04, 0x33,
   0xf1, 0x81, 0x01, 0x88, 0x12, 0x31, 0x03, 0x01, 0x88, 0x12, 0x11, 0x02,
   0x00, 0x88, 0x12, 0x11, 0x02, 0x00, 0x48, 0x1a, 0x11, 0x02, 0x00, 0x70,
   0x04, 0x19, 0x82, 0x01, 0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x38,
   0x80, 0x01, 0x00, 0x00, 0xce, 0x80, 0x01, 0x00, 0x00, 0x83, 0x81, 0x81,
   0x07, 0x80, 0x01, 0x81, 0xe1, 0x04, 0xc0, 0x00, 0x83, 0x31, 0x08, 0x40,
   0x00, 0x82, 0x10, 0x08, 0x20, 0x00, 0x82, 0x19, 0x10, 0x30, 0x00, 0x86,
   0x0c, 0x30, 0x18, 0x00, 0x84, 0x04, 0x60, 0x0e, 0x00, 0xdc, 0x02, 0x80,
   0x03, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00
];
const ICON_BITMAP_WIDTH: usize = 40;
const ICON_BITMAP_HEIGHT: usize = 40;
static DASH_LIST: [c_char; 2] = [12, 24];
const TOO_SMALL: i32 = 0;
const BIG_ENOUGH: i32 = 1;

fn main() {
    unsafe {
        let xlib = Xlib::open().unwrap();
        let prog_name = std::env::args().next().unwrap();
        let prog_name_c_string = CString::new(prog_name.clone()).unwrap();
        let window_name = CString::new("Basic Window Program").unwrap();
        let icon_name = CString::new("basicwin").unwrap();
        let mut window_size = BIG_ENOUGH;

        let display = (xlib.XOpenDisplay)(std::ptr::null());
        if display.is_null() {
            let display_name = (xlib.XDisplayName)(std::ptr::null());
            let display_name_c_str = CStr::from_ptr(display_name);
            let display_name_cow = display_name_c_str.to_string_lossy();
            eprintln!("{}: Cannot connect to X server {}", &prog_name, display_name_cow);
            std::process::exit(-1);
        }

        let screen_num = (xlib.XDefaultScreen)(display);
        let display_width = (xlib.XDisplayWidth)(display, screen_num);
        let display_height = (xlib.XDisplayHeight)(display, screen_num);

        let (x, y) = (0, 0);
        let mut width = display_width / 3;
        let mut height = display_height / 4;

        let win = (xlib.XCreateSimpleWindow)(
            display,
            (xlib.XRootWindow)(display, screen_num),
            x,
            y,
            width as u32,
            height as u32,
            4,
            (xlib.XBlackPixel)(display, screen_num),
            (xlib.XWhitePixel)(display, screen_num)
        );

        let mut ptr = std::ptr::null_mut();
        let mut count = 0;
        if (xlib.XGetIconSizes)(display, (xlib.XRootWindow)(display, screen_num), &mut ptr, &mut count) == 0 {
            eprintln!("{}: Window manager didn't set icon sizes - using default.", &prog_name);
        }

        let icon_pixmap = (xlib.XCreateBitmapFromData)(
            display,
            win,
            ICON_BYTES.as_ptr().cast(),
            ICON_BITMAP_WIDTH as u32,
            ICON_BITMAP_HEIGHT as u32,
        );

        let mut size_hints: XSizeHints = std::mem::zeroed();
        size_hints.flags = PPosition | PSize | PMinSize;
        size_hints.min_width = 300;
        size_hints.min_height = 200;

        let mut wm_hints: XWMHints = std::mem::zeroed();
        let mut class_hints: XClassHint = std::mem::zeroed();
        let mut window_name_text_property: XTextProperty = std::mem::zeroed();
        let mut icon_name_text_property: XTextProperty = std::mem::zeroed();

        if (xlib.XStringListToTextProperty)(&mut (window_name.as_ptr() as *mut i8), 1, &mut window_name_text_property) == 0 {
            eprintln!("{}: structure allocation for windowName failed.", prog_name);
            std::process::exit(-1);
        }

        if (xlib.XStringListToTextProperty)(&mut (icon_name.as_ptr() as *mut i8), 1, &mut icon_name_text_property) == 0 {
            eprintln!("{}: structure allocation for iconName failed.", prog_name);
            std::process::exit(-1);
        }

        wm_hints.initial_state = 1;
        wm_hints.input = True;
        wm_hints.icon_pixmap = icon_pixmap;
        wm_hints.flags = StateHint | IconPixmapHint | InputHint;

        let res_class = CString::new("Basicwin").unwrap();
        class_hints.res_name = prog_name_c_string.as_ptr() as *mut _;
        class_hints.res_class = res_class.as_ptr() as *mut _;

        (xlib.XSetWMProperties)(
            display,
            win,
            &mut window_name_text_property,
            &mut icon_name_text_property,
            std::ptr::null_mut(),
            0,
            &mut size_hints,
            &mut wm_hints,
            &mut class_hints
        );

        (xlib.XSelectInput)(
            display,
            win,
            ExposureMask | KeyPressMask | ButtonPressMask | StructureNotifyMask
        );


        // Load font
        // let font_name_c_string = CString::new("9x15").unwrap();
        // let font_info = (xlib.XLoadQueryFont)(
        //     display,
        //     font_name_c_string.as_ptr()
        // );
        // if font_info.is_null() {
        //     eprintln!("{}: Cannot open 9x15 font", prog_name);
        //     std::process::exit(-1);
        // }

        // Get GC
        let value_mask = 0;
        let mut values: XGCValues = std::mem::zeroed();
        let line_width = 6;
        let line_style = LineOnOffDash;
        let cap_style = CapRound;
        let join_style = JoinRound;
        let dash_offset = 0;
        let gc = (xlib.XCreateGC)(
            display,
            win,
            value_mask,
            &mut values
        );
        // (xlib.XSetFont)(
        //     display,
        //     gc,
        //     (*font_info).fid,
        // );
        (xlib.XSetForeground)(
            display,
            gc,
            (xlib.XBlackPixel)(display, screen_num)
        );
        (xlib.XSetLineAttributes)(
            display,
            gc,
            line_width,
            line_style,
            cap_style,
            join_style
        );
        (xlib.XSetDashes)(
            display,
            gc,
            dash_offset,
            DASH_LIST.as_ptr(),
            DASH_LIST.len() as i32
        );

        (xlib.XMapWindow)(display, win);



        // Enter event loop
        let mut report: XEvent = std::mem::zeroed();
        loop {
            (xlib.XNextEvent)(display, &mut report);
            match report.type_ {
                Expose => {
                    if report.expose.count != 0 {
                        break;
                    }
                    if window_size == TOO_SMALL {
                        // let too_small_c_string = CString::new("Too Small").unwrap();
                        // let y_offset = (*font_info).ascent + 2;
                        // let x_offset = 2;
                        // (xlib.XDrawString)(
                        //     display,
                        //     win,
                        //     gc,
                        //     x_offset,
                        //     y_offset,
                        //     too_small_c_string.as_ptr(),
                        //     too_small_c_string.as_bytes().len() as i32
                        // );
                    } else {
                        //draw_text(&xlib, win, gc, display, screen_num, font_info, width, height);
                        draw_graphics(&xlib, win, gc, display, width, height);
                        
                    }
                },
                ConfigureNotify => {
                    width = report.configure.width;
                    height = report.configure.height;
                    if (width < size_hints.min_width) || (height < size_hints.min_height) {
                        window_size = TOO_SMALL;
                    } else {
                        window_size = BIG_ENOUGH;
                    }
                },
                ButtonPress | KeyPress => {
                    //(xlib.XUnloadFont)(display, (*font_info).fid);
                    (xlib.XFreeGC)(display, gc);
                    (xlib.XCloseDisplay)(display);
                    std::process::exit(1);
                },
                _ => {},
            }
        }




    }
}


unsafe fn draw_graphics(xlib: &Xlib, win: u64, gc: *mut _XGC, display: *mut _XDisplay, win_width: i32, win_height: i32) {
    let height = win_height / 2;
    let width = 3 * win_width / 4;
    let x = win_width / 2 - width / 2;
    let y = win_height / 2 - height / 2;
    (xlib.XDrawRectangle)(
        display,
        win,
        gc,
        x,
        y,
        width as u32,
        height as u32
    );
}

unsafe fn draw_text(xlib: &Xlib, win: u64, gc: *mut _XGC, display: *mut _XDisplay, screen_num: i32, font_info: *mut XFontStruct, win_width: i32, win_height: i32) {
    let display_height = (xlib.XDisplayHeight)(display, screen_num);
    let display_width = (xlib.XDisplayWidth)(display, screen_num);
    let display_depth = (xlib.XDefaultDepth)(display, screen_num);

    let cd_height = CString::new(format!(" Height - {display_height} pixels")).unwrap();
    let cd_width = CString::new(format!(" Width - {display_width} pixels")).unwrap();
    let cd_depth = CString::new(format!(" Depth - {display_depth} plane(s)")).unwrap();
    

    let string_1 = CString::new("Hi! I'm a window, who are you?").unwrap();
    let string_2 = CString::new("To terminate program; Press any key").unwrap();
    let string_3 = CString::new("or button while in this window.").unwrap();
    let string_4 = CString::new("Screen Dimensions:").unwrap();

    let mut len_1 = string_1.as_bytes().len() as i32;
    let mut len_2 = string_2.as_bytes().len() as i32;
    let mut len_3 = string_3.as_bytes().len() as i32;
    let len_4 = string_4.as_bytes().len() as i32;

    let width_1 = (xlib.XTextWidth)(font_info, string_1.as_ptr(), len_1);
    let width_2 = (xlib.XTextWidth)(font_info, string_1.as_ptr(), len_2);
    let width_3 = (xlib.XTextWidth)(font_info, string_1.as_ptr(), len_3);

    let font_height = (*font_info).ascent + (*font_info).descent;

    (xlib.XDrawString)(
        display,
        win,
        gc,
        (win_width - width_1) / 2,
        font_height,
        string_1.as_ptr(),
        len_1
    );

    (xlib.XDrawString)(
        display,
        win,
        gc,
        (win_width - width_2) / 2,
        win_height - (2 * font_height),
        string_2.as_ptr(),
        len_2
    );

    (xlib.XDrawString)(
        display,
        win,
        gc,
        (win_width - width_3) / 2,
        win_height - font_height,
        string_3.as_ptr(),
        len_3
    );

    len_1 = cd_height.as_bytes().len() as i32;
    len_2 = cd_width.as_bytes().len() as i32;
    len_3 = cd_depth.as_bytes().len() as i32;
    
    let x_offset = win_width / 4;
    let initial_y_offset = win_height / 2 - font_height - (*font_info).descent;

    (xlib.XDrawString)(
        display,
        win,
        gc,
        x_offset,
        initial_y_offset,
        string_4.as_ptr(),
        len_4
    );

    (xlib.XDrawString)(
        display,
        win,
        gc,
        x_offset,
        initial_y_offset + font_height,
        cd_height.as_ptr(),
        len_1
    );

    (xlib.XDrawString)(
        display,
        win,
        gc,
        x_offset,
        initial_y_offset + font_height * 2,
        cd_width.as_ptr(),
        len_2
    );

    (xlib.XDrawString)(
        display,
        win,
        gc,
        x_offset,
        initial_y_offset + font_height * 3,
        cd_depth.as_ptr(),
        len_3
    );

}