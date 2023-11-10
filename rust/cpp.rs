// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Rust Protobuf runtime using the C++ kernel.

use crate::__internal::{Private, RawArena, RawMessage, RawRepeatedField};
use paste::paste;
use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::{self, NonNull};

/// A wrapper over a `proto2::Arena`.
///
/// This is not a safe wrapper per se, because the allocation functions still
/// have sharp edges (see their safety docs for more info).
///
/// This is an owning type and will automatically free the arena when
/// dropped.
///
/// Note that this type is neither `Sync` nor `Send`.
#[derive(Debug)]
pub struct Arena {
    #[allow(dead_code)]
    ptr: RawArena,
    _not_sync: PhantomData<UnsafeCell<()>>,
}

impl Arena {
    /// Allocates a fresh arena.
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { ptr: NonNull::dangling(), _not_sync: PhantomData }
    }

    /// Returns the raw, C++-managed pointer to the arena.
    #[inline]
    pub fn raw(&self) -> ! {
        unimplemented!()
    }

    /// Allocates some memory on the arena.
    ///
    /// # Safety
    ///
    /// TODO alignment requirement for layout
    #[inline]
    pub unsafe fn alloc(&self, _layout: Layout) -> &mut [MaybeUninit<u8>] {
        unimplemented!()
    }

    /// Resizes some memory on the arena.
    ///
    /// # Safety
    ///
    /// After calling this function, `ptr` is essentially zapped. `old` must
    /// be the layout `ptr` was allocated with via [`Arena::alloc()`].
    /// TODO alignment for layout
    #[inline]
    pub unsafe fn resize(&self, _ptr: *mut u8, _old: Layout, _new: Layout) -> &[MaybeUninit<u8>] {
        unimplemented!()
    }
}

impl Drop for Arena {
    #[inline]
    fn drop(&mut self) {
        // unimplemented
    }
}

/// Serialized Protobuf wire format data. It's typically produced by
/// `<Message>.serialize()`.
///
/// This struct is ABI-compatible with the equivalent struct on the C++ side. It
/// owns (and drops) its data.
#[repr(C)]
pub struct SerializedData {
    /// Owns the memory.
    data: NonNull<u8>,
    len: usize,
}

impl SerializedData {
    /// Constructs owned serialized data from raw components.
    ///
    /// # Safety
    /// - `data` must be readable for `len` bytes.
    /// - `data` must be an owned pointer and valid until deallocated.
    /// - `data` must have been allocated by the Rust global allocator with a
    ///   size of `len` and align of 1.
    pub unsafe fn from_raw_parts(data: NonNull<u8>, len: usize) -> Self {
        Self { data, len }
    }

    /// Gets a raw slice pointer.
    pub fn as_ptr(&self) -> *const [u8] {
        ptr::slice_from_raw_parts(self.data.as_ptr(), self.len)
    }

    /// Gets a mutable raw slice pointer.
    fn as_mut_ptr(&mut self) -> *mut [u8] {
        ptr::slice_from_raw_parts_mut(self.data.as_ptr(), self.len)
    }
}

impl Deref for SerializedData {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        // SAFETY: `data` is valid for `len` bytes until deallocated as promised by
        // `from_raw_parts`.
        unsafe { &*self.as_ptr() }
    }
}

impl Drop for SerializedData {
    fn drop(&mut self) {
        // SAFETY: `data` was allocated by the Rust global allocator with a
        // size of `len` and align of 1 as promised by `from_raw_parts`.
        unsafe { drop(Box::from_raw(self.as_mut_ptr())) }
    }
}

impl fmt::Debug for SerializedData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.deref(), f)
    }
}

pub type BytesPresentMutData<'msg> = crate::vtable::RawVTableOptionalMutatorData<'msg, [u8]>;
pub type BytesAbsentMutData<'msg> = crate::vtable::RawVTableOptionalMutatorData<'msg, [u8]>;
pub type InnerBytesMut<'msg> = crate::vtable::RawVTableMutator<'msg, [u8]>;
pub type InnerPrimitiveMut<'a, T> = crate::vtable::RawVTableMutator<'a, T>;

/// The raw contents of every generated message.
#[derive(Debug)]
pub struct MessageInner {
    pub msg: RawMessage,
}

