use std::cmp::{max, min};
use crate::render::Render;
use std::io::Write;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct HitPoints {
    current: isize,
    max: isize,
    temporary: isize
}

impl HitPoints {
    pub fn increase_max(&mut self, increment: isize) {
        self.max += increment;
    }

    pub fn get_modified_max(&self) -> isize {
        self.max + self.temporary
    }

    pub fn add_current(&mut self, delta: isize) -> bool {
        let result = self.current + delta;
        let result = max(0, result);
        let result = min(result, self.get_modified_max());
        self.current = result;
        self.is_conscious()
    }

    pub fn add_temporary(&mut self, delta: isize) {
        self.temporary += delta;
        self.add_current(delta);
    }

    pub fn reset_temporary(&mut self) {
        self.temporary = 0;
        let current = min(self.current, self.get_modified_max());
        self.current = current;
    }

    pub fn reset(&mut self) {
        self.reset_temporary();
        self.current = self.get_modified_max();
    }

    pub fn is_conscious(&self) -> bool {
        self.current > 0
    }
}
use anyhow::Result;
impl Render for HitPoints {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        out.write_fmt(format_args!("{}Hit Points:\n", Render::tab(indent)))?;
        if self.temporary == 0 {
            out.write_fmt(format_args!("{} {} / {}\n", Render::tab(indent + 1), self.current, self.get_modified_max()))?;
        } else {
            let diff_char = if self.temporary < 0 { '-' } else { '+' };
            out.write_fmt(format_args!("{} {} / {} [ {} {} {} ]\n", Render::tab(indent + 1), self.current, self.get_modified_max(), self.max, diff_char, self.temporary.abs()))?;
        }
        Ok(())
    }
}