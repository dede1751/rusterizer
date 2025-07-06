// Input abstraction to avoid having to use raylib outside the engine crate
use raylib::prelude::*;

use crate::primitives::Float2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Null = 0,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    Space = 32,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    KeyboardMenu = 348,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    Grave = 96,
}

impl Key {
    fn to_rl(self) -> KeyboardKey {
        // Safety: Key values are guaranteed to be the same as KeyboardKey values
        unsafe { std::mem::transmute(self as u32) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseKey {
    Left = 0,
    Right = 1,
    Middle = 2,
}

impl MouseKey {
    fn to_rl(self) -> MouseButton {
        // Safety: MouseKey values are guaranteed to be the same as MouseButton values
        unsafe { std::mem::transmute(self as u32) }
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    rl: &'a mut RaylibHandle,
}

impl<'a> Input<'a> {
    pub fn new(rl: &'a mut RaylibHandle) -> Self {
        Input { rl }
    }

    pub fn lock_cursor(&mut self) {
        self.rl.hide_cursor();
        self.rl.disable_cursor();
    }

    pub fn unlock_cursor(&mut self) {
        self.rl.enable_cursor();
        self.rl.show_cursor();
    }

    pub fn is_key_down_this_frame(&self, key: Key) -> bool {
        self.rl.is_key_pressed(key.to_rl())
    }

    pub fn is_key_held(&self, key: Key) -> bool {
        self.rl.is_key_down(key.to_rl())
    }

    pub fn is_mouse_down_this_frame(&self, button: MouseKey) -> bool {
        self.rl.is_mouse_button_pressed(button.to_rl())
    }

    pub fn is_mouse_held(&self, button: MouseKey) -> bool {
        self.rl.is_mouse_button_down(button.to_rl())
    }

    pub fn get_mouse_position(&self) -> Float2 {
        let v = self.rl.get_mouse_position();
        Float2::new(v.x, v.y)
    }

    pub fn get_mouse_delta(&self) -> Float2 {
        let v = self.rl.get_mouse_delta();
        Float2::new(v.x, v.y)
    }
}
