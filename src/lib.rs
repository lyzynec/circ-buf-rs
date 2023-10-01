#![cfg_attr(not(test), no_std)]

#[derive(Debug, Copy, Clone)]
pub struct CircBuf<T: Copy, const S: usize> {
  first: usize,
  len: usize,
  initial: T,
  data: [T; S],
}

// new
impl<T: Copy, const S: usize> CircBuf<T, S> {

  pub fn new(initial: T) -> CircBuf<T, S> {
    return CircBuf {
      first: 0,
      len: 0,
      initial: initial,
      data: [initial; S],
    };
  }

}

// push
impl<T: Copy, const S: usize> CircBuf<T, S> {

  pub fn push_back(&mut self, new: T) -> Result<(), ()> {

    if self.len >= S { return Result::Err(()); }

    let index = capped_add(self.first, self.len, S);
    self.data[index] = new;

    self.len += 1;
    return Ok(());
  }

  pub fn push_front(&mut self, new: T) -> Result<(), ()> {

    if self.len >= S { return Result::Err(()); }

    self.first = capped_sub(self.first, 1, S);

    self.data[self.first] = new;
    self.len += 1;
    return Ok(());
  }

  pub unsafe fn push_back_unchecked(&mut self, new: T) {
    let index = capped_add(self.first, self.len, S);
    self.data[index] = new;
    self.len += 1;
  }

  pub unsafe fn push_front_unchecked(&mut self, new: T) {
    self.first = capped_sub(self.first, 1, S);
    self.data[self.first] = new;
    self.len += 1;
  }

}

// pop
impl<T: Copy, const S: usize> CircBuf<T, S> {
  
  pub fn pop_back(&mut self) -> Option<T> {

    if self.len < 1 { return None }
    self.len -= 1;
    let index = capped_add(self.first, self.len, S);

    let out = self.data[index];

    return Some(out);
  }

  pub fn pop_front(&mut self) -> Option<T> {

    if self.len < 1 { return None }

    let out = self.data[self.first];
    self.first = capped_add(self.first, 1, S);
    self.len -= 1;

    return Some(out);
  }

  pub unsafe fn pop_back_unchecked(&mut self) -> T {
    self.len -= 1;
    let index = capped_add(self.first, self.len, S);
    return self.data[index];
  }

  pub unsafe fn pop_front_unchecked(&mut self) -> T {
    let out = self.data[self.first];
    self.first = capped_add(self.first, 1, S);
    self.len -= 1;
    return out;
  }

}

// first
impl<T: Copy, const S: usize> CircBuf<T, S> {

  pub fn first(&self) -> Option<&T> {
    if self.len < 1 { return None }
    return unsafe { Some(self.data.get_unchecked(self.first)) };
  }

  pub fn first_mut(&mut self) -> Option<&mut T> {
    if self.len < 1 { return None }
    return unsafe { Some(self.data.get_unchecked_mut(self.first)) };
  }

  pub unsafe fn first_unchecked(&self) -> &T {
    return self.data.get_unchecked(self.first);
  }

  pub unsafe fn first_unchecked_mut(&mut self) -> &mut T {
    return self.data.get_unchecked_mut(self.first);
  }

}

// last
impl<T: Copy, const S: usize> CircBuf<T, S> {

  pub fn last(&self) -> Option<&T> {
    if self.len < 1 { return None }
    let index = capped_add(self.first, self.len-1, S);
    return unsafe { Some(self.data.get_unchecked(index)) };
  }

  pub fn last_mut(&mut self) -> Option<&mut T> {
    if self.len < 1 { return None }
    let index = capped_add(self.first, self.len-1, S);
    return unsafe { Some(self.data.get_unchecked_mut(index)) };
  }

  pub unsafe fn last_unchecked(&self) -> &T {
    let index = capped_add(self.first, self.len-1, S);
    return self.data.get_unchecked(index);
  }

  pub unsafe fn last_unchecked_mut(&mut self) -> &mut T {
    let index = capped_add(self.first, self.len-1, S);
    return self.data.get_unchecked_mut(index);
  }

}

// get
impl<T: Copy, const S: usize> CircBuf<T, S> {

  pub fn get(&self, index: usize) -> Option<&T> {
    if index >= self.len { return None }
    let index = capped_add(self.first, index, S);
    return unsafe { Some(self.data.get_unchecked(index)) };
  }

  pub fn get_mut(&mut self, index: usize) -> Option<&mut T>{
    if index >= self.len { return None }
    let index = capped_add(self.first, index, S);
    return unsafe { Some(self.data.get_unchecked_mut(index)) };
  }

  pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    let index = capped_add(self.first, index, S);
    return self.data.get_unchecked(index);
  }

  pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
    let index = capped_add(self.first, index, S);
    return self.data.get_unchecked_mut(index);
  }

}

// slices
impl<T: Copy, const S: usize> CircBuf<T, S> {

  pub fn get_sorted_slices(&self) -> (&[T], &[T]) {
    let (first, second) = self.data.split_at(self.first);
    return (second, first);
  }

  pub fn as_slice(&self) -> (usize, [T; S]) {
    let (first, second) = self.get_sorted_slices();
    let mut out = [self.initial; S];
    out[0..first.len()].copy_from_slice(first);
    out[first.len()..(first.len()+second.len())].copy_from_slice(second);

    return (self.len, out);
  }

}

// misc
impl<T: Copy, const S: usize> CircBuf<T, S> {

  #[inline]
  pub fn len(&self) -> usize {
    return self.len;
  }

  pub const fn capacity(&self) -> usize {
    return S;
  }

  #[inline]
  pub fn remaining_capacity(&self) -> usize {
    return self.capacity() - self.len;
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    return self.len == 0;
  }

  #[inline]
  pub fn is_full(&self) -> bool {
    return self.len == S;
  }

}

#[inline]
fn capped_add<T: Into<usize> + From<usize>>(n0: T, n1: T, cap: T) -> T {
  let local_n0: usize = n0.into();
  let local_n1: usize = n1.into();
  let local_cap: usize = cap.into();
  return ((local_n0 + local_n1) % local_cap).into();
}

#[inline]
fn capped_sub<T: Into<usize> + From<usize>>(n0: T, n1: T, cap: T) -> T {
  let local_n0: usize = n0.into();
  let local_n1: usize = n1.into();
  let local_cap: usize = cap.into();
  return ((local_n0.wrapping_sub(local_n1)) % local_cap).into();
}

#[test]
fn main_test() {
  let mut a: CircBuf<u8, 4> = CircBuf::new(0);
  a.push_back(1).unwrap();
  a.push_back(2).unwrap();
  a.push_back(3).unwrap();
  a.push_back(4).unwrap();
  a.pop_front().unwrap();
  a.push_back(5).unwrap();
  assert_eq!(a.as_slice(), (4, [2, 3, 4, 5]));

  a.pop_back().unwrap();
  assert_eq!(a.len(), 3);
}
