// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(non_uppercase_statics)];
#[allow(non_camel_case_types)];

use std::libc::*;

pub type XID = c_ulong;

pub type Mask = c_ulong;

pub type Atom = c_ulong;

pub type VisualID = c_ulong;

pub type Time = c_ulong;

pub type Window = XID;

pub type Drawable = XID;

pub type Font = XID;

pub type Pixmap = XID;

pub type Cursor = XID;

pub type Colormap = XID;

pub type GContext = XID;

pub type KeySym = XID;

pub type KeyCode = c_uchar;

pub type XPointer = *c_char;

pub struct XTextProperty {
    value: *c_char,
    encoding: Atom,
    format: c_int,
    nitems: c_ulong,
}

pub struct struct__XExtData {
    number: c_int,
    next: *c_void /* struct__XExtData */,
    free_private: *u8,
    private_data: XPointer,
}

pub type XExtData = struct__XExtData;

pub struct XExtCodes {
    extension: c_int,
    major_opcode: c_int,
    first_event: c_int,
    first_error: c_int,
}

pub struct XPixmapFormatValues {
    depth: c_int,
    bits_per_pixel: c_int,
    scanline_pad: c_int,
}

pub struct XGCValues {
    function: c_int,
    plane_mask: c_ulong,
    foreground: c_ulong,
    background: c_ulong,
    line_width: c_int,
    line_style: c_int,
    cap_style: c_int,
    join_style: c_int,
    fill_style: c_int,
    fill_rule: c_int,
    arc_mode: c_int,
    tile: Pixmap,
    stipple: Pixmap,
    ts_x_origin: c_int,
    ts_y_origin: c_int,
    font: Font,
    subwindow_mode: c_int,
    graphics_exposures: c_int,
    clip_x_origin: c_int,
    clip_y_origin: c_int,
    clip_mask: Pixmap,
    dash_offset: c_int,
    dashes: c_char,
}

pub type struct__XGC = c_void;

pub type GC = *struct__XGC;

pub struct Visual {
    ext_data: *XExtData,
    visualid: VisualID,
    _class: c_int,
    red_mask: c_ulong,
    green_mask: c_ulong,
    blue_mask: c_ulong,
    bits_per_rgb: c_int,
    map_entries: c_int,
}

pub struct Depth {
    depth: c_int,
    nvisuals: c_int,
    visuals: *Visual,
}

pub type struct__XDisplay = c_void;

pub struct Screen {
    ext_data: *XExtData,
    display: *c_void /* struct__XDisplay */,
    root: Window,
    width: c_int,
    height: c_int,
    mwidth: c_int,
    mheight: c_int,
    ndepths: c_int,
    depths: *Depth,
    root_depth: c_int,
    root_visual: *Visual,
    default_gc: *c_void /* GC */,
    cmap: Colormap,
    white_pixel: c_ulong,
    black_pixel: c_ulong,
    max_maps: c_int,
    min_maps: c_int,
    backing_store: c_int,
    save_unders: c_int,
    root_input_mask: c_long,
}

pub struct ScreenFormat {
    ext_data: *XExtData,
    depth: c_int,
    bits_per_pixel: c_int,
    scanline_pad: c_int,
}

pub struct XSetWindowAttributes {
    background_pixmap: Pixmap,
    background_pixel: c_ulong,
    border_pixmap: Pixmap,
    border_pixel: c_ulong,
    bit_gravity: c_int,
    win_gravity: c_int,
    backing_store: c_int,
    backing_planes: c_ulong,
    backing_pixel: c_ulong,
    save_under: c_int,
    event_mask: c_long,
    do_not_propagate_mask: c_long,
    override_redirect: c_int,
    colormap: Colormap,
    cursor: Cursor,
}

pub struct XWindowAttributes {
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
    depth: c_int,
    visual: *Visual,
    root: Window,
    _class: c_int,
    bit_gravity: c_int,
    win_gravity: c_int,
    backing_store: c_int,
    backing_planes: c_ulong,
    backing_pixel: c_ulong,
    save_under: c_int,
    colormap: Colormap,
    map_installed: c_int,
    map_state: c_int,
    all_event_masks: c_long,
    your_event_mask: c_long,
    do_not_propagate_mask: c_long,
    override_redirect: c_int,
    screen: *Screen,
}

pub struct XHostAddress {
    family: c_int,
    length: c_int,
    address: *c_char,
}

pub struct XServerInterpretedAddress {
    typelength: c_int,
    valuelength: c_int,
    _type: *c_char,
    value: *c_char,
}

pub struct struct__XImage {
    width: c_int,
    height: c_int,
    xoffset: c_int,
    format: c_int,
    data: *c_char,
    byte_order: c_int,
    bitmap_unit: c_int,
    bitmap_bit_order: c_int,
    bitmap_pad: c_int,
    depth: c_int,
    bytes_per_line: c_int,
    bits_per_pixel: c_int,
    red_mask: c_ulong,
    green_mask: c_ulong,
    blue_mask: c_ulong,
    obdata: XPointer,
    f: struct_funcs,
}

pub struct struct_funcs {
    create_image: *u8,
    destroy_image: *u8,
    get_pixel: *u8,
    put_pixel: *u8,
    sub_image: *u8,
    add_pixel: *u8,
}

pub type XImage = struct__XImage;

pub struct XWindowChanges {
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
    sibling: Window,
    stack_mode: c_int,
}

pub struct XColor {
    pixel: c_ulong,
    red: c_ushort,
    green: c_ushort,
    blue: c_ushort,
    flags: c_char,
    pad: c_char,
}

pub struct XSegment {
    x1: c_short,
    y1: c_short,
    x2: c_short,
    y2: c_short,
}

pub struct XPoint {
    x: c_short,
    y: c_short,
}

pub struct XRectangle {
    x: c_short,
    y: c_short,
    width: c_ushort,
    height: c_ushort,
}

pub struct XArc {
    x: c_short,
    y: c_short,
    width: c_ushort,
    height: c_ushort,
    angle1: c_short,
    angle2: c_short,
}

pub struct XKeyboardControl {
    key_click_percent: c_int,
    bell_percent: c_int,
    bell_pitch: c_int,
    bell_duration: c_int,
    led: c_int,
    led_mode: c_int,
    key: c_int,
    auto_repeat_mode: c_int,
}

pub struct XKeyboardState {
    key_click_percent: c_int,
    bell_percent: c_int,
    bell_pitch: c_uint,
    bell_duration: c_uint,
    led_mask: c_ulong,
    global_auto_repeat: c_int,
    auto_repeats: (c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char),
}

pub struct XTimeCoord {
    time: Time,
    x: c_short,
    y: c_short,
}

pub struct XModifierKeymap {
    max_keypermod: c_int,
    modifiermap: *KeyCode,
}

pub type Display = struct__XDisplay;

pub type struct__XPrivate = c_void;

pub type struct__XrmHashBucketRec = c_void;

pub type _XPrivDisplay = *struct_unnamed1;

pub struct XKeyEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    root: Window,
    subwindow: Window,
    time: Time,
    x: c_int,
    y: c_int,
    x_root: c_int,
    y_root: c_int,
    state: c_uint,
    keycode: c_uint,
    same_screen: c_int,
}

pub type XKeyPressedEvent = XKeyEvent;

pub type XKeyReleasedEvent = XKeyEvent;

pub struct XButtonEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    root: Window,
    subwindow: Window,
    time: Time,
    x: c_int,
    y: c_int,
    x_root: c_int,
    y_root: c_int,
    state: c_uint,
    button: c_uint,
    same_screen: c_int,
}

pub type XButtonPressedEvent = XButtonEvent;

pub type XButtonReleasedEvent = XButtonEvent;

pub struct XMotionEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    root: Window,
    subwindow: Window,
    time: Time,
    x: c_int,
    y: c_int,
    x_root: c_int,
    y_root: c_int,
    state: c_uint,
    is_hint: c_char,
    same_screen: c_int,
}

pub type XPointerMovedEvent = XMotionEvent;

pub struct XCrossingEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    root: Window,
    subwindow: Window,
    time: Time,
    x: c_int,
    y: c_int,
    x_root: c_int,
    y_root: c_int,
    mode: c_int,
    detail: c_int,
    same_screen: c_int,
    focus: c_int,
    state: c_uint,
}

