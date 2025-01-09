use std::{
    ops::{Add, Mul},
    sync::Mutex,
};

use crate::{
    modifier::{
        standard::{Additive, Flat, Multiplicative, PostAdditive, PostMultiplicative},
        Modifier,
    },
    stat::{Stat5, StatMarker},
};

pub struct StandardStatS<Marker, Metadata, const N: usize = 2>(
    pub Mutex<
        Stat5<
            Marker,
            Flat<Marker, <Marker as StatMarker>::Raw, Metadata>,
            Additive<Marker, <Marker as StatMarker>::Raw, Metadata>,
            PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>,
            Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>,
            PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>,
            N,
        >,
    >,
)
where
    Marker: StatMarker,
    Flat<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    Additive<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    Metadata: PartialEq + Clone + Copy,
    <Marker as StatMarker>::Raw:
        Add<<<Flat<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw, Output = <Marker as StatMarker>::Raw>,
    <Marker as StatMarker>::Raw: Mul<
        <<Additive<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >;

impl<Marker, Metadata, const N: usize> StandardStatS<Marker, Metadata, N>
where
    Marker: StatMarker,
    Flat<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    Additive<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>: Modifier,
    Metadata: PartialEq + Clone + Copy,
    <Marker as StatMarker>::Raw:
        Add<<<Flat<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw, Output = <Marker as StatMarker>::Raw>,
    <Marker as StatMarker>::Raw: Mul<
        <<Additive<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata> as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >
{
    pub fn new(base: Marker::Raw) -> Self {
        Self(Mutex::new(Stat5::new(
            base,
            Box::new(|b, m1, m2, m3, m4, m5| {
                ((b + Flat::combine(m1)) * Additive::combine(m2) + PostAdditive::combine(m3))
                    * Multiplicative::combine(m4)
                    + PostMultiplicative::combine(m5)
            }),
        )))
    }

    pub fn apply_flat(&self, value: Flat<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_additive(&self, value: Additive<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_post_add(&self, value: PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_multiplicative(&self, value: Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn apply_post_mul(&self, value: PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().apply_m5(value);
    }

    pub fn remove_flat(&self, value: Flat<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_additive(&self, value: Additive<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_post_add(&self, value: PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_multiplicative(&self, value: Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn remove_post_mul(&self, value: PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>) {
        self.0.lock().unwrap().remove_m5(value);
    }

    pub fn has_flat(&self, value: Flat<Marker, <Marker as StatMarker>::Raw, Metadata>) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_additive(&self, value: Additive<Marker, <Marker as StatMarker>::Raw, Metadata>) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_post_add(&self, value: PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_multiplicative(&self, value: Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn has_post_mul(&self, value: PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>) -> bool {
        self.0.lock().unwrap().has_m5(value)
    }

    pub fn for_each_flat<F>(&self, f: F)
    where
        F: FnMut(&Flat<Marker, <Marker as StatMarker>::Raw, Metadata>),
    {
        self.0.lock().unwrap().m1().iter().for_each(f);
    }

    pub fn for_each_additive<F>(&self, f: F)
    where
        F: FnMut(&Additive<Marker, <Marker as StatMarker>::Raw, Metadata>),
    {
        self.0.lock().unwrap().m2().iter().for_each(f);
    }

    pub fn for_each_post_add<F>(&self, f: F)
    where
        F: FnMut(&PostAdditive<Marker, <Marker as StatMarker>::Raw, Metadata>),
    {
        self.0.lock().unwrap().m3().iter().for_each(f);
    }

    pub fn for_each_multiplicative<F>(&self, f: F)
    where
        F: FnMut(&Multiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>),
    {
        self.0.lock().unwrap().m4().iter().for_each(f);
    }

    pub fn for_each_post_mul<F>(&self, f: F)
    where
        F: FnMut(&PostMultiplicative<Marker, <Marker as StatMarker>::Raw, Metadata>),
    {
        self.0.lock().unwrap().m5().iter().for_each(f);
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32() {
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

        let stat = StandardStatS::<DummyMarker, DummyEnum>::new(0.);

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

    #[test]
    fn test_f64() {
        #[derive(PartialEq, Clone, Copy)]
        struct DummyMarker;

        impl StatMarker for DummyMarker {
            type Raw = f64;
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

        let stat = StandardStatS::<DummyMarker, DummyEnum>::new(0.);

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
