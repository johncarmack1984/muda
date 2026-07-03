// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Linux fallback used when the `gtk` feature is disabled.
//!
//! Menu structures can be built and inspected, but nothing renders:
//! there is no toolkit to attach them to on this configuration.

mod icon;

pub(crate) use icon::PlatformIcon;

use crate::{
    accelerator::KeyAccelerator,
    icon::{Icon, NativeIcon},
    items::PredefinedMenuItemType,
    util::{AddOp, Counter},
    IsMenuItem, MenuId, MenuItemKind, MenuItemType,
};
use std::{cell::RefCell, rc::Rc};

static COUNTER: Counter = Counter::new();

pub struct Menu {
    id: MenuId,
    children: Vec<Rc<RefCell<MenuChild>>>,
}

impl Menu {
    pub fn new(id: Option<MenuId>) -> Self {
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            children: Vec::new(),
        }
    }

    pub fn id(&self) -> &MenuId {
        &self.id
    }

    pub fn add_menu_item(&mut self, item: &dyn IsMenuItem, op: AddOp) -> crate::Result<()> {
        let child = item.child();
        match op {
            AddOp::Append => self.children.push(child),
            AddOp::Insert(position) => self.children.insert(position, child),
        }
        Ok(())
    }

    pub fn remove(&mut self, item: &dyn IsMenuItem) -> crate::Result<()> {
        let id = item.id().clone();
        self.children.retain(|c| c.borrow().id != id);
        Ok(())
    }

    pub fn items(&self) -> Vec<MenuItemKind> {
        self.children
            .iter()
            .map(|c| c.borrow().kind(c.clone()))
            .collect()
    }
}

pub struct MenuChild {
    item_type: MenuItemType,
    text: String,
    enabled: bool,
    id: MenuId,

    accelerator: Option<KeyAccelerator>,

    predefined_item_type: Option<PredefinedMenuItemType>,

    checked: bool,

    icon: Option<Icon>,

    pub children: Option<Vec<Rc<RefCell<MenuChild>>>>,
}

/// Constructors
impl MenuChild {
    pub fn new(
        text: &str,
        enabled: bool,
        key_accelerator: Option<KeyAccelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {
            text: text.to_string(),
            enabled,
            accelerator: key_accelerator,
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            item_type: MenuItemType::MenuItem,
            predefined_item_type: None,
            checked: false,
            icon: None,
            children: None,
        }
    }

    pub fn new_submenu(text: &str, enabled: bool, id: Option<MenuId>) -> Self {
        Self {
            text: text.to_string(),
            enabled,
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            children: Some(Vec::new()),
            item_type: MenuItemType::Submenu,
            accelerator: None,
            predefined_item_type: None,
            checked: false,
            icon: None,
        }
    }

    pub(crate) fn new_predefined(item_type: PredefinedMenuItemType, text: Option<String>) -> Self {
        Self {
            text: text.unwrap_or_else(|| item_type.text().to_string()),
            enabled: true,
            accelerator: item_type.accelerator().map(Into::into),
            id: MenuId(COUNTER.next().to_string()),
            item_type: MenuItemType::Predefined,
            predefined_item_type: Some(item_type),
            checked: false,
            icon: None,
            children: None,
        }
    }

    pub fn new_check(
        text: &str,
        enabled: bool,
        checked: bool,
        key_accelerator: Option<KeyAccelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {
            text: text.to_string(),
            enabled,
            checked,
            accelerator: key_accelerator,
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            item_type: MenuItemType::Check,
            predefined_item_type: None,
            icon: None,
            children: None,
        }
    }

    pub fn new_icon(
        text: &str,
        enabled: bool,
        icon: Option<Icon>,
        key_accelerator: Option<KeyAccelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {
            text: text.to_string(),
            enabled,
            icon,
            accelerator: key_accelerator,
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            item_type: MenuItemType::Icon,
            predefined_item_type: None,
            checked: false,
            children: None,
        }
    }

    pub fn new_native_icon(
        text: &str,
        enabled: bool,
        _native_icon: Option<NativeIcon>,
        key_accelerator: Option<KeyAccelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {
            text: text.to_string(),
            enabled,
            accelerator: key_accelerator,
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            item_type: MenuItemType::Icon,
            predefined_item_type: None,
            checked: false,
            icon: None,
            children: None,
        }
    }
}

/// Shared methods
impl MenuChild {
    pub(crate) fn item_type(&self) -> MenuItemType {
        self.item_type
    }

    pub fn id(&self) -> &MenuId {
        &self.id
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_key_accelerator(&mut self, accelerator: Option<KeyAccelerator>) -> crate::Result<()> {
        self.accelerator = accelerator;
        Ok(())
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn set_icon(&mut self, icon: Option<Icon>) {
        self.icon = icon;
    }
}

/// Submenu methods
impl MenuChild {
    pub fn add_menu_item(&mut self, item: &dyn IsMenuItem, op: AddOp) -> crate::Result<()> {
        let child = item.child();
        match op {
            AddOp::Append => self.children.as_mut().unwrap().push(child),
            AddOp::Insert(position) => self.children.as_mut().unwrap().insert(position, child),
        }
        Ok(())
    }

    pub fn remove(&mut self, item: &dyn IsMenuItem) -> crate::Result<()> {
        let id = item.id().clone();
        self.children
            .as_mut()
            .unwrap()
            .retain(|c| c.borrow().id != id);
        Ok(())
    }

    pub fn items(&self) -> Vec<MenuItemKind> {
        self.children
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.borrow().kind(c.clone()))
            .collect()
    }
}