/// Mutators that point to their original message use this to do so.
///
/// Since C++ messages manage their own memory, this can just copy the
/// `RawMessage` instead of referencing an arena like UPB must.
///
/// Note: even though this type is `Copy`, it should only be copied by
/// protobuf internals that can maintain mutation invariants:
///
/// - No concurrent mutation for any two fields in a message: this means
///   mutators cannot be `Send` but are `Sync`.
/// - If there are multiple accessible `Mut` to a single message at a time, they
///   must be different fields, and not be in the same oneof. As such, a `Mut`
///   cannot be `Clone` but *can* reborrow itself with `.as_mut()`, which
///   converts `&'b mut Mut<'a, T>` to `Mut<'b, T>`.
#[derive(Clone, Copy, Debug)]
pub struct MutatorMessageRef<'msg> {
    msg: RawMessage,
    _phantom: PhantomData<&'msg mut ()>,
}
impl<'msg> MutatorMessageRef<'msg> {
    #[allow(clippy::needless_pass_by_ref_mut)] // Sound construction requires mutable access.
    pub fn new(_private: Private, msg: &'msg mut MessageInner) -> Self {
        MutatorMessageRef { msg: msg.msg, _phantom: PhantomData }
    }

    pub fn new_raw(_private: Private, msg: RawMessage) -> Self {
        MutatorMessageRef { msg, _phantom: PhantomData }
    }

    pub fn msg(&self) -> RawMessage {
        self.msg
    }
}

pub fn copy_bytes_in_arena_if_needed_by_runtime<'a>(
    _msg_ref: MutatorMessageRef<'a>,
    val: &'a [u8],
) -> &'a [u8] {
    // Nothing to do, the message manages its own string memory for C++.
    val
}

/// RepeatedField impls delegate out to `extern "C"` functions exposed by
/// `cpp_api.h` and store either a RepeatedField* or a RepeatedPtrField*
/// depending on the type.
///
/// Note: even though this type is `Copy`, it should only be copied by
/// protobuf internals that can maintain mutation invariants:
///
/// - No concurrent mutation for any two fields in a message: this means
///   mutators cannot be `Send` but are `Sync`.
/// - If there are multiple accessible `Mut` to a single message at a time, they
///   must be different fields, and not be in the same oneof. As such, a `Mut`
///   cannot be `Clone` but *can* reborrow itself with `.as_mut()`, which
///   converts `&'b mut Mut<'a, T>` to `Mut<'b, T>`.
#[derive(Debug)]
pub struct RepeatedField<'msg, T: ?Sized> {
    inner: RepeatedFieldInner<'msg>,
    _phantom: PhantomData<&'msg mut T>,
}

/// CPP runtime-specific arguments for initializing a RepeatedField.
/// See RepeatedField comment about mutation invariants for when this type can
/// be copied.
#[derive(Clone, Copy, Debug)]
pub struct RepeatedFieldInner<'msg> {
    pub raw: RawRepeatedField,
    pub _phantom: PhantomData<&'msg ()>,
}

impl<'msg, T: ?Sized> RepeatedField<'msg, T> {
    pub fn from_inner(_private: Private, inner: RepeatedFieldInner<'msg>) -> Self {
        RepeatedField { inner, _phantom: PhantomData }
    }
    pub fn raw(&self) -> RawRepeatedField {
        self.inner.raw
    }
    pub fn inner(&self) -> RepeatedFieldInner<'msg> {
        self.inner
    }
}

// These use manual impls instead of derives to avoid unnecessary bounds on `T`.
// This problem is referred to as "perfect derive".
// https://smallcultfollowing.com/babysteps/blog/2022/04/12/implied-bounds-and-perfect-derive/
impl<'msg, T: ?Sized> Copy for RepeatedField<'msg, T> {}
impl<'msg, T: ?Sized> Clone for RepeatedField<'msg, T> {
    fn clone(&self) -> RepeatedField<'msg, T> {
        *self
    }
}

pub trait RepeatedScalarOps {
    fn new_repeated_field() -> RawRepeatedField;
    fn push(f: RawRepeatedField, v: Self);
    fn len(f: RawRepeatedField) -> usize;
    fn get(f: RawRepeatedField, i: usize) -> Self;
    fn set(f: RawRepeatedField, i: usize, v: Self);
    fn copy_from(src: RawRepeatedField, dst: RawRepeatedField);
}

