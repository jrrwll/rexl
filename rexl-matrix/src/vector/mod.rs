mod ops;
mod iter;

use std::ops::*;
use std::{cmp, fmt};
use crate::{Element, NumericElement};
use self::iter::{VectorIterator, VectorIteratorMut};
use std::fmt::{Display, Formatter, Debug};

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<T: Element> {
    pub data: Vec<T>,
}

impl<T: Element> From<Vec<T>> for Vector<T> {
    #[inline]
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}

impl <T: Element> Into<Vec<T>> for Vector<T> {
    #[inline]
    fn into(self) -> Vec<T> {
        self.data
    }
}

impl<T: Element> Deref for Vector<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<T: Element> DerefMut for Vector<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}

impl <T: Element> Display for Vector<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T: Element> Vector<T> {
    #[inline]
    pub fn new(data: Vec<T>) -> Self {
        Vector { data }
    }

    #[inline]
    pub fn zero(size: usize) -> Self {
        Self::new(vec![T::zero(); size])
    }

    #[inline]
    pub fn from_vec(data: Vec<T>) -> Self {
        Self::new(data)
    }

    #[inline]
    pub fn from_slice(s: &[T]) -> Self {
        Self::new(s.to_vec())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn iter(&self) -> VectorIterator<T> {
        let size = self.len();
        VectorIterator {
            vector: self,
            taken: 0,
            size,
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> VectorIteratorMut<T> {
        let size = self.len();
        VectorIteratorMut {
            vector: self,
            taken: 0,
            size,
        }
    }
}

impl<T: NumericElement + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>> Vector<T> {

    pub fn dot_mul(&self, other: &Self) -> T {
        let n = cmp::min(self.len(), other.len());
        let mut sum = T::zero();
        for i in 0..n {
            sum = sum + self[i] * other[i];
        }
        sum
    }

    #[inline]
    pub fn sum(&self) -> T {
        self.iter().fold(T::zero(), |sum, &elem| sum + elem)
    }

    #[inline]
    pub fn product(&self) -> T {
        self.iter().fold(T::one(), |sum, &elem| sum * elem)
    }

    #[inline]
    pub fn average(&self) -> T {
        self.sum() / T::from_usize(self.len())
    }

    /// (sum(i - avg) ^ 2) / len
    pub fn variance(&self) -> T {
        let avg = self.average();
        self.iter().fold(T::zero(), |sum, &elem|
            sum + (elem - avg) * (elem - avg)) / T::from_usize(self.len())
    }

    /// static methods

    pub fn eq_diff(a0: T, diff: T, size: usize) -> Self {
        let mut y = Self::zero(size);
        y[0] = a0;
        for i in 1..size {
            // y[i] = a0 + i * d;
            y[i] = a0 + T::from_usize(i) * diff;
        }
        y
    }

    pub fn eq_diff_at(ai: T, index: usize, diff: T, size: usize) -> Self {
        let mut y = Self::zero(size);
        y[index] = ai;
        for i in 0..size {
            if i == index { continue; }
            if i > index {
                y[i] = ai + T::from_usize(i - index) * diff;
            } else {
                y[i] = ai - T::from_usize(index - i) * diff;
            }
        }
        y
    }

    pub fn eq_prop(a0: T, prop: T, size: usize) -> Self {
        let mut y = Self::zero(size);
        y[0] = a0;
        let mut prev = a0;
        for i in 1..size {
            prev = prev * prop;
            y[i] = prev;
        }
        y
    }

    pub fn eq_prop_at(ai: T, index: usize, prop: T, size: usize) -> Self {
        let mut y = Self::zero(size);
        y[index] = ai;
        let (mut prev, mut next) = (ai, ai);
        for i in (0..index).rev() {
            // y[i] = y[i + 1] / q;
            next = next / prop;
            y[i] = next;
        }
        for i in index + 1..size {
            // y[i] = y[i - 1] * q;
            prev = prev * prop;
            y[i] = prev;
        }
        y
    }

    pub fn line_sq(start: T, stop: T, size: usize) -> Self {
        let mut y = Self::zero(size);
        y[0] = start;
        y[size - 1] = stop;
        let step = (stop - start) / T::from_usize(size - 1);
        let mut prev = start;
        for i in 1..size - 1 {
            prev = prev + step;
            y[i] = prev;
        }
        y
    }

    /// Note that it could be overflow when T is a integer
    pub fn line_sq_weight<F: Fn(f64) -> T>(start: T, stop: T, weight: Vec<f64>, cast: F) -> Self {
        let size = weight.len() + 1;
        let mut y = Self::zero(size);
        y[0] = start;
        y[size - 1] = stop;

        let diff = stop - start;
        let s: f64 = weight.iter().sum();
        let mut prev = start;
        for i in 1..size - 1 {
            prev = prev + diff * cast(weight[i - 1]) / cast(s);
            y[i] = prev;
        }
        y
    }
}

impl Vector<f64> {
    /// sum(i^n) ^ (1/n)
    pub fn norm(&self, base: f64) -> f64 {
        let sum: f64 = self.iter().map(|&elem| elem.powf(base)).sum();
        sum.powf(1.0 / base)
    }
}