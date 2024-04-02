use argh::FromArgs;
use compact_str::CompactString;

#[derive(FromArgs)]
/// Deck
pub struct Args {
    /// card name
    #[argh(option)]
    pub name: CompactString,
}