macro_rules! impl_repeated_scalar_ops {
    ($($t: ty),*) => {
        paste! { $(
            extern "C" {
                fn [< __pb_rust_RepeatedField_ $t _new >]() -> RawRepeatedField;
                fn [< __pb_rust_RepeatedField_ $t _add >](f: RawRepeatedField, v: $t);
                fn [< __pb_rust_RepeatedField_ $t _size >](f: RawRepeatedField) -> usize;
                fn [< __pb_rust_RepeatedField_ $t _get >](f: RawRepeatedField, i: usize) -> $t;
                fn [< __pb_rust_RepeatedField_ $t _set >](f: RawRepeatedField, i: usize, v: $t);
                fn [< __pb_rust_RepeatedField_ $t _copy_from >](src: RawRepeatedField, dst: RawRepeatedField);
            }
            impl RepeatedScalarOps for $t {
                fn new_repeated_field() -> RawRepeatedField {
                    unsafe { [< __pb_rust_RepeatedField_ $t _new >]() }
                }
                fn push(f: RawRepeatedField, v: Self) {
                    unsafe { [< __pb_rust_RepeatedField_ $t _add >](f, v) }
                }
                fn len(f: RawRepeatedField) -> usize {
                    unsafe { [< __pb_rust_RepeatedField_ $t _size >](f) }
                }
                fn get(f: RawRepeatedField, i: usize) -> Self {
                    unsafe { [< __pb_rust_RepeatedField_ $t _get >](f, i) }
                }
                fn set(f: RawRepeatedField, i: usize, v: Self) {
                    unsafe { [< __pb_rust_RepeatedField_ $t _set >](f, i, v) }
                }
                fn copy_from(src: RawRepeatedField, dst: RawRepeatedField) {
                    unsafe { [< __pb_rust_RepeatedField_ $t _copy_from >](src, dst) }
                }
            }
        )* }
    };
}

impl_repeated_scalar_ops!(i32, u32, i64, u64, f32, f64, bool);

impl<'msg, T: RepeatedScalarOps> RepeatedField<'msg, T> {
    #[allow(clippy::new_without_default, dead_code)]
    /// new() is not currently used in our normal pathways, it is only used
    /// for testing. Existing `RepeatedField<>`s are owned by, and retrieved
    /// from, the containing `Message`.
    pub fn new() -> Self {
        Self::from_inner(
            Private,
            RepeatedFieldInner::<'msg> { raw: T::new_repeated_field(), _phantom: PhantomData },
        )
    }
    pub fn push(&mut self, val: T) {
        T::push(self.inner.raw, val)
    }
    pub fn len(&self) -> usize {
        T::len(self.inner.raw)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        Some(T::get(self.inner.raw, index))
    }
    pub fn set(&mut self, index: usize, val: T) {
        if index >= self.len() {
            return;
        }
        T::set(self.inner.raw, index, val)
    }
    pub fn copy_from(&mut self, src: &RepeatedField<'_, T>) {
        T::copy_from(src.inner.raw, self.inner.raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::boxed::Box;

    // We need to allocate the byte array so SerializedData can own it and
    // deallocate it in its drop. This function makes it easier to do so for our
    // tests.
    fn allocate_byte_array(content: &'static [u8]) -> (*mut u8, usize) {
        let content: &mut [u8] = Box::leak(content.into());
        (content.as_mut_ptr(), content.len())
    }

    #[test]
    fn test_serialized_data_roundtrip() {
        let (ptr, len) = allocate_byte_array(b"Hello world");
        let serialized_data = SerializedData { data: NonNull::new(ptr).unwrap(), len: len };
        assert_eq!(&*serialized_data, b"Hello world");
    }

    #[test]
    fn repeated_field() {
        let mut r = RepeatedField::<i32>::new();
        assert_eq!(r.len(), 0);
        r.push(32);
        assert_eq!(r.get(0), Some(32));

        let mut r = RepeatedField::<u32>::new();
        assert_eq!(r.len(), 0);
        r.push(32);
        assert_eq!(r.get(0), Some(32));

        let mut r = RepeatedField::<f64>::new();
        assert_eq!(r.len(), 0);
        r.push(0.1234f64);
        assert_eq!(r.get(0), Some(0.1234));

        let mut r = RepeatedField::<bool>::new();
        assert_eq!(r.len(), 0);
        r.push(true);
        assert_eq!(r.get(0), Some(true));
    }
}
