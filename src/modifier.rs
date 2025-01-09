use crate::stat::StatMarker;

pub mod standard;

pub trait Modifier: Sized + PartialEq + Clone + Copy {
    type Target: StatMarker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self;

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw;

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw;
}
