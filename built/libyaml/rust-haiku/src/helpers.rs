//! Helper utilities for libyaml Rust transpilation
//!
//! This module provides safe Rust equivalents of libyaml's memory management,
//! buffer operations, string handling, and data structure utilities.

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::collections::VecDeque;
use std::ptr;
use std::slice;

// ============================================================================
// Memory Management
// ============================================================================

/// Allocate memory of the given size (minimum 1 byte).
///
/// Equivalent to libyaml's `yaml_malloc(size)`.
pub fn yaml_malloc(size: usize) -> *mut u8 {
    let size = if size == 0 { 1 } else { size };
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc(layout)
    }
}

/// Reallocate memory at the given pointer to a new size.
///
/// If ptr is null, allocates new memory. Size is minimum 1 byte.
/// Equivalent to libyaml's `yaml_realloc(ptr, size)`.
pub fn yaml_realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    let size = if size == 0 { 1 } else { size };

    if ptr.is_null() {
        yaml_malloc(size)
    } else {
        unsafe {
            // We don't know the original size, so we assume minimum alignment.
            // In practice, this is safe if the pointer came from yaml_malloc.
            let layout = Layout::from_size_align_unchecked(1, 1);
            realloc(ptr, layout, size)
        }
    }
}

/// Free memory at the given pointer.
///
/// Safe to call with null pointers (no-op).
/// Equivalent to libyaml's `yaml_free(ptr)`.
pub fn yaml_free(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let layout = Layout::from_size_align_unchecked(1, 1);
            dealloc(ptr, layout);
        }
    }
}

/// Duplicate a null-terminated C string.
///
/// Returns null if the input is null.
/// Equivalent to libyaml's `yaml_strdup(str)`.
pub fn yaml_strdup(s: *const u8) -> *mut u8 {
    if s.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        // Find length of null-terminated string
        let mut len = 0;
        let mut p = s;
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
        len += 1; // Include null terminator

        let new_ptr = yaml_malloc(len);
        if !new_ptr.is_null() {
            ptr::copy_nonoverlapping(s, new_ptr, len);
        }
        new_ptr
    }
}

// ============================================================================
// Buffer Operations
// ============================================================================

/// A dynamically allocated buffer with start, pointer, and end pointers.
///
/// Similar to the implicit buffer structure in libyaml's BUFFER_* macros.
#[derive(Clone)]
pub struct Buffer {
    pub start: *mut u8,
    pub pointer: *mut u8,
    pub end: *mut u8,
    pub last: *mut u8, // Last valid position before end
}

impl Buffer {
    /// Initialize a new buffer with the given size.
    ///
    /// Equivalent to BUFFER_INIT(context, buffer, size).
    pub fn init(size: usize) -> Option<Self> {
        let start = yaml_malloc(size);
        if start.is_null() {
            return None;
        }

        unsafe {
            let end = start.add(size);
            let last = end.sub(1);
            Some(Buffer {
                start,
                pointer: start,
                end,
                last,
            })
        }
    }

    /// Get the current position in the buffer.
    pub fn position(&self) -> usize {
        unsafe { self.pointer.offset_from(self.start) as usize }
    }

    /// Get the total size of the buffer.
    pub fn size(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }

    /// Get the remaining capacity from current pointer to end.
    pub fn remaining(&self) -> usize {
        unsafe { self.end.offset_from(self.pointer) as usize }
    }

    /// Get a byte at the current position without advancing.
    pub fn peek(&self) -> Option<u8> {
        if self.pointer < self.end {
            unsafe { Some(*self.pointer) }
        } else {
            None
        }
    }

    /// Get a byte at a specific offset from current pointer.
    pub fn peek_at(&self, offset: usize) -> Option<u8> {
        unsafe {
            let pos = self.pointer.add(offset);
            if pos < self.end {
                Some(*pos)
            } else {
                None
            }
        }
    }

    /// Advance the pointer by one byte.
    pub fn advance(&mut self) {
        unsafe {
            if self.pointer < self.end {
                self.pointer = self.pointer.add(1);
            }
        }
    }

    /// Advance the pointer by a specific number of bytes.
    pub fn advance_by(&mut self, count: usize) {
        unsafe {
            let new_ptr = self.pointer.add(count);
            if new_ptr <= self.end {
                self.pointer = new_ptr;
            }
        }
    }

    /// Write a byte at the current position and advance.
    pub fn write(&mut self, byte: u8) -> bool {
        if self.pointer >= self.end {
            return false;
        }
        unsafe {
            *self.pointer = byte;
            self.pointer = self.pointer.add(1);
        }
        true
    }

