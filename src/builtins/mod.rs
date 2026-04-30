mod echo;
mod exit;
mod r#type;

pub use echo::Echo;
pub use exit::Exit;
pub use r#type::Type;

pub(crate) trait Builtin: Sized {
    const NAME: &'static str;
    fn parse(args: &[&str]) -> anyhow::Result<Self>;
    fn run(&self) -> anyhow::Result<()>;
}
