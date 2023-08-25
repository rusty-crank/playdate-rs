use core::{marker::PhantomData, mem::ManuallyDrop};

/// A handle to hold playdate internal allocated objects.
///
/// TODO:
/// There is a case that the user passes a `&T` to the system, and get a `Ref<T>` back using a different getter.
/// e.g. `PLAYDATE.graphics.set_stencil`
/// Need to carefully review each of the case and make sure `T` instead of `&T` is passed to the system, and the old `T` is correctly destroyed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref<'a, T: Sized> {
    wrapper: ManuallyDrop<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Sized> Ref<'a, T> {
    pub(crate) fn from(wrapper: T) -> Self {
        Self {
            wrapper: ManuallyDrop::new(wrapper),
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Sized> core::ops::Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wrapper
    }
}

impl<'a, T: Sized> AsRef<T> for Ref<'a, T> {
    fn as_ref(&self) -> &T {
        &self.wrapper
    }
}
