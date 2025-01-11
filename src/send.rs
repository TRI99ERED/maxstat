use std::sync::Mutex;

use smallvec::SmallVec;

use crate::{
    modifier::Modifier,
    stat::{Stat1, Stat2, Stat3, Stat4, Stat5, Stat6, Stat7, Stat8, StatMarker},
};

pub mod standard;

pub struct Stat1S<Marker, M1, const N: usize = 2>(pub Mutex<Stat1<Marker, M1, N>>)
where
    Marker: StatMarker,
    M1: Modifier;

impl<Marker, M1, const N: usize> Stat1S<Marker, M1, N>
where
    Marker: StatMarker,
    M1: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<dyn Fn(Marker::Raw, &SmallVec<[M1; N]>) -> Marker::Raw + Send>,
    ) -> Self {
        Self(Mutex::new(Stat1::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat2S<Marker, M1, M2, const N: usize = 2>(pub Mutex<Stat2<Marker, M1, M2, N>>)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier;

impl<Marker, M1, M2, const N: usize> Stat2S<Marker, M1, M2, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<dyn Fn(Marker::Raw, &SmallVec<[M1; N]>, &SmallVec<[M2; N]>) -> Marker::Raw + Send>,
    ) -> Self {
        Self(Mutex::new(Stat2::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat3S<Marker, M1, M2, M3, const N: usize = 2>(pub Mutex<Stat3<Marker, M1, M2, M3, N>>)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier;

impl<Marker, M1, M2, M3, const N: usize> Stat3S<Marker, M1, M2, M3, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<
            dyn Fn(
                Marker::Raw,
                &SmallVec<[M1; N]>,
                &SmallVec<[M2; N]>,
                &SmallVec<[M3; N]>,
            ) -> Marker::Raw + Send,
        >,
    ) -> Self {
        Self(Mutex::new(Stat3::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_m3(&self, value: M3) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_m3(&mut self, value: M3) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat4S<Marker, M1, M2, M3, M4, const N: usize = 2>(
    pub Mutex<Stat4<Marker, M1, M2, M3, M4, N>>,
)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier;

impl<Marker, M1, M2, M3, M4, const N: usize> Stat4S<Marker, M1, M2, M3, M4, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<
            dyn Fn(
                Marker::Raw,
                &SmallVec<[M1; N]>,
                &SmallVec<[M2; N]>,
                &SmallVec<[M3; N]>,
                &SmallVec<[M4; N]>,
            ) -> Marker::Raw + Send,
        >,
    ) -> Self {
        Self(Mutex::new(Stat4::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_m3(&self, value: M3) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_m4(&self, value: M4) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_m3(&mut self, value: M3) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_m4(&mut self, value: M4) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat5S<Marker, M1, M2, M3, M4, M5, const N: usize = 2>(
    pub Mutex<Stat5<Marker, M1, M2, M3, M4, M5, N>>,
)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier;

impl<Marker, M1, M2, M3, M4, M5, const N: usize> Stat5S<Marker, M1, M2, M3, M4, M5, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<
            dyn Fn(
                Marker::Raw,
                &SmallVec<[M1; N]>,
                &SmallVec<[M2; N]>,
                &SmallVec<[M3; N]>,
                &SmallVec<[M4; N]>,
                &SmallVec<[M5; N]>,
            ) -> Marker::Raw + Send,
        >,
    ) -> Self {
        Self(Mutex::new(Stat5::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_m3(&self, value: M3) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_m4(&self, value: M4) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn apply_m5(&self, value: M5) {
        self.0.lock().unwrap().apply_m5(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_m3(&mut self, value: M3) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_m4(&mut self, value: M4) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn remove_m5(&mut self, value: M5) {
        self.0.lock().unwrap().remove_m5(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.0.lock().unwrap().has_m5(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat6S<Marker, M1, M2, M3, M4, M5, M6, const N: usize = 2>(
    pub Mutex<Stat6<Marker, M1, M2, M3, M4, M5, M6, N>>,
)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier;

impl<Marker, M1, M2, M3, M4, M5, M6, const N: usize> Stat6S<Marker, M1, M2, M3, M4, M5, M6, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<
            dyn Fn(
                Marker::Raw,
                &SmallVec<[M1; N]>,
                &SmallVec<[M2; N]>,
                &SmallVec<[M3; N]>,
                &SmallVec<[M4; N]>,
                &SmallVec<[M5; N]>,
                &SmallVec<[M6; N]>,
            ) -> Marker::Raw + Send,
        >,
    ) -> Self {
        Self(Mutex::new(Stat6::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_m3(&self, value: M3) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_m4(&self, value: M4) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn apply_m5(&self, value: M5) {
        self.0.lock().unwrap().apply_m5(value);
    }

    pub fn apply_m6(&self, value: M6) {
        self.0.lock().unwrap().apply_m6(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_m3(&mut self, value: M3) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_m4(&mut self, value: M4) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn remove_m5(&mut self, value: M5) {
        self.0.lock().unwrap().remove_m5(value);
    }

    pub fn remove_m6(&mut self, value: M6) {
        self.0.lock().unwrap().remove_m6(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.0.lock().unwrap().has_m5(value)
    }

    pub fn has_m6(&self, value: M6) -> bool {
        self.0.lock().unwrap().has_m6(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat7S<Marker, M1, M2, M3, M4, M5, M6, M7, const N: usize = 2>(
    pub Mutex<Stat7<Marker, M1, M2, M3, M4, M5, M6, M7, N>>,
)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier,
    M7: Modifier;

impl<Marker, M1, M2, M3, M4, M5, M6, M7, const N: usize>
    Stat7S<Marker, M1, M2, M3, M4, M5, M6, M7, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier,
    M7: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<
            dyn Fn(
                Marker::Raw,
                &SmallVec<[M1; N]>,
                &SmallVec<[M2; N]>,
                &SmallVec<[M3; N]>,
                &SmallVec<[M4; N]>,
                &SmallVec<[M5; N]>,
                &SmallVec<[M6; N]>,
                &SmallVec<[M7; N]>,
            ) -> Marker::Raw + Send,
        >,
    ) -> Self {
        Self(Mutex::new(Stat7::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_m3(&self, value: M3) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_m4(&self, value: M4) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn apply_m5(&self, value: M5) {
        self.0.lock().unwrap().apply_m5(value);
    }

    pub fn apply_m6(&self, value: M6) {
        self.0.lock().unwrap().apply_m6(value);
    }

    pub fn apply_m7(&self, value: M7) {
        self.0.lock().unwrap().apply_m7(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_m3(&mut self, value: M3) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_m4(&mut self, value: M4) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn remove_m5(&mut self, value: M5) {
        self.0.lock().unwrap().remove_m5(value);
    }

    pub fn remove_m6(&mut self, value: M6) {
        self.0.lock().unwrap().remove_m6(value);
    }

    pub fn remove_m7(&mut self, value: M7) {
        self.0.lock().unwrap().remove_m7(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.0.lock().unwrap().has_m5(value)
    }

    pub fn has_m6(&self, value: M6) -> bool {
        self.0.lock().unwrap().has_m6(value)
    }

    pub fn has_m7(&self, value: M7) -> bool {
        self.0.lock().unwrap().has_m7(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}

pub struct Stat8S<Marker, M1, M2, M3, M4, M5, M6, M7, M8, const N: usize = 2>(
    pub Mutex<Stat8<Marker, M1, M2, M3, M4, M5, M6, M7, M8, N>>,
)
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier,
    M7: Modifier,
    M8: Modifier;

impl<Marker, M1, M2, M3, M4, M5, M6, M7, M8, const N: usize>
    Stat8S<Marker, M1, M2, M3, M4, M5, M6, M7, M8, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier,
    M7: Modifier,
    M8: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<
            dyn Fn(
                Marker::Raw,
                &SmallVec<[M1; N]>,
                &SmallVec<[M2; N]>,
                &SmallVec<[M3; N]>,
                &SmallVec<[M4; N]>,
                &SmallVec<[M5; N]>,
                &SmallVec<[M6; N]>,
                &SmallVec<[M7; N]>,
                &SmallVec<[M8; N]>,
            ) -> Marker::Raw + Send,
        >,
    ) -> Self {
        Self(Mutex::new(Stat8::new(base, f)))
    }

    pub fn apply_m1(&self, value: M1) {
        self.0.lock().unwrap().apply_m1(value);
    }

    pub fn apply_m2(&self, value: M2) {
        self.0.lock().unwrap().apply_m2(value);
    }

    pub fn apply_m3(&self, value: M3) {
        self.0.lock().unwrap().apply_m3(value);
    }

    pub fn apply_m4(&self, value: M4) {
        self.0.lock().unwrap().apply_m4(value);
    }

    pub fn apply_m5(&self, value: M5) {
        self.0.lock().unwrap().apply_m5(value);
    }

    pub fn apply_m6(&self, value: M6) {
        self.0.lock().unwrap().apply_m6(value);
    }

    pub fn apply_m7(&self, value: M7) {
        self.0.lock().unwrap().apply_m7(value);
    }

    pub fn apply_m8(&self, value: M8) {
        self.0.lock().unwrap().apply_m8(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        self.0.lock().unwrap().remove_m1(value);
    }

    pub fn remove_m2(&mut self, value: M2) {
        self.0.lock().unwrap().remove_m2(value);
    }

    pub fn remove_m3(&mut self, value: M3) {
        self.0.lock().unwrap().remove_m3(value);
    }

    pub fn remove_m4(&mut self, value: M4) {
        self.0.lock().unwrap().remove_m4(value);
    }

    pub fn remove_m5(&mut self, value: M5) {
        self.0.lock().unwrap().remove_m5(value);
    }

    pub fn remove_m6(&mut self, value: M6) {
        self.0.lock().unwrap().remove_m6(value);
    }

    pub fn remove_m7(&mut self, value: M7) {
        self.0.lock().unwrap().remove_m7(value);
    }

    pub fn remove_m8(&mut self, value: M8) {
        self.0.lock().unwrap().remove_m8(value);
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.0.lock().unwrap().has_m1(value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.0.lock().unwrap().has_m2(value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.0.lock().unwrap().has_m3(value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.0.lock().unwrap().has_m4(value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.0.lock().unwrap().has_m5(value)
    }

    pub fn has_m6(&self, value: M6) -> bool {
        self.0.lock().unwrap().has_m6(value)
    }

    pub fn has_m7(&self, value: M7) -> bool {
        self.0.lock().unwrap().has_m7(value)
    }

    pub fn has_m8(&self, value: M8) -> bool {
        self.0.lock().unwrap().has_m8(value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.0.lock().unwrap().base()
    }

    pub fn get(&self) -> Marker::Raw {
        self.0.lock().unwrap().get()
    }
}