    /// Reset pointer to start.
    pub fn reset(&mut self) {
        self.pointer = self.start;
    }

    /// Get buffer contents as a slice from start to pointer.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.start, self.position()) }
    }

    /// Get buffer contents as a mutable slice from start to pointer.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.start, self.position()) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if !self.start.is_null() {
            yaml_free(self.start);
        }
    }
}

// ============================================================================
// String Operations
// ============================================================================

/// Extend a string's capacity by doubling its size.
///
/// Equivalent to libyaml's `yaml_string_extend()`.
pub fn yaml_string_extend(
    start: &mut *mut u8,
    pointer: &mut *mut u8,
    end: &mut *mut u8,
) -> bool {
    unsafe {
        let old_size = (*end).offset_from(*start) as usize;
        let new_size = old_size * 2;

        let new_start = yaml_realloc(*start, new_size);
        if new_start.is_null() {
            return false;
        }

        // Clear the new half
        ptr::write_bytes(new_start.add(old_size), 0, old_size);

        // Update pointers
        *pointer = new_start.add((*pointer).offset_from(*start) as usize);
        *end = new_start.add(new_size);
        *start = new_start;

        true
    }
}

/// Append string B to string A, extending A if necessary.
///
/// Equivalent to libyaml's `yaml_string_join()`.
pub fn yaml_string_join(
    a_start: &mut *mut u8,
    a_pointer: &mut *mut u8,
    a_end: &mut *mut u8,
    b_start: *mut u8,
    b_pointer: *mut u8,
) -> bool {
    unsafe {
        // If b is empty, nothing to do
        if b_start == b_pointer {
            return true;
        }

        let b_len = b_pointer.offset_from(b_start) as usize;

        // Extend a until it has enough space
        while (*a_end).offset_from(*a_pointer) as usize <= b_len {
            if !yaml_string_extend(a_start, a_pointer, a_end) {
                return false;
            }
        }

        // Copy b into a
        ptr::copy_nonoverlapping(b_start, *a_pointer, b_len);
        *a_pointer = (*a_pointer).add(b_len);

        true
    }
}

// ============================================================================
// Character Classification and UTF-8 Width
// ============================================================================

/// Check if a character at a given offset is alphabetic, a digit, '_', or '-'.
pub fn is_alpha_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    let ch = buffer[offset];
    (ch >= b'0' && ch <= b'9')
        || (ch >= b'A' && ch <= b'Z')
        || (ch >= b'a' && ch <= b'z')
        || ch == b'_'
        || ch == b'-'
}

/// Check if the current character is alphabetic, a digit, '_', or '-'.
pub fn is_alpha(buffer: &[u8]) -> bool {
    is_alpha_at(buffer, 0)
}

/// Check if a character at a given offset is a digit.
pub fn is_digit_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    let ch = buffer[offset];
    ch >= b'0' && ch <= b'9'
}

/// Check if the current character is a digit.
pub fn is_digit(buffer: &[u8]) -> bool {
    is_digit_at(buffer, 0)
}

/// Get the numeric value of a digit at a given offset.
pub fn as_digit_at(buffer: &[u8], offset: usize) -> u8 {
    if offset >= buffer.len() {
        return 0;
    }
    buffer[offset] - b'0'
}

/// Get the numeric value of the current digit.
pub fn as_digit(buffer: &[u8]) -> u8 {
    as_digit_at(buffer, 0)
}

/// Check if a character at a given offset is a hexadecimal digit.
pub fn is_hex_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    let ch = buffer[offset];
    (ch >= b'0' && ch <= b'9')
        || (ch >= b'A' && ch <= b'F')
        || (ch >= b'a' && ch <= b'f')
}

/// Check if the current character is a hexadecimal digit.
pub fn is_hex(buffer: &[u8]) -> bool {
    is_hex_at(buffer, 0)
}

/// Get the numeric value of a hex digit at a given offset.
pub fn as_hex_at(buffer: &[u8], offset: usize) -> u8 {
    if offset >= buffer.len() {
        return 0;
    }
    let ch = buffer[offset];
    if ch >= b'A' && ch <= b'F' {
        ch - b'A' + 10
    } else if ch >= b'a' && ch <= b'f' {
        ch - b'a' + 10
    } else {
        ch - b'0'
    }
}

/// Get the numeric value of the current hex digit.
pub fn as_hex(buffer: &[u8]) -> u8 {
    as_hex_at(buffer, 0)
}

