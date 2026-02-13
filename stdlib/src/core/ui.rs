//! Zenith UI Core Module
//!
//! This module defines the base traits and widgets for the Zenith UI framework.

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Widget {
    Container {
        children: Vec<Widget>,
        style: Style,
    },
    Text {
        text: String,
        style: TextStyle,
    },
    Button {
        label: String,
        on_click: String,
        style: ButtonStyle,
    },
    Image {
        src: String,
        width: Option<f32>,
        height: Option<f32>,
    },
    Stack {
        children: Vec<Widget>,
        alignment: Alignment,
    },
    Row {
        children: Vec<Widget>,
        main_axis_alignment: MainAxisAlignment,
        cross_axis_alignment: CrossAxisAlignment,
    },
    Column {
        children: Vec<Widget>,
        main_axis_alignment: MainAxisAlignment,
        cross_axis_alignment: CrossAxisAlignment,
    },
    Padding {
        child: Box<Widget>,
        padding: PaddingValues,
    },
    Center {
        child: Box<Widget>,
    },
    Spacer {
        flex: f32,
    },
    Expanded {
        child: Box<Widget>,
        flex: f32,
    },
}

use crate::core::color::Color;

#[derive(Serialize, Debug, Clone)]
pub struct Style {
    pub width: Dimension,
    pub height: Dimension,
    pub padding: f32,
    pub margin: f32,
    pub background_color: Color,
    pub border_radius: f32,
}

#[derive(Serialize, Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
    pub font_weight: f32,
}

#[derive(Serialize, Debug, Clone)]
pub struct ButtonStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub border_radius: f32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Dimension {
    Auto,
    Pixels(f32),
    Percent(f32),
}

#[derive(Serialize, Debug, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
    SpaceBetween,
}

#[derive(Serialize, Debug, Clone)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Serialize, Debug, Clone)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Serialize, Debug, Clone)]
pub struct PaddingValues {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl PaddingValues {
    pub fn all(value: f32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }
}

pub trait Renderer {
    fn render(&mut self, widget: &Widget);
}
