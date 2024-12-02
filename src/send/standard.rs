use std::{
    ops::{Add, Mul},
    sync::Mutex,
};

use crate::{
    modifier::{
        standard::{Additive, MidFlat, Multiplicative, PostFlat, PreFlat},
        Modifier,
    },
    stat::{Stat5, StatMarker},
};

pub struct StandardStatSBase<
    Marker,
    PreFlat,
    Additive,
    MidFlat,
    Multiplicative,
    PostFlat,
    const N: usize = 2,
>(pub Mutex<Stat5<Marker, PreFlat, Additive, MidFlat, Multiplicative, PostFlat, N>>)
where
    Marker: StatMarker,
    PreFlat: Modifier,
    Additive: Modifier,
    MidFlat: Modifier,
    Multiplicative: Modifier,
    PostFlat: Modifier,
    <Marker as StatMarker>::Raw: Add<
        <<PreFlat as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Additive as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<MidFlat as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Multiplicative as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostFlat as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >;

impl<Marker, PreFlat, Additive, MidFlat, Multiplicative, PostFlat, const N: usize>
    StandardStatSBase<Marker, PreFlat, Additive, MidFlat, Multiplicative, PostFlat, N>
where
    Marker: StatMarker,
    PreFlat: Modifier,
    Additive: Modifier,
    MidFlat: Modifier,
    Multiplicative: Modifier,
    PostFlat: Modifier,
    <Marker as StatMarker>::Raw: Add<
        <<PreFlat as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Additive as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<MidFlat as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Mul<
        <<Multiplicative as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
    <Marker as StatMarker>::Raw: Add<
        <<PostFlat as Modifier>::Target as StatMarker>::Raw,
        Output = <Marker as StatMarker>::Raw,
    >,
{
    pub fn new(base: Marker::Raw) -> Self {
        Self(Mutex::new(Stat5::new(
            base,
            Box::new(|b, m1, m2, m3, m4, m5| {
                ((b + PreFlat::combine(m1)) * Additive::combine(m2) + MidFlat::combine(m3))
                    * Multiplicative::combine(m4)
                    + PostFlat::combine(m5)
            }),
        )))
    }

    pub fn apply_pre_flat(&self, value: PreFlat) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_additive(&self, value: Additive) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_mid_flat(&self, value: MidFlat) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_multiplicative(&self, value: Multiplicative) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn apply_post_flat(&self, value: PostFlat) {
        self.0.lock().unwrap().apply_m5(value);
    }

    pub fn remove_pre_flat(&mut self, value: PreFlat) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_additive(&mut self, value: Additive) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_mid_flat(&mut self, value: MidFlat) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_multiplicative(&mut self, value: Multiplicative) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn remove_post_flat(&mut self, value: PostFlat) {
        self.0.lock().unwrap().remove_m5(value);
    }

    pub fn has_pre_flat(&self, value: PreFlat) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_additive(&self, value: Additive) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_mid_flat(&self, value: MidFlat) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_multiplicative(&self, value: Multiplicative) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn has_post_flat(&self, value: PostFlat) -> bool {
        self.0.lock().unwrap().has_m5(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub type StandardStatS<Marker> = StandardStatSBase<
    Marker,
    PreFlat<Marker, <Marker as StatMarker>::Raw>,
    Additive<Marker, <Marker as StatMarker>::Raw>,
    MidFlat<Marker, <Marker as StatMarker>::Raw>,
    Multiplicative<Marker, <Marker as StatMarker>::Raw>,
    PostFlat<Marker, <Marker as StatMarker>::Raw>,
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

        let stat = StandardStatS::<DummyMarker>::new(0.);

        stat.apply_pre_flat(PreFlat::from_raw(1.));
        stat.apply_pre_flat(PreFlat::from_raw(1.));
        stat.apply_additive(Additive::from_raw(1.));
        stat.apply_additive(Additive::from_raw(1.));
        stat.apply_mid_flat(MidFlat::from_raw(1.));
        stat.apply_mid_flat(MidFlat::from_raw(1.));
        stat.apply_multiplicative(Multiplicative::from_raw(1.));
        stat.apply_multiplicative(Multiplicative::from_raw(1.));
        stat.apply_post_flat(PostFlat::from_raw(1.));
        stat.apply_post_flat(PostFlat::from_raw(1.));

        assert_eq!(10., stat.get());
    }
}