pub type XEnterWindowEvent = XCrossingEvent;

pub type XLeaveWindowEvent = XCrossingEvent;

pub struct XFocusChangeEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    mode: c_int,
    detail: c_int,
}

pub type XFocusInEvent = XFocusChangeEvent;

pub type XFocusOutEvent = XFocusChangeEvent;

pub struct XKeymapEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    key_vector: (c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char),
}

pub struct XExposeEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    count: c_int,
}

pub struct XGraphicsExposeEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    drawable: Drawable,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    count: c_int,
    major_code: c_int,
    minor_code: c_int,
}

pub struct XNoExposeEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    drawable: Drawable,
    major_code: c_int,
    minor_code: c_int,
}

pub struct XVisibilityEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    state: c_int,
}

pub struct XCreateWindowEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    parent: Window,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
    override_redirect: c_int,
}

pub struct XDestroyWindowEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
}

pub struct XUnmapEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
    from_configure: c_int,
}

pub struct XMapEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
    override_redirect: c_int,
}

pub struct XMapRequestEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    parent: Window,
    window: Window,
}

pub struct XReparentEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
    parent: Window,
    x: c_int,
    y: c_int,
    override_redirect: c_int,
}

pub struct XConfigureEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
    above: Window,
    override_redirect: c_int,
}

pub struct XGravityEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
    x: c_int,
    y: c_int,
}

pub struct XResizeRequestEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    width: c_int,
    height: c_int,
}

pub struct XConfigureRequestEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    parent: Window,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
    above: Window,
    detail: c_int,
    value_mask: c_ulong,
}

pub struct XCirculateEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    event: Window,
    window: Window,
    place: c_int,
}

pub struct XCirculateRequestEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    parent: Window,
    window: Window,
    place: c_int,
}

pub struct XPropertyEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    atom: Atom,
    time: Time,
    state: c_int,
}

pub struct XSelectionClearEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    selection: Atom,
    time: Time,
}

pub struct XSelectionRequestEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    owner: Window,
    requestor: Window,
    selection: Atom,
    target: Atom,
    property: Atom,
    time: Time,
}

pub struct XSelectionEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    requestor: Window,
    selection: Atom,
    target: Atom,
    property: Atom,
    time: Time,
}

pub struct XColormapEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    colormap: Colormap,
    _new: c_int,
    state: c_int,
}

pub struct XClientMessageEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    message_type: Atom,
    format: c_int,
    data: union_unnamed2,
}

pub struct XMappingEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
    request: c_int,
    first_keycode: c_int,
    count: c_int,
}

pub struct XErrorEvent {
    _type: c_int,
    display: *Display,
    resourceid: XID,
    serial: c_ulong,
    error_code: c_uchar,
    request_code: c_uchar,
    minor_code: c_uchar,
}

pub struct XAnyEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    window: Window,
}

pub struct XGenericEvent {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    extension: c_int,
    evtype: c_int,
}

pub struct XGenericEventCookie {
    _type: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *Display,
    extension: c_int,
    evtype: c_int,
    cookie: c_uint,
    data: *c_void,
}

pub type union__XEvent = c_void /* FIXME: union type */;

pub type XEvent = union__XEvent;

pub struct XCharStruct {
    lbearing: c_short,
    rbearing: c_short,
    width: c_short,
    ascent: c_short,
    descent: c_short,
    attributes: c_ushort,
}

pub struct XFontProp {
    name: Atom,
    card32: c_ulong,
}

pub struct XFontStruct {
    ext_data: *XExtData,
    fid: Font,
    direction: c_uint,
    min_char_or_byte2: c_uint,
    max_char_or_byte2: c_uint,
    min_byte1: c_uint,
    max_byte1: c_uint,
    all_chars_exist: c_int,
    default_char: c_uint,
    n_properties: c_int,
    properties: *XFontProp,
    min_bounds: XCharStruct,
    max_bounds: XCharStruct,
    per_char: *XCharStruct,
    ascent: c_int,
    descent: c_int,
}

pub struct XTextItem {
    chars: *c_char,
    nchars: c_int,
    delta: c_int,
    font: Font,
}

pub struct XChar2b {
    byte1: c_uchar,
    byte2: c_uchar,
}

pub struct XTextItem16 {
    chars: *XChar2b,
    nchars: c_int,
    delta: c_int,
    font: Font,
}

pub type XEDataObject = c_void /* FIXME: union type */;

pub struct XFontSetExtents {
    max_ink_extent: XRectangle,
    max_logical_extent: XRectangle,
}

pub type struct__XOM = c_void;

pub type XOM = *struct__XOM;

pub type struct__XOC = c_void;

pub type XOC = *struct__XOC;

pub type XFontSet = *struct__XOC;

pub struct XmbTextItem {
    chars: *c_char,
    nchars: c_int,
    delta: c_int,
    font_set: *c_void /* XFontSet */,
}

pub struct XwcTextItem {
    chars: *wchar_t,
    nchars: c_int,
    delta: c_int,
    font_set: *c_void /* XFontSet */,
}

pub struct XOMCharSetList {
    charset_count: c_int,
    charset_list: **c_char,
}


pub type XOrientation = c_uint;
pub static XOMOrientation_LTR_TTB: u32 = 0_u32;
pub static XOMOrientation_RTL_TTB: u32 = 1_u32;
pub static XOMOrientation_TTB_LTR: u32 = 2_u32;
pub static XOMOrientation_TTB_RTL: u32 = 3_u32;
pub static XOMOrientation_Context: u32 = 4_u32;

pub struct XOMOrientation {
    num_orientation: c_int,
    orientation: *XOrientation,
}

pub struct XOMFontInfo {
    num_font: c_int,
    font_struct_list: **XFontStruct,
    font_name_list: **c_char,
}

pub type struct__XIM = c_void;

pub type XIM = *struct__XIM;

pub type struct__XIC = c_void;

pub type XIC = *struct__XIC;

pub type XIMProc = *u8;

pub type XICProc = *u8;

pub type XIDProc = *u8;

pub type XIMStyle = c_ulong;

pub struct XIMStyles {
    count_styles: c_ushort,
    supported_styles: *XIMStyle,
}

pub type XVaNestedList = *c_void;

pub struct XIMCallback {
    client_data: XPointer,
    callback: XIMProc,
}

pub struct XICCallback {
    client_data: XPointer,
    callback: XICProc,
}

pub type XIMFeedback = c_ulong;

pub struct struct__XIMText {
    length: c_ushort,
    feedback: *XIMFeedback,
    encoding_is_wchar: c_int,
    string: union_unnamed3,
}

pub type XIMText = struct__XIMText;

pub type XIMPreeditState = c_ulong;

pub struct struct__XIMPreeditStateNotifyCallbackStruct {
    state: XIMPreeditState,
}

pub type XIMPreeditStateNotifyCallbackStruct = struct__XIMPreeditStateNotifyCallbackStruct;

pub type XIMResetState = c_ulong;

pub type XIMStringConversionFeedback = c_ulong;

pub struct struct__XIMStringConversionText {
    length: c_ushort,
    feedback: *XIMStringConversionFeedback,
    encoding_is_wchar: c_int,
    string: union_unnamed4,
}

pub type XIMStringConversionText = struct__XIMStringConversionText;

pub type XIMStringConversionPosition = c_ushort;

pub type XIMStringConversionType = c_ushort;

pub type XIMStringConversionOperation = c_ushort;


pub type XIMCaretDirection = c_uint;
pub static XIMForwardChar: u32 = 0_u32;
pub static XIMBackwardChar: u32 = 1_u32;
pub static XIMForwardWord: u32 = 2_u32;
pub static XIMBackwardWord: u32 = 3_u32;
pub static XIMCaretUp: u32 = 4_u32;
pub static XIMCaretDown: u32 = 5_u32;
pub static XIMNextLine: u32 = 6_u32;
pub static XIMPreviousLine: u32 = 7_u32;
pub static XIMLineStart: u32 = 8_u32;
pub static XIMLineEnd: u32 = 9_u32;
pub static XIMAbsolutePosition: u32 = 10_u32;
pub static XIMDontChange: u32 = 11_u32;

