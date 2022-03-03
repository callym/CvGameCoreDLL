//! This file exists to give us access to the system allocator.
//! We have to build this crate in `#[no_std]` mode because we're linking it into a DLL compiled with VS 2003,
//! which is older than any MSVC version that Rust supports, but we also DO want to use the `alloc` crate!

pub use core::alloc::*;
use core::{cmp, intrinsics, ptr, ptr::NonNull};

const MIN_ALIGN: usize = 8;

#[global_allocator]
static GLOBAL: System = System;

#[derive(Debug, Default, Copy, Clone)]
struct System;

impl System {
  #[inline]
  fn alloc_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
    match layout.size() {
      0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
      // SAFETY: `layout` is non-zero in size,
      size => unsafe {
        let raw_ptr = if zeroed {
          GlobalAlloc::alloc_zeroed(self, layout)
        } else {
          GlobalAlloc::alloc(self, layout)
        };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, size))
      },
    }
  }

  // SAFETY: Same as `Allocator::grow`
  #[inline]
  unsafe fn grow_impl(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
    zeroed: bool,
  ) -> Result<NonNull<[u8]>, AllocError> {
    debug_assert!(
      new_layout.size() >= old_layout.size(),
      "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
    );

    match old_layout.size() {
      0 => self.alloc_impl(new_layout, zeroed),

      // SAFETY: `new_size` is non-zero as `new_size` is greater than or equal to `old_size`
      // as required by safety conditions and the `old_size == 0` case was handled in the
      // previous match arm. Other conditions must be upheld by the caller
      old_size if old_layout.align() == new_layout.align() => unsafe {
        let new_size = new_layout.size();

        // `realloc` probably checks for `new_size >= old_layout.size()` or something similar.
        intrinsics::assume(new_size >= old_layout.size());

        let raw_ptr = GlobalAlloc::realloc(self, ptr.as_ptr(), old_layout, new_size);
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        if zeroed {
          raw_ptr.add(old_size).write_bytes(0, new_size - old_size);
        }
        Ok(NonNull::slice_from_raw_parts(ptr, new_size))
      },

      // SAFETY: because `new_layout.size()` must be greater than or equal to `old_size`,
      // both the old and new memory allocation are valid for reads and writes for `old_size`
      // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
      // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
      // for `dealloc` must be upheld by the caller.
      old_size => unsafe {
        let new_ptr = self.alloc_impl(new_layout, zeroed)?;
        ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), old_size);
        Allocator::deallocate(&self, ptr, old_layout);
        Ok(new_ptr)
      },
    }
  }
}

unsafe impl Allocator for System {
  #[inline]
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    self.alloc_impl(layout, false)
  }

  #[inline]
  fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    self.alloc_impl(layout, true)
  }

  #[inline]
  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    if layout.size() != 0 {
      // SAFETY: `layout` is non-zero in size,
      // other conditions must be upheld by the caller
      unsafe { GlobalAlloc::dealloc(self, ptr.as_ptr(), layout) }
    }
  }

  #[inline]
  unsafe fn grow(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    // SAFETY: all conditions must be upheld by the caller
    unsafe { self.grow_impl(ptr, old_layout, new_layout, false) }
  }

  #[inline]
  unsafe fn grow_zeroed(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    // SAFETY: all conditions must be upheld by the caller
    unsafe { self.grow_impl(ptr, old_layout, new_layout, true) }
  }

  #[inline]
  unsafe fn shrink(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
  ) -> Result<NonNull<[u8]>, AllocError> {
    debug_assert!(
      new_layout.size() <= old_layout.size(),
      "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
    );

    match new_layout.size() {
      // SAFETY: conditions must be upheld by the caller
      0 => unsafe {
        Allocator::deallocate(&self, ptr, old_layout);
        Ok(NonNull::slice_from_raw_parts(new_layout.dangling(), 0))
      },

      // SAFETY: `new_size` is non-zero. Other conditions must be upheld by the caller
      new_size if old_layout.align() == new_layout.align() => unsafe {
        // `realloc` probably checks for `new_size <= old_layout.size()` or something similar.
        intrinsics::assume(new_size <= old_layout.size());

        let raw_ptr = GlobalAlloc::realloc(self, ptr.as_ptr(), old_layout, new_size);
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, new_size))
      },

      // SAFETY: because `new_size` must be smaller than or equal to `old_layout.size()`,
      // both the old and new memory allocation are valid for reads and writes for `new_size`
      // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
      // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
      // for `dealloc` must be upheld by the caller.
      new_size => unsafe {
        let new_ptr = Allocator::allocate(&self, new_layout)?;
        ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_mut_ptr(), new_size);
        Allocator::deallocate(&self, ptr, old_layout);
        Ok(new_ptr)
      },
    }
  }
}

unsafe impl GlobalAlloc for System {
  #[inline]
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    unsafe { libc::malloc(layout.size()) as *mut u8 }
  }

  #[inline]
  unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    unsafe {
      // See the comment above in `alloc` for why this check looks the way it does.
      if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
        libc::calloc(layout.size(), 1) as *mut u8
      } else {
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
          ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr
      }
    }
  }

  #[inline]
  unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
    unsafe { libc::free(ptr as *mut libc::c_void) }
  }

  #[inline]
  unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe {
      if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
        libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
      } else {
        realloc_fallback(self, ptr, layout, new_size)
      }
    }
  }
}

unsafe fn realloc_fallback(
  alloc: &System,
  ptr: *mut u8,
  old_layout: Layout,
  new_size: usize,
) -> *mut u8 {
  unsafe {
    // Docs for GlobalAlloc::realloc require this to be valid:
    let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

    let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
    if !new_ptr.is_null() {
      let size = cmp::min(old_layout.size(), new_size);
      ptr::copy_nonoverlapping(ptr, new_ptr, size);
      GlobalAlloc::dealloc(alloc, ptr, old_layout);
    }
    new_ptr
  }
}

#[alloc_error_handler]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// cbindgen:ignore
pub extern "C" fn oom(_: core::alloc::Layout) -> ! {
  core::intrinsics::abort();
}
