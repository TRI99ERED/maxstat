use std::marker::PhantomData;

use crate::stat::StatMarker;

use super::Modifier;

#[derive(PartialEq, Clone, Copy)]
pub struct Flat<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    raw: Raw,
    metadata: Option<Metadata>,
    _p: PhantomData<Marker>,
}

impl<Marker, Raw, Metadata> Flat<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<Metadata>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.metadata
    }
}

impl<Marker, Metadata> Modifier for Flat<Marker, f32, Metadata>
where
    Marker: StatMarker<Raw = f32>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker, Metadata> Modifier for Flat<Marker, f64, Metadata>
where
    Marker: StatMarker<Raw = f64>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Additive<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    raw: Raw,
    metadata: Option<Metadata>,
    _p: PhantomData<Marker>,
}

impl<Marker, Raw, Metadata> Additive<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<Metadata>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.metadata
    }
}

impl<Marker, Metadata> Modifier for Additive<Marker, f32, Metadata>
where
    Marker: StatMarker<Raw = f32>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc + m.raw)
    }
}

impl<Marker, Metadata> Modifier for Additive<Marker, f64, Metadata>
where
    Marker: StatMarker<Raw = f64>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PostAdditive<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    raw: Raw,
    metadata: Option<Metadata>,
    _p: PhantomData<Marker>,
}

impl<Marker, Raw, Metadata> PostAdditive<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<Metadata>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.metadata
    }
}

impl<Marker, Metadata> Modifier for PostAdditive<Marker, f32, Metadata>
where
    Marker: StatMarker<Raw = f32>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker, Metadata> Modifier for PostAdditive<Marker, f64, Metadata>
where
    Marker: StatMarker<Raw = f64>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Multiplicative<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    raw: Raw,
    metadata: Option<Metadata>,
    _p: PhantomData<Marker>,
}

impl<Marker, Raw, Metadata> Multiplicative<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<Metadata>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.metadata
    }
}

impl<Marker, Metadata> Modifier for Multiplicative<Marker, f32, Metadata>
where
    Marker: StatMarker<Raw = f32>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc * m.raw)
    }
}

impl<Marker, Metadata> Modifier for Multiplicative<Marker, f64, Metadata>
where
    Marker: StatMarker<Raw = f64>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc * m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PostMultiplicative<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    raw: Raw,
    metadata: Option<Metadata>,
    _p: PhantomData<Marker>,
}

impl<Marker, Raw, Metadata> PostMultiplicative<Marker, Raw, Metadata>
where
    Marker: StatMarker,
    Raw: PartialEq + Clone + Copy,
    Metadata: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<Metadata>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.metadata
    }
}

impl<Marker, Metadata> Modifier for PostMultiplicative<Marker, f32, Metadata>
where
    Marker: StatMarker<Raw = f32>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker, Metadata> Modifier for PostMultiplicative<Marker, f64, Metadata>
where
    Marker: StatMarker<Raw = f64>,
    Metadata: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self {
            raw,
            metadata: None,
            _p: PhantomData,
        }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}