pub struct struct__XIMStringConversionCallbackStruct {
    position: XIMStringConversionPosition,
    direction: XIMCaretDirection,
    operation: XIMStringConversionOperation,
    factor: c_ushort,
    text: *XIMStringConversionText,
}

pub type XIMStringConversionCallbackStruct = struct__XIMStringConversionCallbackStruct;

pub struct struct__XIMPreeditDrawCallbackStruct {
    caret: c_int,
    chg_first: c_int,
    chg_length: c_int,
    text: *XIMText,
}

pub type XIMPreeditDrawCallbackStruct = struct__XIMPreeditDrawCallbackStruct;


pub type XIMCaretStyle = c_uint;
pub static XIMIsInvisible: u32 = 0_u32;
pub static XIMIsPrimary: u32 = 1_u32;
pub static XIMIsSecondary: u32 = 2_u32;

pub struct struct__XIMPreeditCaretCallbackStruct {
    position: c_int,
    direction: XIMCaretDirection,
    style: XIMCaretStyle,
}

pub type XIMPreeditCaretCallbackStruct = struct__XIMPreeditCaretCallbackStruct;


pub type XIMStatusDataType = c_uint;
pub static XIMTextType: u32 = 0_u32;
pub static XIMBitmapType: u32 = 1_u32;

pub struct struct__XIMStatusDrawCallbackStruct {
    _type: XIMStatusDataType,
    data: union_unnamed5,
}

pub type XIMStatusDrawCallbackStruct = struct__XIMStatusDrawCallbackStruct;

pub struct struct__XIMHotKeyTrigger {
    keysym: KeySym,
    modifier: c_int,
    modifier_mask: c_int,
}

pub type XIMHotKeyTrigger = struct__XIMHotKeyTrigger;

pub struct struct__XIMHotKeyTriggers {
    num_hot_key: c_int,
    key: *XIMHotKeyTrigger,
}

pub type XIMHotKeyTriggers = struct__XIMHotKeyTriggers;

pub type XIMHotKeyState = c_ulong;

pub struct XIMValuesList {
    count_values: c_ushort,
    supported_values: **c_char,
}

/* FIXME: global variable _Xdebug */

pub type XErrorHandler = *u8;

pub type XIOErrorHandler = *u8;

pub type XConnectionWatchProc = *u8;

pub type union_unnamed3 = c_void /* FIXME: union type */;

pub type union_unnamed5 = c_void /* FIXME: union type */;

pub type union_unnamed2 = c_void /* FIXME: union type */;

pub type union_unnamed4 = c_void /* FIXME: union type */;

pub struct struct_unnamed1 {
    ext_data: *XExtData,
    private1: *c_void /* struct__XPrivate */,
    fd: c_int,
    private2: c_int,
    proto_major_version: c_int,
    proto_minor_version: c_int,
    vendor: *c_char,
    private3: XID,
    private4: XID,
    private5: XID,
    private6: c_int,
    resource_alloc: *u8,
    byte_order: c_int,
    bitmap_unit: c_int,
    bitmap_pad: c_int,
    bitmap_bit_order: c_int,
    nformats: c_int,
    pixmap_format: *ScreenFormat,
    private8: c_int,
    release: c_int,
    private9: *c_void /* struct__XPrivate */,
    private10: *c_void /* struct__XPrivate */,
    qlen: c_int,
    last_request_read: c_ulong,
    request: c_ulong,
    private11: XPointer,
    private12: XPointer,
    private13: XPointer,
    private14: XPointer,
    max_request_size: c_uint,
    db: *c_void /* struct__XrmHashBucketRec */,
    private15: *u8,
    display_name: *c_char,
    default_screen: c_int,
    nscreens: c_int,
    screens: *Screen,
    motion_buffer: c_ulong,
    private16: c_ulong,
    min_keycode: c_int,
    max_keycode: c_int,
    private17: XPointer,
    private18: XPointer,
    private19: c_int,
    xdefaults: *c_char,
}

// Additions --pcwalton

pub struct XVisualInfo {
    visual: *Visual,
    visualid: VisualID,
    screen: c_int,
    depth: c_int,
    class: c_int,
    red_mask: c_ulong,
    green_mask: c_ulong,
    blue_mask: c_ulong,
    colormap_size: c_int,
    bits_per_rgb: c_int,
}

pub static ZPixmap: c_int = 2;  // depth == drawable depth