/// Check if a character at a given offset is ASCII.
pub fn is_ascii_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    buffer[offset] <= 0x7F
}

/// Check if the current character is ASCII.
pub fn is_ascii(buffer: &[u8]) -> bool {
    is_ascii_at(buffer, 0)
}

/// Check if a character at a given offset is a space.
pub fn is_space_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    buffer[offset] == b' '
}

/// Check if the current character is a space.
pub fn is_space(buffer: &[u8]) -> bool {
    is_space_at(buffer, 0)
}

/// Check if a character at a given offset is a tab.
pub fn is_tab_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    buffer[offset] == b'\t'
}

/// Check if the current character is a tab.
pub fn is_tab(buffer: &[u8]) -> bool {
    is_tab_at(buffer, 0)
}

/// Check if a character at a given offset is blank (space or tab).
pub fn is_blank_at(buffer: &[u8], offset: usize) -> bool {
    is_space_at(buffer, offset) || is_tab_at(buffer, offset)
}

/// Check if the current character is blank (space or tab).
pub fn is_blank(buffer: &[u8]) -> bool {
    is_blank_at(buffer, 0)
}

/// Check if a character at a given offset is a line break.
///
/// Recognizes CR, LF, NEL, LS, and PS according to YAML spec.
pub fn is_break_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }

    match buffer[offset] {
        b'\r' => true,              // CR (#xD)
        b'\n' => true,              // LF (#xA)
        0xC2 => {
            // NEL (#x85) is encoded as C2 85 in UTF-8
            if offset + 1 < buffer.len() && buffer[offset + 1] == 0x85 {
                return true;
            }
            false
        }
        0xE2 => {
            // LS (#x2028) is E2 80 A8
            // PS (#x2029) is E2 80 A9
            if offset + 2 < buffer.len()
                && buffer[offset + 1] == 0x80
                && (buffer[offset + 2] == 0xA8 || buffer[offset + 2] == 0xA9)
            {
                return true;
            }
            false
        }
        _ => false,
    }
}

/// Check if the current character is a line break.
pub fn is_break(buffer: &[u8]) -> bool {
    is_break_at(buffer, 0)
}

/// Check if a character at a given offset is a CRLF pair.
pub fn is_crlf_at(buffer: &[u8], offset: usize) -> bool {
    if offset + 1 >= buffer.len() {
        return false;
    }
    buffer[offset] == b'\r' && buffer[offset + 1] == b'\n'
}

/// Check if the current character is a CRLF pair.
pub fn is_crlf(buffer: &[u8]) -> bool {
    is_crlf_at(buffer, 0)
}

/// Check if a character at a given offset is NUL (0x00).
pub fn is_z_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }
    buffer[offset] == 0x00
}

/// Check if the current character is NUL.
pub fn is_z(buffer: &[u8]) -> bool {
    is_z_at(buffer, 0)
}

/// Check if a character at a given offset is a BOM (Byte Order Mark).
///
/// BOM is U+FEFF, encoded as EF BB BF in UTF-8.
pub fn is_bom_at(buffer: &[u8], offset: usize) -> bool {
    if offset + 2 >= buffer.len() {
        return false;
    }
    buffer[offset] == 0xEF && buffer[offset + 1] == 0xBB && buffer[offset + 2] == 0xBF
}

/// Check if the current character is a BOM.
pub fn is_bom(buffer: &[u8]) -> bool {
    is_bom_at(buffer, 0)
}

/// Check if a character at a given offset is a line break or NUL.
pub fn is_breakz_at(buffer: &[u8], offset: usize) -> bool {
    is_break_at(buffer, offset) || is_z_at(buffer, offset)
}

/// Check if the current character is a line break or NUL.
pub fn is_breakz(buffer: &[u8]) -> bool {
    is_breakz_at(buffer, 0)
}

/// Check if a character at a given offset is a space, line break, or NUL.
pub fn is_spacez_at(buffer: &[u8], offset: usize) -> bool {
    is_space_at(buffer, offset) || is_breakz_at(buffer, offset)
}

/// Check if the current character is a space, line break, or NUL.
pub fn is_spacez(buffer: &[u8]) -> bool {
    is_spacez_at(buffer, 0)
}

/// Check if a character at a given offset is blank (space/tab), line break, or NUL.
pub fn is_blankz_at(buffer: &[u8], offset: usize) -> bool {
    is_blank_at(buffer, offset) || is_breakz_at(buffer, offset)
}

/// Check if the current character is blank (space/tab), line break, or NUL.
pub fn is_blankz(buffer: &[u8]) -> bool {
    is_blankz_at(buffer, 0)
}

