use std::{
    cell::RefCell,
    ops::{Add, Mul},
};

use crate::{
    modifier::{
        standard::{Additive, Flat, Multiplicative, PostAdditive, PostMultiplicative},
        Modifier,
    },
    stat::{Stat5, StatMarker},
};

pub struct StandardStatNSBase<
    Marker,
    Flat,
    Additive,
    PostAdditive,
    Multiplicative,
    PostMultiplicative,
    const N: usize = 2,
>(pub RefCell<Stat5<Marker, Flat, Additive, PostAdditive, Multiplicative, PostMultiplicative, N>>)
where
    Marker: StatMarker,
    Flat: Modifier,
    Additive: Modifier,
    PostAdditive: Modifier,
    Multiplicative: Modifier,
    PostMultiplicative: Modifier,
    <Marker as StatMarker>::Raw:
        Add<<<Flat as Modifier>::Target as StatMarker>::Raw, Output = <Marker as StatMarker>::Raw>,
    <Marker as StatMarker>::Raw: Mul<
        <<Additive as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostAdditive as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Multiplicative as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostMultiplicative as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >;

impl<Marker, Flat, Additive, PostAdditive, Multiplicative, PostMultiplicative, const N: usize>
    StandardStatNSBase<Marker, Flat, Additive, PostAdditive, Multiplicative, PostMultiplicative, N>
where
    Marker: StatMarker,
    Flat: Modifier,
    Additive: Modifier,
    PostAdditive: Modifier,
    Multiplicative: Modifier,
    PostMultiplicative: Modifier,
    <Marker as StatMarker>::Raw:
        Add<<<Flat as Modifier>::Target as StatMarker>::Raw, Output = <Marker as StatMarker>::Raw>,
    <Marker as StatMarker>::Raw: Mul<
        <<Additive as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostAdditive as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Multiplicative as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostMultiplicative as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
{
    pub fn new(base: Marker::Raw) -> Self {
        Self(RefCell::new(Stat5::new(
            base,
            Box::new(|b, m1, m2, m3, m4, m5| {
                ((b + Flat::combine(m1)) * Additive::combine(m2) + PostAdditive::combine(m3))
                    * Multiplicative::combine(m4)
                    + PostMultiplicative::combine(m5)
            }),
        )))
    }

    pub fn apply_flat(&self, value: Flat) {
        self.0.borrow_mut().apply_m1(value);
    }

    pub fn apply_additive(&self, value: Additive) {
        self.0.borrow_mut().apply_m2(value);
    }

    pub fn apply_post_add(&self, value: PostAdditive) {
        self.0.borrow_mut().apply_m3(value);
    }

    pub fn apply_multiplicative(&self, value: Multiplicative) {
        self.0.borrow_mut().apply_m4(value);
    }

    pub fn apply_post_mul(&self, value: PostMultiplicative) {
        self.0.borrow_mut().apply_m5(value);
    }

    pub fn remove_flat(&self, value: Flat) {
        self.0.borrow_mut().remove_m1(value);
    }

    pub fn remove_additive(&self, value: Additive) {
        self.0.borrow_mut().remove_m2(value);
    }

    pub fn remove_post_add(&self, value: PostAdditive) {
        self.0.borrow_mut().remove_m3(value);
    }

    pub fn remove_multiplicative(&self, value: Multiplicative) {
        self.0.borrow_mut().remove_m4(value);
    }

    pub fn remove_post_mul(&self, value: PostMultiplicative) {
        self.0.borrow_mut().remove_m5(value);
    }

    pub fn has_flat(&self, value: Flat) -> bool {
        self.0.borrow().has_m1(value)
    }

    pub fn has_additive(&self, value: Additive) -> bool {
        self.0.borrow().has_m2(value)
    }

    pub fn has_post_add(&self, value: PostAdditive) -> bool {
        self.0.borrow().has_m3(value)
    }

    pub fn has_multiplicative(&self, value: Multiplicative) -> bool {
        self.0.borrow().has_m4(value)
    }

    pub fn has_post_mul(&self, value: PostMultiplicative) -> bool {
        self.0.borrow().has_m5(value)
    }
    
    pub fn for_each_flat<F>(&self, f: F)
    where
        F: FnMut(&Flat),
    {
        self.0.borrow().m1().iter().for_each(f);
    }
    
    pub fn for_each_additive<F>(&self, f: F)
    where
        F: FnMut(&Additive),
    {
        self.0.borrow().m2().iter().for_each(f);
    }
    
    pub fn for_each_post_add<F>(&self, f: F)
    where
        F: FnMut(&PostAdditive),
    {
        self.0.borrow().m3().iter().for_each(f);
    }
    
    pub fn for_each_multiplicative<F>(&self, f: F)
    where
        F: FnMut(&Multiplicative),
    {
        self.0.borrow().m4().iter().for_each(f);
    }
    
    pub fn for_each_post_mul<F>(&self, f: F)
    where
        F: FnMut(&PostMultiplicative),
    {
        self.0.borrow().m5().iter().for_each(f);
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.borrow().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.borrow_mut().get()
    }
}

pub type StandardStatNS<Marker, Metadata> = StandardStatNSBase<
    Marker,
    Flat<Marker, <Marker as StatMarker>::Raw, Metadata>,
    Additive<Marker, <Marker as StatMarker>::Raw, Metadata>,
    PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>,
    Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>,
    PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>,
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[derive(PartialEq, Clone, Copy)]
        struct DummyMarker;

        impl StatMarker for DummyMarker {
            type Raw = f32;
        }

        #[derive(PartialEq, Clone, Copy, Debug)]
        enum DummyEnum {
            First,
            Second,
            Third,
            Fourth,
            Fifth,
            Sixth,
            Seventh,
            Eighth,
            Ninth,
            Tenth,
        }

        let stat = StandardStatNS::<DummyMarker, DummyEnum>::new(0.);

        stat.apply_flat(
            Flat::from_raw(1.)
                .set_metadata(Some(DummyEnum::First))
                .build(),
        );
        stat.apply_flat(
            Flat::from_raw(1.)
                .set_metadata(Some(DummyEnum::Second))
                .build(),
        );
        stat.apply_additive(
            Additive::from_raw(1.)
                .set_metadata(Some(DummyEnum::Third))
                .build(),
        );
        stat.apply_additive(
            Additive::from_raw(1.)
                .set_metadata(Some(DummyEnum::Fourth))
                .build(),
        );
        stat.apply_post_add(
            PostAdditive::from_raw(1.)
                .set_metadata(Some(DummyEnum::Fifth))
                .build(),
        );
        stat.apply_post_add(
            PostAdditive::from_raw(1.)
                .set_metadata(Some(DummyEnum::Sixth))
                .build(),
        );
        stat.apply_multiplicative(
            Multiplicative::from_raw(1.)
                .set_metadata(Some(DummyEnum::Seventh))
                .build(),
        );
        stat.apply_multiplicative(
            Multiplicative::from_raw(1.)
                .set_metadata(Some(DummyEnum::Eighth))
                .build(),
        );
        stat.apply_post_mul(
            PostMultiplicative::from_raw(1.)
                .set_metadata(Some(DummyEnum::Ninth))
                .build(),
        );
        stat.apply_post_mul(
            PostMultiplicative::from_raw(1.)
                .set_metadata(Some(DummyEnum::Tenth))
                .build(),
        );

        assert_eq!(10., stat.get());

        stat.for_each_flat(|f| println!("{:?}: +{}", f.metadata().unwrap(), f.raw()));
    }
}