#[link(name="X11")]
extern {

pub fn XGetTextProperty(arg0: *Display, arg1: Window, arg2: *mut XTextProperty, arg3: Atom) -> c_int;

pub fn XGetWMName(arg0: *Display, arg1: Window, arg2: *mut XTextProperty) -> c_int;

pub fn Xutf8TextPropertyToTextList(arg0: *Display, arg1: *XTextProperty, arg2: *mut *mut *c_char, arg3: *mut c_int) -> c_int;

pub fn _Xmblen(arg0: *c_char, arg1: c_int) -> c_int;

pub fn XLoadQueryFont(arg0: *Display, arg1: *c_char) -> *XFontStruct;

pub fn XQueryFont(arg0: *Display, arg1: XID) -> *XFontStruct;

pub fn XGetMotionEvents(arg0: *Display, arg1: Window, arg2: Time, arg3: Time, arg4: *c_int) -> *XTimeCoord;

pub fn XDeleteModifiermapEntry(arg0: *XModifierKeymap, arg1: KeyCode, arg2: c_int) -> *XModifierKeymap;

pub fn XGetModifierMapping(arg0: *Display) -> *XModifierKeymap;

pub fn XInsertModifiermapEntry(arg0: *XModifierKeymap, arg1: KeyCode, arg2: c_int) -> *XModifierKeymap;

pub fn XNewModifiermap(arg0: c_int) -> *XModifierKeymap;

pub fn XCreateImage(arg0: *Display, arg1: *Visual, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: *c_char, arg6: c_uint, arg7: c_uint, arg8: c_int, arg9: c_int) -> *XImage;

pub fn XInitImage(arg0: *XImage) -> c_int;

pub fn XGetImage(arg0: *Display, arg1: Drawable, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_ulong, arg7: c_int) -> *XImage;

pub fn XGetSubImage(arg0: *Display, arg1: Drawable, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_ulong, arg7: c_int, arg8: *XImage, arg9: c_int, arg10: c_int) -> *XImage;

pub fn XOpenDisplay(arg0: *c_char) -> *Display;

pub fn XrmInitialize();

pub fn XFetchBytes(arg0: *Display, arg1: *c_int) -> *c_char;

pub fn XFetchBuffer(arg0: *Display, arg1: *c_int, arg2: c_int) -> *c_char;

pub fn XGetAtomName(arg0: *Display, arg1: Atom) -> *c_char;

pub fn XGetAtomNames(arg0: *Display, arg1: *Atom, arg2: c_int, arg3: **c_char) -> c_int;

pub fn XGetDefault(arg0: *Display, arg1: *c_char, arg2: *c_char) -> *c_char;

pub fn XDisplayName(arg0: *c_char) -> *c_char;

pub fn XKeysymToString(arg0: KeySym) -> *c_char;

pub fn XSynchronize(arg0: *Display, arg1: c_int) -> *u8;

pub fn XSetAfterFunction(arg0: *Display, arg1: *u8) -> *u8;

pub fn XInternAtom(arg0: *Display, arg1: *c_char, arg2: c_char) -> Atom;

pub fn XInternAtoms(arg0: *Display, arg1: **c_char, arg2: c_int, arg3: c_int, arg4: *Atom) -> c_int;

pub fn XCopyColormapAndFree(arg0: *Display, arg1: Colormap) -> Colormap;

pub fn XCreateColormap(arg0: *Display, arg1: Window, arg2: *Visual, arg3: c_int) -> Colormap;

pub fn XCreatePixmapCursor(arg0: *Display, arg1: Pixmap, arg2: Pixmap, arg3: *XColor, arg4: *XColor, arg5: c_uint, arg6: c_uint) -> Cursor;

pub fn XCreateGlyphCursor(arg0: *Display, arg1: Font, arg2: Font, arg3: c_uint, arg4: c_uint, arg5: *XColor, arg6: *XColor) -> Cursor;

pub fn XCreateFontCursor(arg0: *Display, arg1: c_uint) -> Cursor;

pub fn XLoadFont(arg0: *Display, arg1: *c_char) -> Font;

pub fn XCreateGC(arg0: *Display, arg1: Drawable, arg2: c_ulong, arg3: *XGCValues) -> GC;

pub fn XGContextFromGC(arg0: GC) -> GContext;

pub fn XFlushGC(arg0: *Display, arg1: GC);

pub fn XCreatePixmap(arg0: *Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: c_uint) -> Pixmap;

pub fn XCreateBitmapFromData(arg0: *Display, arg1: Drawable, arg2: *c_char, arg3: c_uint, arg4: c_uint) -> Pixmap;

pub fn XCreatePixmapFromBitmapData(arg0: *Display, arg1: Drawable, arg2: *c_char, arg3: c_uint, arg4: c_uint, arg5: c_ulong, arg6: c_ulong, arg7: c_uint) -> Pixmap;

pub fn XCreateSimpleWindow(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_uint, arg7: c_ulong, arg8: c_ulong) -> Window;

pub fn XGetSelectionOwner(arg0: *Display, arg1: Atom) -> Window;

pub fn XCreateWindow(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_uint, arg9: *Visual, arg10: c_ulong, arg11: *XSetWindowAttributes) -> Window;

pub fn XListInstalledColormaps(arg0: *Display, arg1: Window, arg2: *c_int) -> *Colormap;

pub fn XListFonts(arg0: *Display, arg1: *c_char, arg2: c_int, arg3: *c_int) -> **c_char;

pub fn XListFontsWithInfo(arg0: *Display, arg1: *c_char, arg2: c_int, arg3: *c_int, arg4: **XFontStruct) -> **c_char;

pub fn XGetFontPath(arg0: *Display, arg1: *c_int) -> **c_char;

pub fn XListExtensions(arg0: *Display, arg1: *c_int) -> **c_char;

pub fn XListProperties(arg0: *Display, arg1: Window, arg2: *c_int) -> *Atom;

pub fn XListHosts(arg0: *Display, arg1: *c_int, arg2: *c_int) -> *XHostAddress;

pub fn XKeycodeToKeysym(arg0: *Display, arg1: KeyCode, arg2: c_int) -> KeySym;

pub fn XLookupKeysym(arg0: *XKeyEvent, arg1: c_int) -> KeySym;

pub fn XGetKeyboardMapping(arg0: *Display, arg1: KeyCode, arg2: c_int, arg3: *c_int) -> *KeySym;

pub fn XStringToKeysym(arg0: *c_char) -> KeySym;

pub fn XMaxRequestSize(arg0: *Display) -> c_long;

pub fn XExtendedMaxRequestSize(arg0: *Display) -> c_long;

pub fn XResourceManagerString(arg0: *Display) -> *c_char;

pub fn XScreenResourceString(arg0: *Screen) -> *c_char;

pub fn XDisplayMotionBufferSize(arg0: *Display) -> c_ulong;

pub fn XVisualIDFromVisual(arg0: *Visual) -> VisualID;

pub fn XInitThreads() -> c_int;

pub fn XLockDisplay(arg0: *Display);

pub fn XUnlockDisplay(arg0: *Display);

pub fn XInitExtension(arg0: *Display, arg1: *c_char) -> *XExtCodes;

pub fn XAddExtension(arg0: *Display) -> *XExtCodes;

pub fn XFindOnExtensionList(arg0: **XExtData, arg1: c_int) -> *XExtData;

pub fn XEHeadOfExtensionList(arg0: XEDataObject) -> **XExtData;

pub fn XRootWindow(arg0: *Display, arg1: c_int) -> Window;

pub fn XDefaultRootWindow(arg0: *Display) -> Window;

pub fn XRootWindowOfScreen(arg0: *Screen) -> Window;

pub fn XDefaultVisual(arg0: *Display, arg1: c_int) -> *Visual;

pub fn XDefaultVisualOfScreen(arg0: *Screen) -> *Visual;

pub fn XDefaultGC(arg0: *Display, arg1: c_int) -> GC;

pub fn XDefaultGCOfScreen(arg0: *Screen) -> GC;

pub fn XBlackPixel(arg0: *Display, arg1: c_int) -> c_ulong;

pub fn XWhitePixel(arg0: *Display, arg1: c_int) -> c_ulong;

pub fn XAllPlanes() -> c_ulong;

pub fn XBlackPixelOfScreen(arg0: *Screen) -> c_ulong;

pub fn XWhitePixelOfScreen(arg0: *Screen) -> c_ulong;

pub fn XNextRequest(arg0: *Display) -> c_ulong;

pub fn XLastKnownRequestProcessed(arg0: *Display) -> c_ulong;

pub fn XServerVendor(arg0: *Display) -> *c_char;

pub fn XDisplayString(arg0: *Display) -> *c_char;

pub fn XDefaultColormap(arg0: *Display, arg1: c_int) -> Colormap;

pub fn XDefaultColormapOfScreen(arg0: *Screen) -> Colormap;

pub fn XDisplayOfScreen(arg0: *Screen) -> *Display;

pub fn XScreenOfDisplay(arg0: *Display, arg1: c_int) -> *Screen;

pub fn XDefaultScreenOfDisplay(arg0: *Display) -> *Screen;

pub fn XEventMaskOfScreen(arg0: *Screen) -> c_long;

pub fn XScreenNumberOfScreen(arg0: *Screen) -> c_int;

pub fn XSetErrorHandler(arg0: XErrorHandler) -> XErrorHandler;

pub fn XSetIOErrorHandler(arg0: XIOErrorHandler) -> XIOErrorHandler;

pub fn XListPixmapFormats(arg0: *Display, arg1: *c_int) -> *XPixmapFormatValues;

pub fn XListDepths(arg0: *Display, arg1: c_int, arg2: *c_int) -> *c_int;

pub fn XReconfigureWMWindow(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_uint, arg4: *XWindowChanges) -> c_int;

pub fn XGetWMProtocols(arg0: *Display, arg1: Window, arg2: **Atom, arg3: *c_int) -> c_int;

pub fn XSetWMProtocols(arg0: *Display, arg1: Window, arg2: *Atom, arg3: c_int) -> c_int;

pub fn XIconifyWindow(arg0: *Display, arg1: Window, arg2: c_int) -> c_int;

pub fn XWithdrawWindow(arg0: *Display, arg1: Window, arg2: c_int) -> c_int;

pub fn XGetCommand(arg0: *Display, arg1: Window, arg2: ***c_char, arg3: *c_int) -> c_int;

pub fn XGetWMColormapWindows(arg0: *Display, arg1: Window, arg2: **Window, arg3: *c_int) -> c_int;

pub fn XSetWMColormapWindows(arg0: *Display, arg1: Window, arg2: *Window, arg3: c_int) -> c_int;

pub fn XFreeStringList(arg0: *mut *c_char);

pub fn XSetTransientForHint(arg0: *Display, arg1: Window, arg2: Window) -> c_int;

pub fn XActivateScreenSaver(arg0: *Display) -> c_int;

pub fn XAddHost(arg0: *Display, arg1: *XHostAddress) -> c_int;

pub fn XAddHosts(arg0: *Display, arg1: *XHostAddress, arg2: c_int) -> c_int;

pub fn XAddToExtensionList(arg0: **struct__XExtData, arg1: *XExtData) -> c_int;

pub fn XAddToSaveSet(arg0: *Display, arg1: Window) -> c_int;

pub fn XAllocColor(arg0: *Display, arg1: Colormap, arg2: *XColor) -> c_int;

pub fn XAllocColorCells(arg0: *Display, arg1: Colormap, arg2: c_int, arg3: *c_ulong, arg4: c_uint, arg5: *c_ulong, arg6: c_uint) -> c_int;

pub fn XAllocColorPlanes(arg0: *Display, arg1: Colormap, arg2: c_int, arg3: *c_ulong, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: *c_ulong, arg9: *c_ulong, arg10: *c_ulong) -> c_int;

pub fn XAllocNamedColor(arg0: *Display, arg1: Colormap, arg2: *c_char, arg3: *XColor, arg4: *XColor) -> c_int;

pub fn XAllowEvents(arg0: *Display, arg1: c_int, arg2: Time) -> c_int;

pub fn XAutoRepeatOff(arg0: *Display) -> c_int;

pub fn XAutoRepeatOn(arg0: *Display) -> c_int;

pub fn XBell(arg0: *Display, arg1: c_int) -> c_int;

pub fn XBitmapBitOrder(arg0: *Display) -> c_int;

pub fn XBitmapPad(arg0: *Display) -> c_int;

pub fn XBitmapUnit(arg0: *Display) -> c_int;

pub fn XCellsOfScreen(arg0: *Screen) -> c_int;

pub fn XChangeActivePointerGrab(arg0: *Display, arg1: c_uint, arg2: Cursor, arg3: Time) -> c_int;

pub fn XChangeGC(arg0: *Display, arg1: GC, arg2: c_ulong, arg3: *XGCValues) -> c_int;

pub fn XChangeKeyboardControl(arg0: *Display, arg1: c_ulong, arg2: *XKeyboardControl) -> c_int;

pub fn XChangeKeyboardMapping(arg0: *Display, arg1: c_int, arg2: c_int, arg3: *KeySym, arg4: c_int) -> c_int;

pub fn XChangePointerControl(arg0: *Display, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;

pub fn XChangeProperty(arg0: *Display, arg1: Window, arg2: Atom, arg3: Atom, arg4: c_int, arg5: c_int, arg6: *c_uchar, arg7: c_int) -> c_int;

pub fn XChangeSaveSet(arg0: *Display, arg1: Window, arg2: c_int) -> c_int;

pub fn XChangeWindowAttributes(arg0: *Display, arg1: Window, arg2: c_ulong, arg3: *XSetWindowAttributes) -> c_int;

pub fn XCheckIfEvent(arg0: *Display, arg1: *XEvent, arg2: *u8, arg3: XPointer) -> c_int;

pub fn XCheckMaskEvent(arg0: *Display, arg1: c_long, arg2: *XEvent) -> c_int;

pub fn XCheckTypedEvent(arg0: *Display, arg1: c_int, arg2: *XEvent) -> c_int;

pub fn XCheckTypedWindowEvent(arg0: *Display, arg1: Window, arg2: c_int, arg3: *XEvent) -> c_int;

pub fn XCheckWindowEvent(arg0: *Display, arg1: Window, arg2: c_long, arg3: *XEvent) -> c_int;

pub fn XCirculateSubwindows(arg0: *Display, arg1: Window, arg2: c_int) -> c_int;

pub fn XCirculateSubwindowsDown(arg0: *Display, arg1: Window) -> c_int;

pub fn XCirculateSubwindowsUp(arg0: *Display, arg1: Window) -> c_int;

pub fn XClearArea(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_int) -> c_int;

pub fn XClearWindow(arg0: *Display, arg1: Window) -> c_int;

pub fn XCloseDisplay(arg0: *Display) -> c_int;

pub fn XConfigureWindow(arg0: *Display, arg1: Window, arg2: c_uint, arg3: *XWindowChanges) -> c_int;

pub fn XConnectionNumber(arg0: *Display) -> c_int;

pub fn XConvertSelection(arg0: *Display, arg1: Atom, arg2: Atom, arg3: Atom, arg4: Window, arg5: Time) -> c_int;

pub fn XCopyArea(arg0: *Display, arg1: Drawable, arg2: Drawable, arg3: GC, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: c_int, arg9: c_int) -> c_int;

pub fn XCopyGC(arg0: *Display, arg1: GC, arg2: c_ulong, arg3: GC) -> c_int;

pub fn XCopyPlane(arg0: *Display, arg1: Drawable, arg2: Drawable, arg3: GC, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: c_int, arg9: c_int, arg10: c_ulong) -> c_int;

pub fn XDefaultDepth(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDefaultDepthOfScreen(arg0: *Screen) -> c_int;

pub fn XDefaultScreen(arg0: *Display) -> c_int;

pub fn XDefineCursor(arg0: *Display, arg1: Window, arg2: Cursor) -> c_int;

pub fn XDeleteProperty(arg0: *Display, arg1: Window, arg2: Atom) -> c_int;

pub fn XDestroyWindow(arg0: *Display, arg1: Window) -> c_int;

pub fn XDestroySubwindows(arg0: *Display, arg1: Window) -> c_int;

pub fn XDoesBackingStore(arg0: *Screen) -> c_int;

pub fn XDoesSaveUnders(arg0: *Screen) -> c_int;

pub fn XDisableAccessControl(arg0: *Display) -> c_int;

pub fn XDisplayCells(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDisplayHeight(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDisplayHeightMM(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDisplayKeycodes(arg0: *Display, arg1: *c_int, arg2: *c_int) -> c_int;

pub fn XDisplayPlanes(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDisplayWidth(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDisplayWidthMM(arg0: *Display, arg1: c_int) -> c_int;

pub fn XDrawArc(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int) -> c_int;

pub fn XDrawArcs(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XArc, arg4: c_int) -> c_int;

pub fn XDrawImageString(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *c_char, arg6: c_int) -> c_int;

pub fn XDrawImageString16(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XChar2b, arg6: c_int) -> c_int;

pub fn XDrawLine(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int) -> c_int;

pub fn XDrawLines(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XPoint, arg4: c_int, arg5: c_int) -> c_int;

pub fn XDrawPoint(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int) -> c_int;

pub fn XDrawPoints(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XPoint, arg4: c_int, arg5: c_int) -> c_int;

pub fn XDrawRectangle(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint) -> c_int;

pub fn XDrawRectangles(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XRectangle, arg4: c_int) -> c_int;

pub fn XDrawSegments(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XSegment, arg4: c_int) -> c_int;

pub fn XDrawString(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *c_char, arg6: c_int) -> c_int;

pub fn XDrawString16(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XChar2b, arg6: c_int) -> c_int;

pub fn XDrawText(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XTextItem, arg6: c_int) -> c_int;

pub fn XDrawText16(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XTextItem16, arg6: c_int) -> c_int;

pub fn XEnableAccessControl(arg0: *Display) -> c_int;

pub fn XEventsQueued(arg0: *Display, arg1: c_int) -> c_int;

pub fn XFetchName(arg0: *Display, arg1: Window, arg2: **c_char) -> c_int;

pub fn XFillArc(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int) -> c_int;

pub fn XFillArcs(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XArc, arg4: c_int) -> c_int;

pub fn XFillPolygon(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XPoint, arg4: c_int, arg5: c_int, arg6: c_int) -> c_int;

pub fn XFillRectangle(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint) -> c_int;

pub fn XFillRectangles(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XRectangle, arg4: c_int) -> c_int;

pub fn XFlush(arg0: *Display) -> c_int;

pub fn XForceScreenSaver(arg0: *Display, arg1: c_int) -> c_int;

pub fn XFree(arg0: *mut c_void) -> c_int;

pub fn XFreeColormap(arg0: *Display, arg1: Colormap) -> c_int;

pub fn XFreeColors(arg0: *Display, arg1: Colormap, arg2: *c_ulong, arg3: c_int, arg4: c_ulong) -> c_int;

pub fn XFreeCursor(arg0: *Display, arg1: Cursor) -> c_int;

pub fn XFreeExtensionList(arg0: **c_char) -> c_int;

pub fn XFreeFont(arg0: *Display, arg1: *XFontStruct) -> c_int;

pub fn XFreeFontInfo(arg0: **c_char, arg1: *XFontStruct, arg2: c_int) -> c_int;

pub fn XFreeFontNames(arg0: **c_char) -> c_int;

pub fn XFreeFontPath(arg0: **c_char) -> c_int;

pub fn XFreeGC(arg0: *Display, arg1: GC) -> c_int;

pub fn XFreeModifiermap(arg0: *XModifierKeymap) -> c_int;

pub fn XFreePixmap(arg0: *Display, arg1: Pixmap) -> c_int;

pub fn XGeometry(arg0: *Display, arg1: c_int, arg2: *c_char, arg3: *c_char, arg4: c_uint, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int, arg9: *c_int, arg10: *c_int, arg11: *c_int, arg12: *c_int) -> c_int;

pub fn XGetErrorDatabaseText(arg0: *Display, arg1: *c_char, arg2: *c_char, arg3: *c_char, arg4: *c_char, arg5: c_int) -> c_int;

pub fn XGetErrorText(arg0: *Display, arg1: c_int, arg2: *c_char, arg3: c_int) -> c_int;

pub fn XGetFontProperty(arg0: *XFontStruct, arg1: Atom, arg2: *c_ulong) -> c_int;

pub fn XGetGCValues(arg0: *Display, arg1: GC, arg2: c_ulong, arg3: *XGCValues) -> c_int;

pub fn XGetGeometry(arg0: *Display, arg1: Drawable, arg2: *mut Window, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_uint, arg7: *mut c_uint, arg8: *mut c_uint) -> c_int;

pub fn XGetIconName(arg0: *Display, arg1: Window, arg2: **c_char) -> c_int;

pub fn XGetInputFocus(arg0: *Display, arg1: *Window, arg2: *c_int) -> c_int;

pub fn XGetKeyboardControl(arg0: *Display, arg1: *XKeyboardState) -> c_int;

pub fn XGetPointerControl(arg0: *Display, arg1: *c_int, arg2: *c_int, arg3: *c_int) -> c_int;

pub fn XGetPointerMapping(arg0: *Display, arg1: *c_uchar, arg2: c_int) -> c_int;

pub fn XGetScreenSaver(arg0: *Display, arg1: *c_int, arg2: *c_int, arg3: *c_int, arg4: *c_int) -> c_int;

pub fn XGetTransientForHint(arg0: *Display, arg1: Window, arg2: *Window) -> c_int;

pub fn XGetWindowProperty(arg0: *Display, arg1: Window, arg2: Atom, arg3: c_long, arg4: c_long, arg5: c_char, arg6: Atom, arg7: *mut Atom, arg8: *mut c_int, arg9: *mut c_ulong, arg10: *mut c_ulong, arg11: *mut *mut c_uchar) -> c_int;

pub fn XGetWindowAttributes(arg0: *Display, arg1: Window, arg2: *mut XWindowAttributes) -> c_int;

pub fn XGrabButton(arg0: *Display, arg1: c_uint, arg2: c_uint, arg3: Window, arg4: c_int, arg5: c_uint, arg6: c_int, arg7: c_int, arg8: Window, arg9: Cursor) -> c_int;

pub fn XGrabKey(arg0: *Display, arg1: c_int, arg2: c_uint, arg3: Window, arg4: c_int, arg5: c_int, arg6: c_int) -> c_int;

pub fn XGrabKeyboard(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_int, arg5: Time) -> c_int;

pub fn XGrabPointer(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_uint, arg4: c_int, arg5: c_int, arg6: Window, arg7: Cursor, arg8: Time) -> c_int;

pub fn XGrabServer(arg0: *Display) -> c_int;

pub fn XHeightMMOfScreen(arg0: *Screen) -> c_int;

pub fn XHeightOfScreen(arg0: *Screen) -> c_int;

pub fn XIfEvent(arg0: *Display, arg1: *XEvent, arg2: *u8, arg3: XPointer) -> c_int;

pub fn XImageByteOrder(arg0: *Display) -> c_int;

pub fn XInstallColormap(arg0: *Display, arg1: Colormap) -> c_int;

pub fn XKeysymToKeycode(arg0: *Display, arg1: KeySym) -> KeyCode;

pub fn XKillClient(arg0: *Display, arg1: XID) -> c_int;

pub fn XLookupColor(arg0: *Display, arg1: Colormap, arg2: *c_char, arg3: *XColor, arg4: *XColor) -> c_int;

pub fn XLowerWindow(arg0: *Display, arg1: Window) -> c_int;

pub fn XMapRaised(arg0: *Display, arg1: Window) -> c_int;

pub fn XMapSubwindows(arg0: *Display, arg1: Window) -> c_int;

pub fn XMapWindow(arg0: *Display, arg1: Window) -> c_int;

pub fn XMaskEvent(arg0: *Display, arg1: c_long, arg2: *XEvent) -> c_int;

pub fn XMaxCmapsOfScreen(arg0: *Screen) -> c_int;

pub fn XMinCmapsOfScreen(arg0: *Screen) -> c_int;

pub fn XMoveResizeWindow(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint) -> c_int;

pub fn XMoveWindow(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_int) -> c_int;

pub fn XNextEvent(arg0: *Display, arg1: *mut XEvent) -> c_int;

pub fn XNoOp(arg0: *Display) -> c_int;

pub fn XParseColor(arg0: *Display, arg1: Colormap, arg2: *c_char, arg3: *XColor) -> c_int;

pub fn XParseGeometry(arg0: *c_char, arg1: *c_int, arg2: *c_int, arg3: *c_uint, arg4: *c_uint) -> c_int;

pub fn XPeekEvent(arg0: *Display, arg1: *XEvent) -> c_int;

pub fn XPeekIfEvent(arg0: *Display, arg1: *XEvent, arg2: *u8, arg3: XPointer) -> c_int;

pub fn XPending(arg0: *Display) -> c_int;

pub fn XPlanesOfScreen(arg0: *Screen) -> c_int;

pub fn XProtocolRevision(arg0: *Display) -> c_int;

pub fn XProtocolVersion(arg0: *Display) -> c_int;

pub fn XPutBackEvent(arg0: *Display, arg1: *XEvent) -> c_int;

pub fn XPutImage(arg0: *Display, arg1: Drawable, arg2: GC, arg3: *XImage, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_uint, arg9: c_uint) -> c_int;

pub fn XQLength(arg0: *Display) -> c_int;

pub fn XQueryBestCursor(arg0: *Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: *c_uint, arg5: *c_uint) -> c_int;

pub fn XQueryBestSize(arg0: *Display, arg1: c_int, arg2: Drawable, arg3: c_uint, arg4: c_uint, arg5: *c_uint, arg6: *c_uint) -> c_int;

pub fn XQueryBestStipple(arg0: *Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: *c_uint, arg5: *c_uint) -> c_int;

pub fn XQueryBestTile(arg0: *Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: *c_uint, arg5: *c_uint) -> c_int;

pub fn XQueryColor(arg0: *Display, arg1: Colormap, arg2: *XColor) -> c_int;

pub fn XQueryColors(arg0: *Display, arg1: Colormap, arg2: *XColor, arg3: c_int) -> c_int;

pub fn XQueryExtension(arg0: *Display, arg1: *c_char, arg2: *c_int, arg3: *c_int, arg4: *c_int) -> c_int;

pub fn XQueryKeymap(arg0: *Display, arg1: *c_char) -> c_int;

pub fn XQueryPointer(arg0: *Display, arg1: Window, arg2: *Window, arg3: *Window, arg4: *c_int, arg5: *c_int, arg6: *c_int, arg7: *c_int, arg8: *c_uint) -> c_int;

pub fn XQueryTextExtents(arg0: *Display, arg1: XID, arg2: *c_char, arg3: c_int, arg4: *c_int, arg5: *c_int, arg6: *c_int, arg7: *XCharStruct) -> c_int;

pub fn XQueryTextExtents16(arg0: *Display, arg1: XID, arg2: *XChar2b, arg3: c_int, arg4: *c_int, arg5: *c_int, arg6: *c_int, arg7: *XCharStruct) -> c_int;

pub fn XQueryTree(arg0: *Display, arg1: Window, arg2: *mut Window, arg3: *mut Window, arg4: *mut *mut Window, arg5: *mut c_uint) -> c_int;

pub fn XRaiseWindow(arg0: *Display, arg1: Window) -> c_int;

pub fn XReadBitmapFile(arg0: *Display, arg1: Drawable, arg2: *c_char, arg3: *c_uint, arg4: *c_uint, arg5: *Pixmap, arg6: *c_int, arg7: *c_int) -> c_int;

pub fn XReadBitmapFileData(arg0: *c_char, arg1: *c_uint, arg2: *c_uint, arg3: **c_uchar, arg4: *c_int, arg5: *c_int) -> c_int;

pub fn XRebindKeysym(arg0: *Display, arg1: KeySym, arg2: *KeySym, arg3: c_int, arg4: *c_uchar, arg5: c_int) -> c_int;

pub fn XRecolorCursor(arg0: *Display, arg1: Cursor, arg2: *XColor, arg3: *XColor) -> c_int;

pub fn XRefreshKeyboardMapping(arg0: *XMappingEvent) -> c_int;

pub fn XRemoveFromSaveSet(arg0: *Display, arg1: Window) -> c_int;

pub fn XRemoveHost(arg0: *Display, arg1: *XHostAddress) -> c_int;

pub fn XRemoveHosts(arg0: *Display, arg1: *XHostAddress, arg2: c_int) -> c_int;

pub fn XReparentWindow(arg0: *Display, arg1: Window, arg2: Window, arg3: c_int, arg4: c_int) -> c_int;

pub fn XResetScreenSaver(arg0: *Display) -> c_int;

pub fn XResizeWindow(arg0: *Display, arg1: Window, arg2: c_uint, arg3: c_uint) -> c_int;

pub fn XRestackWindows(arg0: *Display, arg1: *Window, arg2: c_int) -> c_int;

pub fn XRotateBuffers(arg0: *Display, arg1: c_int) -> c_int;

pub fn XRotateWindowProperties(arg0: *Display, arg1: Window, arg2: *Atom, arg3: c_int, arg4: c_int) -> c_int;

pub fn XScreenCount(arg0: *Display) -> c_int;

pub fn XSelectInput(arg0: *Display, arg1: Window, arg2: c_long) -> c_int;

pub fn XSendEvent(arg0: *Display, arg1: Window, arg2: c_int, arg3: c_long, arg4: *XEvent) -> c_int;

pub fn XSetAccessControl(arg0: *Display, arg1: c_int) -> c_int;

pub fn XSetArcMode(arg0: *Display, arg1: GC, arg2: c_int) -> c_int;

pub fn XSetBackground(arg0: *Display, arg1: GC, arg2: c_ulong) -> c_int;

pub fn XSetClipMask(arg0: *Display, arg1: GC, arg2: Pixmap) -> c_int;

pub fn XSetClipOrigin(arg0: *Display, arg1: GC, arg2: c_int, arg3: c_int) -> c_int;

pub fn XSetClipRectangles(arg0: *Display, arg1: GC, arg2: c_int, arg3: c_int, arg4: *XRectangle, arg5: c_int, arg6: c_int) -> c_int;

pub fn XSetCloseDownMode(arg0: *Display, arg1: c_int) -> c_int;

pub fn XSetCommand(arg0: *Display, arg1: Window, arg2: **c_char, arg3: c_int) -> c_int;

pub fn XSetDashes(arg0: *Display, arg1: GC, arg2: c_int, arg3: *c_char, arg4: c_int) -> c_int;

pub fn XSetFillRule(arg0: *Display, arg1: GC, arg2: c_int) -> c_int;

pub fn XSetFillStyle(arg0: *Display, arg1: GC, arg2: c_int) -> c_int;

pub fn XSetFont(arg0: *Display, arg1: GC, arg2: Font) -> c_int;

pub fn XSetFontPath(arg0: *Display, arg1: **c_char, arg2: c_int) -> c_int;

pub fn XSetForeground(arg0: *Display, arg1: GC, arg2: c_ulong) -> c_int;

pub fn XSetFunction(arg0: *Display, arg1: GC, arg2: c_int) -> c_int;

pub fn XSetGraphicsExposures(arg0: *Display, arg1: GC, arg2: c_int) -> c_int;

pub fn XSetIconName(arg0: *Display, arg1: Window, arg2: *c_char) -> c_int;

pub fn XSetInputFocus(arg0: *Display, arg1: Window, arg2: c_int, arg3: Time) -> c_int;

pub fn XSetLineAttributes(arg0: *Display, arg1: GC, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;

pub fn XSetModifierMapping(arg0: *Display, arg1: *XModifierKeymap) -> c_int;

pub fn XSetPlaneMask(arg0: *Display, arg1: GC, arg2: c_ulong) -> c_int;

pub fn XSetPointerMapping(arg0: *Display, arg1: *c_uchar, arg2: c_int) -> c_int;

pub fn XSetScreenSaver(arg0: *Display, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> c_int;

pub fn XSetSelectionOwner(arg0: *Display, arg1: Atom, arg2: Window, arg3: Time) -> c_int;

pub fn XSetState(arg0: *Display, arg1: GC, arg2: c_ulong, arg3: c_ulong, arg4: c_int, arg5: c_ulong) -> c_int;

pub fn XSetStipple(arg0: *Display, arg1: GC, arg2: Pixmap) -> c_int;

pub fn XSetSubwindowMode(arg0: *Display, arg1: GC, arg2: c_int) -> c_int;

pub fn XSetTSOrigin(arg0: *Display, arg1: GC, arg2: c_int, arg3: c_int) -> c_int;

pub fn XSetTile(arg0: *Display, arg1: GC, arg2: Pixmap) -> c_int;

pub fn XSetWindowBackground(arg0: *Display, arg1: Window, arg2: c_ulong) -> c_int;

pub fn XSetWindowBackgroundPixmap(arg0: *Display, arg1: Window, arg2: Pixmap) -> c_int;

pub fn XSetWindowBorder(arg0: *Display, arg1: Window, arg2: c_ulong) -> c_int;

pub fn XSetWindowBorderPixmap(arg0: *Display, arg1: Window, arg2: Pixmap) -> c_int;

pub fn XSetWindowBorderWidth(arg0: *Display, arg1: Window, arg2: c_uint) -> c_int;

pub fn XSetWindowColormap(arg0: *Display, arg1: Window, arg2: Colormap) -> c_int;

pub fn XStoreBuffer(arg0: *Display, arg1: *c_char, arg2: c_int, arg3: c_int) -> c_int;

pub fn XStoreBytes(arg0: *Display, arg1: *c_char, arg2: c_int) -> c_int;

pub fn XStoreColor(arg0: *Display, arg1: Colormap, arg2: *XColor) -> c_int;

pub fn XStoreColors(arg0: *Display, arg1: Colormap, arg2: *XColor, arg3: c_int) -> c_int;

pub fn XStoreName(arg0: *Display, arg1: Window, arg2: *c_char) -> c_int;

pub fn XStoreNamedColor(arg0: *Display, arg1: Colormap, arg2: *c_char, arg3: c_ulong, arg4: c_int) -> c_int;

pub fn XSync(arg0: *Display, arg1: c_int) -> c_int;

pub fn XTextExtents(arg0: *XFontStruct, arg1: *c_char, arg2: c_int, arg3: *c_int, arg4: *c_int, arg5: *c_int, arg6: *XCharStruct) -> c_int;

pub fn XTextExtents16(arg0: *XFontStruct, arg1: *XChar2b, arg2: c_int, arg3: *c_int, arg4: *c_int, arg5: *c_int, arg6: *XCharStruct) -> c_int;

pub fn XTextWidth(arg0: *XFontStruct, arg1: *c_char, arg2: c_int) -> c_int;

pub fn XTextWidth16(arg0: *XFontStruct, arg1: *XChar2b, arg2: c_int) -> c_int;

pub fn XTranslateCoordinates(arg0: *Display, arg1: Window, arg2: Window, arg3: c_int, arg4: c_int, arg5: *c_int, arg6: *c_int, arg7: *Window) -> c_int;

pub fn XUndefineCursor(arg0: *Display, arg1: Window) -> c_int;

pub fn XUngrabButton(arg0: *Display, arg1: c_uint, arg2: c_uint, arg3: Window) -> c_int;

pub fn XUngrabKey(arg0: *Display, arg1: c_int, arg2: c_uint, arg3: Window) -> c_int;

pub fn XUngrabKeyboard(arg0: *Display, arg1: Time) -> c_int;

pub fn XUngrabPointer(arg0: *Display, arg1: Time) -> c_int;

pub fn XUngrabServer(arg0: *Display) -> c_int;

pub fn XUninstallColormap(arg0: *Display, arg1: Colormap) -> c_int;

pub fn XUnloadFont(arg0: *Display, arg1: Font) -> c_int;

pub fn XUnmapSubwindows(arg0: *Display, arg1: Window) -> c_int;

pub fn XUnmapWindow(arg0: *Display, arg1: Window) -> c_int;

pub fn XVendorRelease(arg0: *Display) -> c_int;

pub fn XWarpPointer(arg0: *Display, arg1: Window, arg2: Window, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int) -> c_int;

pub fn XWidthMMOfScreen(arg0: *Screen) -> c_int;

pub fn XWidthOfScreen(arg0: *Screen) -> c_int;

pub fn XWindowEvent(arg0: *Display, arg1: Window, arg2: c_long, arg3: *XEvent) -> c_int;

pub fn XWriteBitmapFile(arg0: *Display, arg1: *c_char, arg2: Pixmap, arg3: c_uint, arg4: c_uint, arg5: c_int, arg6: c_int) -> c_int;

pub fn XSupportsLocale() -> c_int;

pub fn XSetLocaleModifiers(arg0: *c_char) -> *c_char;

pub fn XOpenOM(arg0: *Display, arg1: *struct__XrmHashBucketRec, arg2: *c_char, arg3: *c_char) -> XOM;

pub fn XCloseOM(arg0: XOM) -> c_int;

pub fn XSetOMValues(arg0: XOM/* FIXME: variadic function */) -> *c_char;

pub fn XGetOMValues(arg0: XOM/* FIXME: variadic function */) -> *c_char;

pub fn XDisplayOfOM(arg0: XOM) -> *Display;

pub fn XLocaleOfOM(arg0: XOM) -> *c_char;

pub fn XCreateOC(arg0: XOM/* FIXME: variadic function */) -> XOC;

pub fn XDestroyOC(arg0: XOC);

pub fn XOMOfOC(arg0: XOC) -> XOM;

pub fn XSetOCValues(arg0: XOC/* FIXME: variadic function */) -> *c_char;

pub fn XGetOCValues(arg0: XOC/* FIXME: variadic function */) -> *c_char;

pub fn XCreateFontSet(arg0: *Display, arg1: *c_char, arg2: ***c_char, arg3: *c_int, arg4: **c_char) -> XFontSet;

pub fn XFreeFontSet(arg0: *Display, arg1: XFontSet);

pub fn XFontsOfFontSet(arg0: XFontSet, arg1: ***XFontStruct, arg2: ***c_char) -> c_int;

pub fn XBaseFontNameListOfFontSet(arg0: XFontSet) -> *c_char;

pub fn XLocaleOfFontSet(arg0: XFontSet) -> *c_char;

pub fn XContextDependentDrawing(arg0: XFontSet) -> c_int;

pub fn XDirectionalDependentDrawing(arg0: XFontSet) -> c_int;

pub fn XContextualDrawing(arg0: XFontSet) -> c_int;

pub fn XExtentsOfFontSet(arg0: XFontSet) -> *XFontSetExtents;

pub fn XmbTextEscapement(arg0: XFontSet, arg1: *c_char, arg2: c_int) -> c_int;

pub fn XwcTextEscapement(arg0: XFontSet, arg1: *wchar_t, arg2: c_int) -> c_int;

pub fn Xutf8TextEscapement(arg0: XFontSet, arg1: *c_char, arg2: c_int) -> c_int;

pub fn XmbTextExtents(arg0: XFontSet, arg1: *c_char, arg2: c_int, arg3: *XRectangle, arg4: *XRectangle) -> c_int;

pub fn XwcTextExtents(arg0: XFontSet, arg1: *wchar_t, arg2: c_int, arg3: *XRectangle, arg4: *XRectangle) -> c_int;

pub fn Xutf8TextExtents(arg0: XFontSet, arg1: *c_char, arg2: c_int, arg3: *XRectangle, arg4: *XRectangle) -> c_int;

pub fn XmbTextPerCharExtents(arg0: XFontSet, arg1: *c_char, arg2: c_int, arg3: *XRectangle, arg4: *XRectangle, arg5: c_int, arg6: *c_int, arg7: *XRectangle, arg8: *XRectangle) -> c_int;

pub fn XwcTextPerCharExtents(arg0: XFontSet, arg1: *wchar_t, arg2: c_int, arg3: *XRectangle, arg4: *XRectangle, arg5: c_int, arg6: *c_int, arg7: *XRectangle, arg8: *XRectangle) -> c_int;

pub fn Xutf8TextPerCharExtents(arg0: XFontSet, arg1: *c_char, arg2: c_int, arg3: *XRectangle, arg4: *XRectangle, arg5: c_int, arg6: *c_int, arg7: *XRectangle, arg8: *XRectangle) -> c_int;

pub fn XmbDrawText(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XmbTextItem, arg6: c_int);

pub fn XwcDrawText(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XwcTextItem, arg6: c_int);

pub fn Xutf8DrawText(arg0: *Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *XmbTextItem, arg6: c_int);

pub fn XmbDrawString(arg0: *Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *c_char, arg7: c_int);

pub fn XwcDrawString(arg0: *Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *wchar_t, arg7: c_int);

pub fn Xutf8DrawString(arg0: *Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *c_char, arg7: c_int);

pub fn XmbDrawImageString(arg0: *Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *c_char, arg7: c_int);

pub fn XwcDrawImageString(arg0: *Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *wchar_t, arg7: c_int);

pub fn Xutf8DrawImageString(arg0: *Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *c_char, arg7: c_int);

pub fn XOpenIM(arg0: *Display, arg1: *struct__XrmHashBucketRec, arg2: *c_char, arg3: *c_char) -> XIM;

pub fn XCloseIM(arg0: XIM) -> c_int;

pub fn XGetIMValues(arg0: XIM/* FIXME: variadic function */) -> *c_char;

pub fn XSetIMValues(arg0: XIM/* FIXME: variadic function */) -> *c_char;

pub fn XDisplayOfIM(arg0: XIM) -> *Display;

pub fn XLocaleOfIM(arg0: XIM) -> *c_char;

pub fn XCreateIC(arg0: XIM/* FIXME: variadic function */) -> XIC;

pub fn XDestroyIC(arg0: XIC);

pub fn XSetICFocus(arg0: XIC);

pub fn XUnsetICFocus(arg0: XIC);

pub fn XwcResetIC(arg0: XIC) -> *wchar_t;

pub fn XmbResetIC(arg0: XIC) -> *c_char;

pub fn Xutf8ResetIC(arg0: XIC) -> *c_char;

pub fn XSetICValues(arg0: XIC/* FIXME: variadic function */) -> *c_char;

pub fn XGetICValues(arg0: XIC/* FIXME: variadic function */) -> *c_char;

pub fn XIMOfIC(arg0: XIC) -> XIM;

pub fn XFilterEvent(arg0: *XEvent, arg1: Window) -> c_int;

pub fn XmbLookupString(arg0: XIC, arg1: *XKeyPressedEvent, arg2: *c_char, arg3: c_int, arg4: *KeySym, arg5: *c_int) -> c_int;

pub fn XwcLookupString(arg0: XIC, arg1: *XKeyPressedEvent, arg2: *wchar_t, arg3: c_int, arg4: *KeySym, arg5: *c_int) -> c_int;

pub fn Xutf8LookupString(arg0: XIC, arg1: *XKeyPressedEvent, arg2: *c_char, arg3: c_int, arg4: *KeySym, arg5: *c_int) -> c_int;

pub fn XVaCreateNestedList(arg0: c_int/* FIXME: variadic function */) -> XVaNestedList;

pub fn XRegisterIMInstantiateCallback(arg0: *Display, arg1: *struct__XrmHashBucketRec, arg2: *c_char, arg3: *c_char, arg4: XIDProc, arg5: XPointer) -> c_int;

pub fn XUnregisterIMInstantiateCallback(arg0: *Display, arg1: *struct__XrmHashBucketRec, arg2: *c_char, arg3: *c_char, arg4: XIDProc, arg5: XPointer) -> c_int;

pub fn XInternalConnectionNumbers(arg0: *Display, arg1: **c_int, arg2: *c_int) -> c_int;

pub fn XProcessInternalConnection(arg0: *Display, arg1: c_int);

pub fn XAddConnectionWatch(arg0: *Display, arg1: XConnectionWatchProc, arg2: XPointer) -> c_int;

pub fn XRemoveConnectionWatch(arg0: *Display, arg1: XConnectionWatchProc, arg2: XPointer);

pub fn XSetAuthorization(arg0: *c_char, arg1: c_int, arg2: *c_char, arg3: c_int);

pub fn _Xmbtowc(arg0: *wchar_t, arg1: *c_char, arg2: c_int) -> c_int;

pub fn _Xwctomb(arg0: *c_char, arg1: wchar_t) -> c_int;

pub fn XGetEventData(arg0: *Display, arg1: *XGenericEventCookie) -> c_int;

pub fn XFreeEventData(arg0: *Display, arg1: *XGenericEventCookie);

}
