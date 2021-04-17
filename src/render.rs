use anyhow::Result;

pub trait Render {
    fn render(&self, indent: usize, out: &mut std::io::Write) -> Result<()>;
}

impl Render {
    pub fn tab(indent: usize) -> String {
        (0..indent).map(|_| '\t').collect::<String>()
    }
}