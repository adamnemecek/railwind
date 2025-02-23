mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COLORS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
}

#[derive(Debug)]
pub enum Svg<'a> {
    Fill(Fill<'a>),
    Stroke(Stroke<'a>),
    StrokeWidth(StrokeWidth),
}

impl<'a> Svg<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let args = if let Ok(str) = get_args(value) {
            str
        } else {
            return Ok(None);
        };

        let class_name = get_class_name(value);

        let svg = match class_name {
            "fill" => Self::Fill(Fill(args)),
            "stroke" => {
                if let Some(stroke) = StrokeWidth::new(args) {
                    Self::StrokeWidth(stroke)
                } else {
                    Self::Stroke(Stroke::new(args))
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(svg))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Fill(s) => s.to_decl(),
            Self::Stroke(s) => s.to_decl(),
            Self::StrokeWidth(s) => Ok(s.to_decl()),
        }
    }
}
