#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Clone)]
pub struct XPLMWindow {
    inner: XPLMWindowID,
}

impl XPLMWindow {
    pub fn get_inner(&self) -> XPLMWindowID {
        self.inner
    }
}

impl Drop for XPLMWindow {
    fn drop(&mut self) {
        unsafe {
            XPLMDestroyWindow(self.inner);
        }
    }
}

unsafe impl Send for XPLMWindow {}
unsafe impl Sync for XPLMWindow {}

use enum_primitive_derive::Primitive;
#[derive(Debug, Copy, Clone, PartialEq, Primitive)]
#[repr(i32)]
pub enum WindowDecoration {
    None = xplm_WindowDecorationNone,
    RoundRectangle = xplm_WindowDecorationRoundRectangle,
    SelfDecorated = xplm_WindowDecorationSelfDecorated,
    SelfDecoratedResizeable = xplm_WindowDecorationSelfDecoratedResizable,
}

#[derive(Debug, Copy, Clone, PartialEq, Primitive)]
#[repr(i32)]
pub enum WindowLayer {
    FlightOverlay = xplm_WindowLayerFlightOverlay,
    FloatingWindows = xplm_WindowLayerFloatingWindows,
    Modal = xplm_WindowLayerModal,
    GrowlNotifications = xplm_WindowLayerGrowlNotifications,
}

pub struct XPLMWindowBuilder {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    visible: i32,
    drawWindowFunc: XPLMDrawWindow_f,
    handleMouseClickFunc: XPLMHandleMouseClick_f,
    handleKeyFunc: XPLMHandleKey_f,
    handleCursorFunc: XPLMHandleCursor_f,
    handleMouseWheelFunc: XPLMHandleMouseWheel_f,
    refcon: *mut ::std::os::raw::c_void,
    decorateAsFloatingWindow: WindowDecoration,
    layer: WindowLayer,
    handleRightClickFunc: XPLMHandleMouseClick_f,
}

impl XPLMWindowBuilder {
    pub fn new() -> Self {
        XPLMWindowBuilder {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            visible: 1,
            drawWindowFunc: None,
            handleMouseClickFunc: None,
            handleKeyFunc: None,
            handleCursorFunc: None,
            handleMouseWheelFunc: None,
            refcon: std::ptr::null_mut(),
            decorateAsFloatingWindow: WindowDecoration::RoundRectangle,
            layer: WindowLayer::FloatingWindows,
            handleRightClickFunc: None,
        }
    }
    pub fn left<'a>(&'a mut self, left: i32) -> &'a mut Self {
        self.left = left;
        self
    }
    pub fn top<'a>(&'a mut self, top: i32) -> &'a mut Self {
        self.top = top;
        self
    }
    pub fn right<'a>(&'a mut self, right: i32) -> &'a mut Self {
        self.right = right;
        self
    }
    pub fn bottom<'a>(&'a mut self, bottom: i32) -> &'a mut Self {
        self.bottom = bottom;
        self
    }
    pub fn visible<'a>(&'a mut self, visible: i32) -> &'a mut Self {
        self.visible = visible;
        self
    }
    pub fn drawWindowFunc<'a>(&'a mut self, drawWindowFunc: XPLMDrawWindow_f) -> &'a mut Self {
        self.drawWindowFunc = drawWindowFunc;
        self
    }

    pub fn handleMouseClickFunc<'a>(
        &'a mut self,
        handleMouseClickFunc: XPLMHandleMouseClick_f,
    ) -> &'a mut Self {
        self.handleMouseClickFunc = handleMouseClickFunc;
        self
    }
    pub fn handleKeyFunc<'a>(&'a mut self, handleKeyFunc: XPLMHandleKey_f) -> &'a mut Self {
        self.handleKeyFunc = handleKeyFunc;
        self
    }
    pub fn handleCursorFunc<'a>(
        &'a mut self,
        handleCursorFunc: XPLMHandleCursor_f,
    ) -> &'a mut Self {
        self.handleCursorFunc = handleCursorFunc;
        self
    }

    pub fn handleMouseWheelFunc<'a>(
        &'a mut self,
        handleMouseWheelFunc: XPLMHandleMouseWheel_f,
    ) -> &'a mut Self {
        self.handleMouseWheelFunc = handleMouseWheelFunc;
        self
    }

    pub fn refcon<'a>(&'a mut self, refcon: *mut ::std::os::raw::c_void) -> &'a mut Self {
        self.refcon = refcon;
        self
    }

    pub fn decorateAsFloatingWindow<'a>(
        &'a mut self,
        decorateAsFloatingWindow: WindowDecoration,
    ) -> &'a mut Self {
        self.decorateAsFloatingWindow = decorateAsFloatingWindow;
        self
    }

    pub fn layer<'a>(&'a mut self, layer: WindowLayer) -> &'a mut Self {
        self.layer = layer;
        self
    }

    pub fn handleRightClickFunc<'a>(
        &'a mut self,
        handleRightClickFunc: XPLMHandleMouseClick_f,
    ) -> &'a mut Self {
        self.handleRightClickFunc = handleRightClickFunc;
        self
    }
    pub fn build(&self) -> XPLMWindow {
        use num_traits::ToPrimitive;
        let params = &mut XPLMCreateWindow_t {
            structSize: std::mem::size_of::<XPLMCreateWindow_t>() as i32,
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            visible: self.visible,
            drawWindowFunc: self.drawWindowFunc,
            handleMouseClickFunc: self.handleMouseClickFunc,
            handleKeyFunc: self.handleKeyFunc,
            handleCursorFunc: self.handleCursorFunc,
            handleMouseWheelFunc: self.handleMouseWheelFunc,
            refcon: self.refcon,
            decorateAsFloatingWindow: self.decorateAsFloatingWindow.to_i32().unwrap(),
            layer: self.layer.to_i32().unwrap(),
            handleRightClickFunc: self.handleRightClickFunc,
        };
        let window: XPLMWindowID;
        unsafe {
            window = XPLMCreateWindowEx(params);
        }
        XPLMWindow { inner: window }
    }
}

// TODO enum from XPLMWindowPositioningMode
