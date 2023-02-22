use crate::{
    shapes::*,
    tensor::{CopySlice, DeviceStorage, Tensor, ZerosTensor},
};

use std::vec::Vec;

/// Generates a tensor with ordered data from 0 to `N`.
pub trait Arange<E: Dtype>: DeviceStorage + ZerosTensor<E> + CopySlice<E> {
    /// Generates a tensor with ordered data from 0 to `N`.
    ///
    /// Const sized tensor:
    /// ```rust
    /// # use dfdx::{prelude::*, data::Arange};
    /// # let dev: Cpu = Default::default();
    /// let t: Tensor<Rank1<5>, f32, _> = dev.arange(Const::<5>);
    /// assert_eq!(t.array(), [0.0, 1.0, 2.0, 3.0, 4.0]);
    /// ```
    ///
    /// Runtime sized tensor:
    /// ```rust
    /// # use dfdx::{prelude::*, data::Arange};
    /// # let dev: Cpu = Default::default();
    /// let t: Tensor<(usize, ), f32, _> = dev.arange(5);
    /// assert_eq!(t.as_vec(), [0.0, 1.0, 2.0, 3.0, 4.0]);
    /// ```
    fn arange<Size: Dim>(&self, n: Size) -> Tensor<(Size,), E, Self> {
        let mut data = Vec::with_capacity(n.size());
        for i in 0..n.size() {
            data.push(E::from_usize(i).unwrap());
        }
        let mut t = self.zeros_like(&(n,));
        t.copy_from(&data);
        t
    }
}
impl<E: Dtype, D: ZerosTensor<E> + CopySlice<E>> Arange<E> for D {}