/// Check if a character at a given offset is printable according to YAML spec.
///
/// Printable includes:
/// - LF (#xA)
/// - #x20-#x7E (ASCII printable)
/// - UTF-8 sequences for other ranges (with some exclusions)
pub fn is_printable_at(buffer: &[u8], offset: usize) -> bool {
    if offset >= buffer.len() {
        return false;
    }

    let ch = buffer[offset];

    // LF (#xA)
    if ch == 0x0A {
        return true;
    }

    // #x20 <= . <= #x7E
    if ch >= 0x20 && ch <= 0x7E {
        return true;
    }

    // Multi-byte UTF-8 sequences
    if ch == 0xC2 {
        // #0xA0 <= . <= #xD7FF (starts with C2)
        if offset + 1 < buffer.len() && buffer[offset + 1] >= 0xA0 {
            return true;
        }
    } else if ch > 0xC2 && ch < 0xED {
        // #0xA0 <= . <= #xD7FF (continues)
        return true;
    } else if ch == 0xED {
        // Partial range for #xD800-#xDFFF
        if offset + 1 < buffer.len() && buffer[offset + 1] < 0xA0 {
            return true;
        }
    } else if ch == 0xEE {
        // More ranges
        return true;
    } else if ch == 0xEF {
        // #xE000 <= . <= #xFFFD (with exclusions)
        if offset + 2 < buffer.len() {
            let b1 = buffer[offset + 1];
            let b2 = buffer[offset + 2];

            // Exclude BOM (#xFEFF)
            if b1 == 0xBB && b2 == 0xBF {
                return false;
            }

            // Exclude #xFFFE and #xFFFF
            if b1 == 0xBF && (b2 == 0xBE || b2 == 0xBF) {
                return false;
            }

            return true;
        }
    }

    false
}

/// Check if the current character is printable.
pub fn is_printable(buffer: &[u8]) -> bool {
    is_printable_at(buffer, 0)
}

/// Determine the width (in bytes) of a UTF-8 character at a given offset.
///
/// Returns 1 for ASCII, 2-4 for multi-byte sequences, or 0 for invalid.
pub fn width_at(buffer: &[u8], offset: usize) -> usize {
    if offset >= buffer.len() {
        return 0;
    }

    let ch = buffer[offset];
    if ch & 0x80 == 0x00 {
        1 // 1-byte character
    } else if ch & 0xE0 == 0xC0 {
        2 // 2-byte character
    } else if ch & 0xF0 == 0xE0 {
        3 // 3-byte character
    } else if ch & 0xF8 == 0xF0 {
        4 // 4-byte character
    } else {
        0 // Invalid start byte
    }
}

/// Get the width of the current UTF-8 character.
pub fn width(buffer: &[u8]) -> usize {
    width_at(buffer, 0)
}

/// Move a pointer forward by one UTF-8 character width.
pub fn move_pointer(buffer: &mut Buffer) {
    let w = width_at(
        unsafe { slice::from_raw_parts(buffer.pointer, buffer.remaining()) },
        0,
    );
    if w > 0 {
        buffer.advance_by(w);
    }
}

// ============================================================================
// Stack Operations
// ============================================================================

