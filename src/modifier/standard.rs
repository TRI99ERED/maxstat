use std::marker::PhantomData;

use crate::stat::StatMarker;

use super::Modifier;

#[derive(PartialEq, Clone, Copy)]
pub struct PreFlat<Marker, R>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
{
    raw: R,
    _p: PhantomData<Marker>,
}

impl<Marker> Modifier for PreFlat<Marker, f32>
where
    Marker: StatMarker<Raw = f32>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker> Modifier for PreFlat<Marker, f64>
where
    Marker: StatMarker<Raw = f64>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Additive<Marker, R>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
{
    raw: R,
    _p: PhantomData<Marker>,
}

impl<Marker> Modifier for Additive<Marker, f32>
where
    Marker: StatMarker<Raw = f32>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc + m.raw)
    }
}

impl<Marker> Modifier for Additive<Marker, f64>
where
    Marker: StatMarker<Raw = f64>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct MidFlat<Marker, R>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
{
    raw: R,
    _p: PhantomData<Marker>,
}

impl<Marker> Modifier for MidFlat<Marker, f32>
where
    Marker: StatMarker<Raw = f32>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker> Modifier for MidFlat<Marker, f64>
where
    Marker: StatMarker<Raw = f64>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Multiplicative<Marker, R>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
{
    raw: R,
    _p: PhantomData<Marker>,
}

impl<Marker> Modifier for Multiplicative<Marker, f32>
where
    Marker: StatMarker<Raw = f32>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc * m.raw)
    }
}

impl<Marker> Modifier for Multiplicative<Marker, f64>
where
    Marker: StatMarker<Raw = f64>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc * m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PostFlat<Marker, R>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
{
    raw: R,
    _p: PhantomData<Marker>,
}

impl<Marker> Modifier for PostFlat<Marker, f32>
where
    Marker: StatMarker<Raw = f32>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker> Modifier for PostFlat<Marker, f64>
where
    Marker: StatMarker<Raw = f64>,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}
