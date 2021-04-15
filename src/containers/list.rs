use crate::containers::SharedList;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;

    use super::TakeFirst;
    impl<T: Clone> TakeFirst<T> for List<T> {
        fn take_first(self) -> T {
            self.into_iter()
                .next()
                .expect("expected at least 1 element")
        }
    }

    use super::{AsSharedList, SharedList};
    impl<T> AsSharedList<T> for List<T> {
        fn shared(&self) -> SharedList<T> {
            &self
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    /// C-compatible list
    #[repr(C)]
    pub struct List<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }

            if self.len != 0 {
                unsafe {
                    // propagate Drop on items
                    std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(self.ptr, self.len));
                }
            }

            if self.capacity != 0 {
                unsafe {
                    // free memory
                    let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                    std::alloc::Global
                        .deallocate(std::ptr::NonNull::new(self.ptr as *mut u8).unwrap(), layout);
                }
            }

            self.ptr = std::ptr::null_mut();
            self.len = 0;
            self.capacity = 0;
        }
    }

    impl<T> std::fmt::Debug for List<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for List<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl<T> PartialEq<&[T]> for List<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &&[T]) -> bool {
            self.as_ref() == *other
        }
    }

    impl<T> Clone for List<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            let copied = self.as_ref().iter().map(|e| e.clone()).collect::<Vec<_>>();
            Self::from(copied)
        }
    }

    impl<T> From<Vec<T>> for List<T>
    where
        T: Clone,
    {
        fn from(mut vec: Vec<T>) -> Self {
            let ptr = vec.as_mut_ptr();
            let len = vec.len();
            let capacity = vec.capacity();
            std::mem::forget(vec);
            Self { ptr, len, capacity }
        }
    }

    impl<T> From<List<T>> for Vec<T>
    where
        T: Clone,
    {
        fn from(mut list: List<T>) -> Self {
            let ptr = list.ptr;
            let len = list.len;
            let capacity = list.capacity;
            list.ptr = std::ptr::null_mut();
            list.len = 0;
            list.capacity = 0;
            unsafe { Vec::from_raw_parts(ptr, len, capacity) }
        }
    }

    impl From<String> for List<u8> {
        fn from(s: String) -> Self {
            Self::from(s.into_bytes())
        }
    }

    impl From<&str> for List<u8> {
        fn from(s: &str) -> Self {
            List::from(s.to_string())
        }
    }

    impl From<&String> for List<u8> {
        fn from(s: &String) -> Self {
            List::from(s.clone())
        }
    }

    impl From<&[u8]> for List<u8> {
        fn from(bytes: &[u8]) -> Self {
            Self::from(bytes.to_vec())
        }
    }

    impl<T> List<T> {
        /// Equivalent of Vec::new
        pub fn new() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        }

        /// Equivalent of Vec::is_empty
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Equivalent of Vec::iter
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.as_ref().iter()
        }

        /// Equivalent of Vec::with_capacity
        pub fn with_capacity(capacity: usize) -> Self {
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            let ptr = std::alloc::Global.allocate(layout).unwrap().as_mut_ptr() as *mut T;
            Self {
                ptr,
                len: 0,
                capacity,
            }
        }

        fn grow(&mut self) {
            let prev_capacity = self.capacity;
            let prev_layout = std::alloc::Layout::array::<T>(prev_capacity).unwrap();
            let prev_ptr = self.ptr;

            let new_capacity = if prev_capacity == 0 {
                1
            } else {
                prev_capacity * 2
            };
            let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
            let new_ptr;

            // allocate space for new data
            new_ptr = std::alloc::Global
                .allocate(new_layout)
                .unwrap()
                .as_mut_ptr() as *mut T;

            if self.len > 0 {
                unsafe {
                    // copy data
                    std::ptr::copy(prev_ptr, new_ptr, self.len);

                    // free old data
                    std::alloc::Global.deallocate(
                        std::ptr::NonNull::new(prev_ptr as *mut u8).unwrap(),
                        prev_layout,
                    );
                };
            }

            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }

        /// Equivalent of Vec::push
        pub fn push(&mut self, item: T) {
            if self.len == self.capacity {
                self.grow()
            }
            unsafe {
                let end = self.ptr.add(self.len);
                end.write(item);
                self.len += 1;
            }
        }

        /// Equivalent of Vec::remove
        pub fn remove(&mut self, index: usize) -> T {
            if index > self.len {
                panic!("can't remove index {}, len is {}", index, self.len)
            }
            unsafe {
                let ptr = self.ptr.add(index);
                let result = ptr.read();
                std::ptr::copy(ptr.offset(1), ptr, self.len - index - 1);
                self.len -= 1;
                result
            }
        }

        /// Helper for swapping &mut self with Self::default()
        pub fn take(&mut self) -> Self {
            std::mem::take(self)
        }
    }

    impl<T> std::ops::Deref for List<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    impl<T> std::ops::DerefMut for List<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
        }
    }

    impl<T, I: std::slice::SliceIndex<[T]>> std::ops::Index<I> for List<T> {
        type Output = I::Output;

        fn index(&self, index: I) -> &Self::Output {
            std::ops::Index::index(&**self, index)
        }
    }

    impl<T> Default for List<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    use std::alloc::Allocator;

    use super::TakeFirst;
    impl<T: Clone> TakeFirst<T> for List<T> {
        fn take_first(self) -> T {
            if self.is_empty() {
                panic!("can't get the first item from an empty list")
            } else {
                unsafe { self.ptr.as_ref() }.unwrap().clone()
            }
        }
    }

    use super::{AsSharedList, SharedList};
    impl<T> AsSharedList<T> for List<T> {
        fn shared(&self) -> SharedList<T> {
            SharedList::from_raw(self.ptr, self.len)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{List as GenericList, TakeFirst};
        type List = GenericList<usize>;

        #[test]
        fn test_new() {
            let list = List::new();
            assert_eq!(list.len, 0);
            assert_eq!(list.capacity, 0);
        }

        #[test]
        fn test_with_capacity() {
            let list = List::with_capacity(20);
            assert_eq!(list.len, 0);
            assert_eq!(list.capacity, 20);
        }

        #[test]
        fn test_push() {
            let mut list = List::new();
            let mut vec = vec![];
            for i in 1..20 {
                list.push(i);
                vec.push(i);
            }
            assert_eq!(list.as_ref(), &vec);
        }

        #[test]
        fn test_take_first() {
            let mut list = List::new();
            list.push(10);
            list.push(20);
            assert_eq!(list.take_first(), 10)
        }

        #[test]
        fn test_from_vec() {
            let list = List::from(vec![1, 2, 3]);
            assert_eq!(list.as_ref(), &[1, 2, 3])
        }
    }
}

pub(crate) trait TakeFirst<T: Clone> {
    fn take_first(self) -> T;
}

pub(crate) trait AsSharedList<T> {
    fn shared(&self) -> SharedList<T>;
}
