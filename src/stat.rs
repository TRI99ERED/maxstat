use std::{
    cmp::PartialEq,
    marker::PhantomData,
    ops::{Add, Mul},
};

use smallvec::SmallVec;

use crate::modifier::Modifier;

pub trait StatMarker: PartialEq + Clone + Copy {
    type Raw: PartialEq + Clone + Copy + Add<Output = Self::Raw> + Mul<Output = Self::Raw>;
}

pub struct Stat1<Marker, M1, const N: usize = 2>
where
    Marker: StatMarker,
    M1: Modifier,
{
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    f: Box<dyn Fn(Marker::Raw, &SmallVec<[M1; N]>) -> Marker::Raw>,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, const N: usize> Stat1<Marker, M1, N>
where
    Marker: StatMarker,
    M1: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<dyn Fn(Marker::Raw, &SmallVec<[M1; N]>) -> Marker::Raw>,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(self.base, &self.m1)
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }
}

pub struct Stat2<Marker, M1, M2, const N: usize = 2>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
{
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    f: Box<dyn Fn(Marker::Raw, &SmallVec<[M1; N]>, &SmallVec<[M2; N]>) -> Marker::Raw>,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, const N: usize> Stat2<Marker, M1, M2, N>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
{
    pub fn new(
        base: Marker::Raw,
        f: Box<dyn Fn(Marker::Raw, &SmallVec<[M1; N]>, &SmallVec<[M2; N]>) -> Marker::Raw>,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(self.base, &self.m1, &self.m2)
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }
}

pub struct Stat3<Marker, M1, M2, M3, const N: usize = 2>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
{
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    m3: SmallVec<[M3; N]>,
    f: Box<
        dyn Fn(
            Marker::Raw,
            &SmallVec<[M1; N]>,
            &SmallVec<[M2; N]>,
            &SmallVec<[M3; N]>,
        ) -> Marker::Raw,
    >,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, M3, const N: usize> Stat3<Marker, M1, M2, M3, N>
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
            ) -> Marker::Raw,
        >,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            m3: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn apply_m3(&mut self, value: M3) {
        self.m3.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn remove_m3(&mut self, value: M3) {
        if let Some(i) = self.m3.iter().position(|&v| v == value) {
            self.m3.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.m3.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(self.base, &self.m1, &self.m2, &self.m3)
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }

    pub fn m3(&self) -> &SmallVec<[M3; N]> {
        &self.m3
    }
}

pub struct Stat4<Marker, M1, M2, M3, M4, const N: usize = 2>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
{
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    m3: SmallVec<[M3; N]>,
    m4: SmallVec<[M4; N]>,
    f: Box<
        dyn Fn(
            Marker::Raw,
            &SmallVec<[M1; N]>,
            &SmallVec<[M2; N]>,
            &SmallVec<[M3; N]>,
            &SmallVec<[M4; N]>,
        ) -> Marker::Raw,
    >,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, M3, M4, const N: usize> Stat4<Marker, M1, M2, M3, M4, N>
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
            ) -> Marker::Raw,
        >,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            m3: SmallVec::new(),
            m4: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn apply_m3(&mut self, value: M3) {
        self.m3.push(value);
    }

    pub fn apply_m4(&mut self, value: M4) {
        self.m4.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn remove_m3(&mut self, value: M3) {
        if let Some(i) = self.m3.iter().position(|&v| v == value) {
            self.m3.swap_remove(i);
        }
    }

    pub fn remove_m4(&mut self, value: M4) {
        if let Some(i) = self.m4.iter().position(|&v| v == value) {
            self.m4.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.m3.iter().any(|&v| v == value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.m4.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(self.base, &self.m1, &self.m2, &self.m3, &self.m4)
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }

    pub fn m3(&self) -> &SmallVec<[M3; N]> {
        &self.m3
    }

    pub fn m4(&self) -> &SmallVec<[M4; N]> {
        &self.m4
    }
}

pub struct Stat5<Marker, M1, M2, M3, M4, M5, const N: usize = 2>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
{
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    m3: SmallVec<[M3; N]>,
    m4: SmallVec<[M4; N]>,
    m5: SmallVec<[M5; N]>,
    f: Box<
        dyn Fn(
            Marker::Raw,
            &SmallVec<[M1; N]>,
            &SmallVec<[M2; N]>,
            &SmallVec<[M3; N]>,
            &SmallVec<[M4; N]>,
            &SmallVec<[M5; N]>,
        ) -> Marker::Raw,
    >,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, M3, M4, M5, const N: usize> Stat5<Marker, M1, M2, M3, M4, M5, N>
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
            ) -> Marker::Raw,
        >,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            m3: SmallVec::new(),
            m4: SmallVec::new(),
            m5: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn apply_m3(&mut self, value: M3) {
        self.m3.push(value);
    }

    pub fn apply_m4(&mut self, value: M4) {
        self.m4.push(value);
    }

    pub fn apply_m5(&mut self, value: M5) {
        self.m5.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn remove_m3(&mut self, value: M3) {
        if let Some(i) = self.m3.iter().position(|&v| v == value) {
            self.m3.swap_remove(i);
        }
    }

    pub fn remove_m4(&mut self, value: M4) {
        if let Some(i) = self.m4.iter().position(|&v| v == value) {
            self.m4.swap_remove(i);
        }
    }

    pub fn remove_m5(&mut self, value: M5) {
        if let Some(i) = self.m5.iter().position(|&v| v == value) {
            self.m5.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.m3.iter().any(|&v| v == value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.m4.iter().any(|&v| v == value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.m5.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(self.base, &self.m1, &self.m2, &self.m3, &self.m4, &self.m5)
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }

    pub fn m3(&self) -> &SmallVec<[M3; N]> {
        &self.m3
    }

    pub fn m4(&self) -> &SmallVec<[M4; N]> {
        &self.m4
    }

    pub fn m5(&self) -> &SmallVec<[M5; N]> {
        &self.m5
    }
}

pub struct Stat6<Marker, M1, M2, M3, M4, M5, M6, const N: usize = 2>
where
    Marker: StatMarker,
    M1: Modifier,
    M2: Modifier,
    M3: Modifier,
    M4: Modifier,
    M5: Modifier,
    M6: Modifier,
{
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    m3: SmallVec<[M3; N]>,
    m4: SmallVec<[M4; N]>,
    m5: SmallVec<[M5; N]>,
    m6: SmallVec<[M6; N]>,
    f: Box<
        dyn Fn(
            Marker::Raw,
            &SmallVec<[M1; N]>,
            &SmallVec<[M2; N]>,
            &SmallVec<[M3; N]>,
            &SmallVec<[M4; N]>,
            &SmallVec<[M5; N]>,
            &SmallVec<[M6; N]>,
        ) -> Marker::Raw,
    >,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, M3, M4, M5, M6, const N: usize> Stat6<Marker, M1, M2, M3, M4, M5, M6, N>
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
            ) -> Marker::Raw,
        >,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            m3: SmallVec::new(),
            m4: SmallVec::new(),
            m5: SmallVec::new(),
            m6: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn apply_m3(&mut self, value: M3) {
        self.m3.push(value);
    }

    pub fn apply_m4(&mut self, value: M4) {
        self.m4.push(value);
    }

    pub fn apply_m5(&mut self, value: M5) {
        self.m5.push(value);
    }

    pub fn apply_m6(&mut self, value: M6) {
        self.m6.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn remove_m3(&mut self, value: M3) {
        if let Some(i) = self.m3.iter().position(|&v| v == value) {
            self.m3.swap_remove(i);
        }
    }

    pub fn remove_m4(&mut self, value: M4) {
        if let Some(i) = self.m4.iter().position(|&v| v == value) {
            self.m4.swap_remove(i);
        }
    }

    pub fn remove_m5(&mut self, value: M5) {
        if let Some(i) = self.m5.iter().position(|&v| v == value) {
            self.m5.swap_remove(i);
        }
    }

    pub fn remove_m6(&mut self, value: M6) {
        if let Some(i) = self.m6.iter().position(|&v| v == value) {
            self.m6.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.m3.iter().any(|&v| v == value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.m4.iter().any(|&v| v == value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.m5.iter().any(|&v| v == value)
    }

    pub fn has_m6(&self, value: M6) -> bool {
        self.m6.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(
            self.base, &self.m1, &self.m2, &self.m3, &self.m4, &self.m5, &self.m6,
        )
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }

    pub fn m3(&self) -> &SmallVec<[M3; N]> {
        &self.m3
    }

    pub fn m4(&self) -> &SmallVec<[M4; N]> {
        &self.m4
    }

    pub fn m5(&self) -> &SmallVec<[M5; N]> {
        &self.m5
    }

    pub fn m6(&self) -> &SmallVec<[M6; N]> {
        &self.m6
    }
}

pub struct Stat7<Marker, M1, M2, M3, M4, M5, M6, M7, const N: usize = 2>
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
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    m3: SmallVec<[M3; N]>,
    m4: SmallVec<[M4; N]>,
    m5: SmallVec<[M5; N]>,
    m6: SmallVec<[M6; N]>,
    m7: SmallVec<[M7; N]>,
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
        ) -> Marker::Raw,
    >,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, M3, M4, M5, M6, M7, const N: usize>
    Stat7<Marker, M1, M2, M3, M4, M5, M6, M7, N>
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
            ) -> Marker::Raw,
        >,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            m3: SmallVec::new(),
            m4: SmallVec::new(),
            m5: SmallVec::new(),
            m6: SmallVec::new(),
            m7: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn apply_m3(&mut self, value: M3) {
        self.m3.push(value);
    }

    pub fn apply_m4(&mut self, value: M4) {
        self.m4.push(value);
    }

    pub fn apply_m5(&mut self, value: M5) {
        self.m5.push(value);
    }

    pub fn apply_m6(&mut self, value: M6) {
        self.m6.push(value);
    }

    pub fn apply_m7(&mut self, value: M7) {
        self.m7.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn remove_m3(&mut self, value: M3) {
        if let Some(i) = self.m3.iter().position(|&v| v == value) {
            self.m3.swap_remove(i);
        }
    }

    pub fn remove_m4(&mut self, value: M4) {
        if let Some(i) = self.m4.iter().position(|&v| v == value) {
            self.m4.swap_remove(i);
        }
    }

    pub fn remove_m5(&mut self, value: M5) {
        if let Some(i) = self.m5.iter().position(|&v| v == value) {
            self.m5.swap_remove(i);
        }
    }

    pub fn remove_m6(&mut self, value: M6) {
        if let Some(i) = self.m6.iter().position(|&v| v == value) {
            self.m6.swap_remove(i);
        }
    }

    pub fn remove_m7(&mut self, value: M7) {
        if let Some(i) = self.m7.iter().position(|&v| v == value) {
            self.m7.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.m3.iter().any(|&v| v == value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.m4.iter().any(|&v| v == value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.m5.iter().any(|&v| v == value)
    }

    pub fn has_m6(&self, value: M6) -> bool {
        self.m6.iter().any(|&v| v == value)
    }

    pub fn has_m7(&self, value: M7) -> bool {
        self.m7.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(
            self.base, &self.m1, &self.m2, &self.m3, &self.m4, &self.m5, &self.m6, &self.m7,
        )
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }

    pub fn m3(&self) -> &SmallVec<[M3; N]> {
        &self.m3
    }

    pub fn m4(&self) -> &SmallVec<[M4; N]> {
        &self.m4
    }

    pub fn m5(&self) -> &SmallVec<[M5; N]> {
        &self.m5
    }

    pub fn m6(&self) -> &SmallVec<[M6; N]> {
        &self.m6
    }

    pub fn m7(&self) -> &SmallVec<[M7; N]> {
        &self.m7
    }
}

pub struct Stat8<Marker, M1, M2, M3, M4, M5, M6, M7, M8, const N: usize = 2>
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
    base: Marker::Raw,
    m1: SmallVec<[M1; N]>,
    m2: SmallVec<[M2; N]>,
    m3: SmallVec<[M3; N]>,
    m4: SmallVec<[M4; N]>,
    m5: SmallVec<[M5; N]>,
    m6: SmallVec<[M6; N]>,
    m7: SmallVec<[M7; N]>,
    m8: SmallVec<[M8; N]>,
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
        ) -> Marker::Raw,
    >,
    _p: PhantomData<Marker>,
}

impl<Marker, M1, M2, M3, M4, M5, M6, M7, M8, const N: usize>
    Stat8<Marker, M1, M2, M3, M4, M5, M6, M7, M8, N>
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
            ) -> Marker::Raw,
        >,
    ) -> Self {
        Self {
            base,
            m1: SmallVec::new(),
            m2: SmallVec::new(),
            m3: SmallVec::new(),
            m4: SmallVec::new(),
            m5: SmallVec::new(),
            m6: SmallVec::new(),
            m7: SmallVec::new(),
            m8: SmallVec::new(),
            f,
            _p: PhantomData,
        }
    }

    pub fn apply_m1(&mut self, value: M1) {
        self.m1.push(value);
    }

    pub fn apply_m2(&mut self, value: M2) {
        self.m2.push(value);
    }

    pub fn apply_m3(&mut self, value: M3) {
        self.m3.push(value);
    }

    pub fn apply_m4(&mut self, value: M4) {
        self.m4.push(value);
    }

    pub fn apply_m5(&mut self, value: M5) {
        self.m5.push(value);
    }

    pub fn apply_m6(&mut self, value: M6) {
        self.m6.push(value);
    }

    pub fn apply_m7(&mut self, value: M7) {
        self.m7.push(value);
    }

    pub fn apply_m8(&mut self, value: M8) {
        self.m8.push(value);
    }

    pub fn remove_m1(&mut self, value: M1) {
        if let Some(i) = self.m1.iter().position(|&v| v == value) {
            self.m1.swap_remove(i);
        }
    }

    pub fn remove_m2(&mut self, value: M2) {
        if let Some(i) = self.m2.iter().position(|&v| v == value) {
            self.m2.swap_remove(i);
        }
    }

    pub fn remove_m3(&mut self, value: M3) {
        if let Some(i) = self.m3.iter().position(|&v| v == value) {
            self.m3.swap_remove(i);
        }
    }

    pub fn remove_m4(&mut self, value: M4) {
        if let Some(i) = self.m4.iter().position(|&v| v == value) {
            self.m4.swap_remove(i);
        }
    }

    pub fn remove_m5(&mut self, value: M5) {
        if let Some(i) = self.m5.iter().position(|&v| v == value) {
            self.m5.swap_remove(i);
        }
    }

    pub fn remove_m6(&mut self, value: M6) {
        if let Some(i) = self.m6.iter().position(|&v| v == value) {
            self.m6.swap_remove(i);
        }
    }

    pub fn remove_m7(&mut self, value: M7) {
        if let Some(i) = self.m7.iter().position(|&v| v == value) {
            self.m7.swap_remove(i);
        }
    }

    pub fn remove_m8(&mut self, value: M8) {
        if let Some(i) = self.m8.iter().position(|&v| v == value) {
            self.m8.swap_remove(i);
        }
    }

    pub fn has_m1(&self, value: M1) -> bool {
        self.m1.iter().any(|&v| v == value)
    }

    pub fn has_m2(&self, value: M2) -> bool {
        self.m2.iter().any(|&v| v == value)
    }

    pub fn has_m3(&self, value: M3) -> bool {
        self.m3.iter().any(|&v| v == value)
    }

    pub fn has_m4(&self, value: M4) -> bool {
        self.m4.iter().any(|&v| v == value)
    }

    pub fn has_m5(&self, value: M5) -> bool {
        self.m5.iter().any(|&v| v == value)
    }

    pub fn has_m6(&self, value: M6) -> bool {
        self.m6.iter().any(|&v| v == value)
    }

    pub fn has_m7(&self, value: M7) -> bool {
        self.m7.iter().any(|&v| v == value)
    }

    pub fn has_m8(&self, value: M8) -> bool {
        self.m8.iter().any(|&v| v == value)
    }

    pub fn base(&self) -> Marker::Raw {
        self.base
    }

    pub fn get(&self) -> Marker::Raw {
        (self.f)(
            self.base, &self.m1, &self.m2, &self.m3, &self.m4, &self.m5, &self.m6, &self.m7,
            &self.m8,
        )
    }

    pub fn m1(&self) -> &SmallVec<[M1; N]> {
        &self.m1
    }

    pub fn m2(&self) -> &SmallVec<[M2; N]> {
        &self.m2
    }

    pub fn m3(&self) -> &SmallVec<[M3; N]> {
        &self.m3
    }

    pub fn m4(&self) -> &SmallVec<[M4; N]> {
        &self.m4
    }

    pub fn m5(&self) -> &SmallVec<[M5; N]> {
        &self.m5
    }

    pub fn m6(&self) -> &SmallVec<[M6; N]> {
        &self.m6
    }

    pub fn m7(&self) -> &SmallVec<[M7; N]> {
        &self.m7
    }

    pub fn m8(&self) -> &SmallVec<[M8; N]> {
        &self.m8
    }
}

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

        #[derive(Clone, Copy, PartialEq)]
        struct DummyModifier {
            raw: f32,
        }

        impl Modifier for DummyModifier {
            type Target = DummyMarker;

            fn from_raw(raw: <<Self as Modifier>::Target as StatMarker>::Raw) -> Self {
                Self { raw }
            }

            fn raw(&self) -> <<Self as Modifier>::Target as StatMarker>::Raw {
                self.raw
            }

            fn combine(s: &[Self]) -> <<Self as Modifier>::Target as StatMarker>::Raw {
                s.iter().fold(0., |acc, m| acc + m.raw)
            }
        }

        let mut stat = Stat1::<DummyMarker, DummyModifier>::new(
            1.,
            Box::new(|b, v| b + DummyModifier::combine(&v)),
        );
        stat.apply_m1(DummyModifier::from_raw(1.));
        stat.apply_m1(DummyModifier::from_raw(2.));
        stat.apply_m1(DummyModifier::from_raw(3.));

        assert_eq!(7., stat.get());
    }
}