/// A simple LIFO stack implemented with Vec.
#[derive(Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack.
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    /// Create a new stack with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Push an item onto the stack.
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Pop an item from the stack, returning None if empty.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Get a reference to the top item without removing it.
    pub fn top(&self) -> Option<&T> {
        self.items.last()
    }

    /// Get a mutable reference to the top item.
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items in the stack.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Clear the stack.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get a slice of all items.
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Get a mutable slice of all items.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items
    }

    /// Check if the stack contains at least `size` items.
    pub fn has_at_least(&self, size: usize) -> bool {
        self.items.len() >= size
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Queue Operations
// ============================================================================

/// A simple FIFO queue.
///
/// Uses VecDeque for efficient O(1) operations at both ends.
#[derive(Clone)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Create a new empty queue.
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    /// Create a new queue with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            items: VecDeque::with_capacity(capacity),
        }
    }

    /// Enqueue an item at the back.
    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    /// Dequeue an item from the front, returning None if empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Get a reference to the front item without removing it.
    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    /// Get a mutable reference to the front item.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.items.front_mut()
    }

    /// Get a reference to the back item.
    pub fn back(&self) -> Option<&T> {
        self.items.back()
    }

    /// Get a mutable reference to the back item.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.items.back_mut()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items in the queue.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Clear the queue.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get a slice of all items.
    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.items.as_slices()
    }

    /// Get a mutable slice of all items (as two contiguous slices).
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        self.items.as_mut_slices()
    }

    /// Insert an item at a specific index.
    ///
    /// Equivalent to QUEUE_INSERT in libyaml.
    pub fn insert(&mut self, index: usize, value: T) {
        self.items.insert(index, value);
    }

    /// Get an element by index.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            self.items.get(index)
        } else {
            None
        }
    }

    /// Get a mutable reference to an element by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.items.len() {
            self.items.get_mut(index)
        } else {
            None
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Constants
// ============================================================================

/// Size of the input raw buffer (before UTF-8 decoding).
pub const INPUT_RAW_BUFFER_SIZE: usize = 16384;

/// Size of the input buffer (after UTF-8 decoding).
/// Should be able to accommodate the whole raw buffer decoded.
pub const INPUT_BUFFER_SIZE: usize = INPUT_RAW_BUFFER_SIZE * 3;

/// Size of the output buffer.
pub const OUTPUT_BUFFER_SIZE: usize = 16384;

/// Size of the output raw buffer (before UTF-8 encoding).
/// Should be able to accommodate the whole output buffer encoded.
pub const OUTPUT_RAW_BUFFER_SIZE: usize = OUTPUT_BUFFER_SIZE * 2 + 2;

/// Maximum file size for YAML parsing.
pub const MAX_FILE_SIZE: usize = !0_usize / 2;

/// Initial size for stacks.
pub const INITIAL_STACK_SIZE: usize = 16;

/// Initial size for queues.
pub const INITIAL_QUEUE_SIZE: usize = 16;

/// Initial size for strings.
pub const INITIAL_STRING_SIZE: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malloc_and_free() {
        let ptr = yaml_malloc(100);
        assert!(!ptr.is_null());
        unsafe {
            *ptr = 42;
            assert_eq!(*ptr, 42);
        }
        yaml_free(ptr);
    }

    #[test]
    fn test_malloc_zero() {
        let ptr = yaml_malloc(0);
        assert!(!ptr.is_null());
        yaml_free(ptr);
    }

    #[test]
    fn test_free_null() {
        yaml_free(ptr::null_mut()); // Should not crash
    }

    #[test]
    fn test_is_digit() {
        assert!(is_digit(b"5xyz"));
        assert!(!is_digit(b"abc"));
        assert!(is_digit_at(b"abc5", 3));
    }

    #[test]
    fn test_is_hex() {
        assert!(is_hex(b"Axy"));
        assert!(is_hex(b"f12"));
        assert!(!is_hex(b"Gxy"));
    }

    #[test]
    fn test_is_alpha() {
        assert!(is_alpha(b"abc"));
        assert!(is_alpha(b"_var"));
        assert!(is_alpha(b"-key"));
        assert!(!is_alpha(b"!sym"));
    }

    #[test]
    fn test_is_space() {
        assert!(is_space(b" x"));
        assert!(!is_space(b"\tx"));
        assert!(!is_space(b"ax"));
    }

    #[test]
    fn test_is_blank() {
        assert!(is_blank(b" x"));
        assert!(is_blank(b"\tx"));
        assert!(!is_blank(b"ax"));
    }

    #[test]
    fn test_is_break() {
        assert!(is_break(b"\rx"));
        assert!(is_break(b"\nx"));
        assert!(!is_break(b"ax"));
    }

    #[test]
    fn test_width() {
        assert_eq!(width(b"a"), 1); // ASCII
        assert_eq!(width(&[0xC2, 0xA0]), 2); // 2-byte UTF-8
        assert_eq!(width(&[0xE2, 0x80, 0xA8]), 3); // 3-byte UTF-8
        assert_eq!(width(&[0xF0, 0x90, 0x80, 0x80]), 4); // 4-byte UTF-8
    }

    #[test]
    fn test_buffer_init() {
        let buf = Buffer::init(100);
        assert!(buf.is_some());
        let buf = buf.unwrap();
        assert_eq!(buf.size(), 100);
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.remaining(), 100);
    }

    #[test]
    fn test_buffer_write() {
        let mut buf = Buffer::init(10).unwrap();
        assert!(buf.write(42));
        assert_eq!(buf.position(), 1);
        assert!(buf.write(99));
        assert_eq!(buf.position(), 2);
    }

    #[test]
    fn test_stack() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_queue() {
        let mut queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_queue_insert() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(3);
        queue.insert(1, 2);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }
}
