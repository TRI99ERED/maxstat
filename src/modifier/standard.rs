use std::marker::PhantomData;

use crate::stat::StatMarker;

use super::Modifier;

#[derive(PartialEq, Clone, Copy)]
pub struct Flat<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    raw: R,
    metadata: Option<M>,
    _p: PhantomData<Marker>,
}

impl<Marker, R, M> Flat<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<M>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<M> {
        self.metadata
    }
}

impl<Marker, M> Modifier for Flat<Marker, f32, M>
where
    Marker: StatMarker<Raw = f32>,
    M: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker, M> Modifier for Flat<Marker, f64, M>
where
    Marker: StatMarker<Raw = f64>,
    M: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Additive<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    raw: R,
    metadata: Option<M>,
    _p: PhantomData<Marker>,
}

impl<Marker, R, M> Additive<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<M>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<M> {
        self.metadata
    }
}

impl<Marker, M> Modifier for Additive<Marker, f32, M>
where
    Marker: StatMarker<Raw = f32>,
    M: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc + m.raw)
    }
}

impl<Marker, M> Modifier for Additive<Marker, f64, M>
where
    Marker: StatMarker<Raw = f64>,
    M: PartialEq + Clone + Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PostAdditive<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone +  Copy,
{
    raw: R,
    metadata: Option<M>,
    _p: PhantomData<Marker>,
}

impl<Marker, R, M> PostAdditive<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<M>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<M> {
        self.metadata
    }
}

impl<Marker, M> Modifier for PostAdditive<Marker, f32, M>
where
    Marker: StatMarker<Raw = f32>,
    M: PartialEq + Clone +  Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker, M> Modifier for PostAdditive<Marker, f64, M>
where
    Marker: StatMarker<Raw = f64>,
    M: PartialEq + Clone +  Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Multiplicative<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone +  Copy,
{
    raw: R,
    metadata: Option<M>,
    _p: PhantomData<Marker>,
}

impl<Marker, R, M> Multiplicative<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<M>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<M> {
        self.metadata
    }
}

impl<Marker, M> Modifier for Multiplicative<Marker, f32, M>
where
    Marker: StatMarker<Raw = f32>,
    M: PartialEq + Clone +  Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc * m.raw)
    }
}

impl<Marker, M> Modifier for Multiplicative<Marker, f64, M>
where
    Marker: StatMarker<Raw = f64>,
    M: PartialEq + Clone +  Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(1., |acc, m| acc * m.raw)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PostMultiplicative<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone +  Copy,
{
    raw: R,
    metadata: Option<M>,
    _p: PhantomData<Marker>,
}

impl<Marker, R, M> PostMultiplicative<Marker, R, M>
where
    Marker: StatMarker,
    R: PartialEq + Clone +  Copy,
    M: PartialEq + Clone + Copy,
{
    pub fn build(&mut self) -> Self {
        *self
    }

    pub fn set_metadata(&mut self, metadata: Option<M>) -> &mut Self {
        self.metadata = metadata;
        self
    }

    pub fn metadata(&self) -> Option<M> {
        self.metadata
    }
}

impl<Marker, M> Modifier for PostMultiplicative<Marker, f32, M>
where
    Marker: StatMarker<Raw = f32>,
    M: PartialEq + Clone +  Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}

impl<Marker, M> Modifier for PostMultiplicative<Marker, f64, M>
where
    Marker: StatMarker<Raw = f64>,
    M: PartialEq + Clone +  Copy,
{
    type Target = Marker;

    fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
        Self { raw, metadata: None, _p: PhantomData }
    }

    fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        self.raw
    }

    fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
        s.iter().fold(0., |acc, m| acc + m.raw)
    }
}
