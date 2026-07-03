// Copyright 2014-2021 The winit contributors
// Copyright 2021-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use crate::icon::BadIcon;

/// An icon used for the window titlebar, taskbar, etc.
#[derive(Debug, Clone)]
pub struct PlatformIcon {
    #[allow(unused)]
    raw: Vec<u8>,
    #[allow(unused)]
    width: i32,
    #[allow(unused)]
    height: i32,
}

impl PlatformIcon {
    /// Creates an `Icon` from 32bpp RGBA data.
    ///
    /// The length of `rgba` must be divisible by 4, and `width * height` must equal
    /// `rgba.len() / 4`. Otherwise, this will return a `BadIcon` error.
    pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
        Ok(Self {
            raw: rgba,
            width: width as i32,
            height: height as i32,
        })
    }
}
