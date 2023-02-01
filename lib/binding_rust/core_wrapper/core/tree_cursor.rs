use crate::core::util::*;
use crate::core::*;
use :: c2rust_bitfields;
use std::os;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type TSStateId = uint16_t;
pub type TSSymbol = uint16_t;
pub type TSFieldId = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub states: *const bool,
    pub symbol_map: *const TSSymbol,
    pub create: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scan: Option<unsafe extern "C" fn(*mut libc::c_void, *mut TSLexer, *const bool) -> bool>,
    pub serialize:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char) -> libc::c_uint>,
    pub deserialize:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_uint) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub count: uint8_t,
    pub reusable: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub type_: uint8_t,
    pub child_count: uint8_t,
    pub symbol: TSSymbol,
    pub dynamic_precedence: int16_t,
    pub production_id: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_: uint8_t,
    pub state: TSStateId,
    pub extra: bool,
    pub repetition: bool,
}
type C2RustUnnamed_3 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_4 = crate::core::util::LongShortData;
type C2RustUnnamed_5 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadFirstLeaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeCursor {
    pub tree: *const TSTree,
    pub stack: C2RustUnnamed_7,
    pub root_alias_symbol: TSSymbol,
}
type C2RustUnnamed_7 = crate::core::util::StackElement<*mut TreeCursorEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeCursorEntry {
    pub subtree: *const Subtree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub descendant_index: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Array {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
pub const TreeCursorStepVisible: TreeCursorStep = 2;
pub const TreeCursorStepHidden: TreeCursorStep = 1;
pub type TreeCursorStep = libc::c_uint;
pub const TreeCursorStepNone: TreeCursorStep = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CursorChildIterator {
    pub parent: Subtree,
    pub tree: *const TSTree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub descendant_index: uint32_t,
    pub alias_sequence: *const TSSymbol,
}
#[inline]
unsafe extern "C" fn ts_subtree_visible_child_count(mut self_0: Subtree) -> uint32_t {
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .visible_child_count;
    } else {
        return 0 as libc::c_int as uint32_t;
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_extra(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).extra() as libc::c_int
    } else {
        (*self_0.ptr).extra() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).visible() as libc::c_int
    } else {
        (*self_0.ptr).visible() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        (*self_0.ptr).child_count
    };
}
#[inline]
unsafe extern "C" fn length_zero() -> Length {
    let mut result: Length = {
        let mut init = Length {
            bytes: 0 as libc::c_int as uint32_t,
            extent: {
                let mut init = TSPoint {
                    row: 0 as libc::c_int as uint32_t,
                    column: 0 as libc::c_int as uint32_t,
                };
                init
            },
        };
        init
    };
    return result;
}
#[inline]
unsafe extern "C" fn ts_subtree_padding(mut self_0: Subtree) -> Length {
    if (self_0.data).is_inline() {
        let mut result: Length = {
            let mut init = Length {
                bytes: self_0.data.padding_bytes as uint32_t,
                extent: {
                    let mut init = TSPoint {
                        row: (self_0.data).padding_rows() as uint32_t,
                        column: self_0.data.padding_columns as uint32_t,
                    };
                    init
                },
            };
            init
        };
        return result;
    } else {
        return (*self_0.ptr).padding;
    };
}
#[inline]
unsafe extern "C" fn length_add(mut len1: Length, mut len2: Length) -> Length {
    let mut result: Length = Length {
        bytes: 0,
        extent: TSPoint { row: 0, column: 0 },
    };
    result.bytes = (len1.bytes).wrapping_add(len2.bytes);
    result.extent = point_add(len1.extent, len2.extent);
    return result;
}
#[inline]
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new((a.row).wrapping_add(b.row), b.column);
    } else {
        return point__new(a.row, (a.column).wrapping_add(b.column));
    };
}
#[inline]
unsafe extern "C" fn point__new(mut row: libc::c_uint, mut column: libc::c_uint) -> TSPoint {
    let mut result: TSPoint = {
        let mut init = TSPoint {
            row: row,
            column: column,
        };
        init
    };
    return result;
}
#[inline]
unsafe extern "C" fn ts_subtree_size(mut self_0: Subtree) -> Length {
    if (self_0.data).is_inline() {
        let mut result: Length = {
            let mut init = Length {
                bytes: self_0.data.size_bytes as uint32_t,
                extent: {
                    let mut init = TSPoint {
                        row: 0 as libc::c_int as uint32_t,
                        column: self_0.data.size_bytes as uint32_t,
                    };
                    init
                },
            };
            init
        };
        return result;
    } else {
        return (*self_0.ptr).size;
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_visible_descendant_count(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0
        || (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int as libc::c_uint
    } else {
        (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .visible_descendant_count
    };
}
static mut LENGTH_UNDEFINED: Length = {
    let mut init = Length {
        bytes: 0 as libc::c_int as uint32_t,
        extent: {
            let mut init = TSPoint {
                row: 0 as libc::c_int as uint32_t,
                column: 1 as libc::c_int as uint32_t,
            };
            init
        },
    };
    init
};
#[inline]
unsafe extern "C" fn length_is_undefined(mut length: Length) -> bool {
    return length.bytes == 0 as libc::c_int as libc::c_uint
        && length.extent.column != 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ts_subtree_total_size(mut self_0: Subtree) -> Length {
    return length_add(ts_subtree_padding(self_0), ts_subtree_size(self_0));
}
#[inline]
unsafe extern "C" fn point_gte(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row > b.row || a.row == b.row && a.column >= b.column;
}
#[inline]
unsafe extern "C" fn _array__grow(
    mut self_0: *mut Array,
    mut count: uint32_t,
    mut element_size: size_t,
) {
    let mut new_size: uint32_t = ((*self_0).size).wrapping_add(count);
    if new_size > (*self_0).capacity {
        let mut new_capacity: uint32_t =
            ((*self_0).capacity).wrapping_mul(2 as libc::c_int as libc::c_uint);
        if new_capacity < 8 as libc::c_int as libc::c_uint {
            new_capacity = 8 as libc::c_int as uint32_t;
        }
        if new_capacity < new_size {
            new_capacity = new_size;
        }
        _array__reserve(self_0, element_size, new_capacity);
    }
}
#[inline]
unsafe extern "C" fn _array__reserve(
    mut self_0: *mut Array,
    mut element_size: size_t,
    mut new_capacity: uint32_t,
) {
    if new_capacity > (*self_0).capacity {
        if !((*self_0).contents).is_null() {
            (*self_0).contents = crate::core::alloc::ts_realloc(
                (*self_0).contents,
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        } else {
            (*self_0).contents = crate::core::alloc::ts_malloc(
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        }
        (*self_0).capacity = new_capacity;
    }
}
#[inline]
unsafe extern "C" fn _array__delete(mut self_0: *mut Array) {
    if !((*self_0).contents).is_null() {
        crate::core::alloc::ts_free((*self_0).contents);
        (*self_0).contents = 0 as *mut libc::c_void;
        (*self_0).size = 0 as libc::c_int as uint32_t;
        (*self_0).capacity = 0 as libc::c_int as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn _array__splice(
    mut self_0: *mut Array,
    mut element_size: size_t,
    mut index: uint32_t,
    mut old_count: uint32_t,
    mut new_count: uint32_t,
    mut elements: *const libc::c_void,
) {
    let mut new_size: uint32_t = ((*self_0).size)
        .wrapping_add(new_count)
        .wrapping_sub(old_count);
    let mut old_end: uint32_t = index.wrapping_add(old_count);
    let mut new_end: uint32_t = index.wrapping_add(new_count);
    if old_end <= (*self_0).size {
    } else {
        panic!();
    }
    _array__reserve(self_0, element_size, new_size);
    let mut contents: *mut libc::c_char = (*self_0).contents as *mut libc::c_char;
    if (*self_0).size > old_end {
        std::ptr::copy(
            contents.offset((old_end as libc::c_ulong).wrapping_mul(element_size) as isize)
                as *const libc::c_void,
            contents.offset((new_end as libc::c_ulong).wrapping_mul(element_size) as isize)
                as *mut libc::c_void,
            ((((*self_0).size).wrapping_sub(old_end) as libc::c_ulong).wrapping_mul(element_size))
                as usize,
        );
    }
    if new_count > 0 as libc::c_int as libc::c_uint {
        if !elements.is_null() {
            std::ptr::copy_nonoverlapping(
                elements,
                contents.offset((index as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
                (new_count as libc::c_ulong).wrapping_mul(element_size),
            );
        } else {
            std::ptr::write_bytes(
                contents.offset((index as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
                (0 as libc::c_int) as u8,
                ((new_count as libc::c_ulong).wrapping_mul(element_size)) as usize,
            );
        }
    }
    (*self_0).size = ((*self_0).size as libc::c_uint)
        .wrapping_add(new_count.wrapping_sub(old_count)) as uint32_t
        as uint32_t;
}
#[inline]
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.symbol as libc::c_int
    } else {
        (*self_0.ptr).symbol as libc::c_int
    }) as TSSymbol;
}
#[inline]
unsafe extern "C" fn ts_language_alias_sequence(
    mut self_0: *const TSLanguage,
    mut production_id: uint32_t,
) -> *const TSSymbol {
    return if production_id != 0 {
        &*((*self_0).alias_sequences).offset(
            production_id.wrapping_mul((*self_0).max_alias_sequence_length as libc::c_uint)
                as isize,
        ) as *const TSSymbol
    } else {
        0 as *const TSSymbol
    };
}
#[inline]
unsafe extern "C" fn ts_language_alias_at(
    mut self_0: *const TSLanguage,
    mut production_id: uint32_t,
    mut child_index: uint32_t,
) -> TSSymbol {
    return (if production_id != 0 {
        *((*self_0).alias_sequences).offset(
            production_id
                .wrapping_mul((*self_0).max_alias_sequence_length as libc::c_uint)
                .wrapping_add(child_index) as isize,
        ) as libc::c_int
    } else {
        0 as libc::c_int
    }) as TSSymbol;
}
#[inline]
unsafe extern "C" fn ts_language_field_map(
    mut self_0: *const TSLanguage,
    mut production_id: uint32_t,
    mut start: *mut *const TSFieldMapEntry,
    mut end: *mut *const TSFieldMapEntry,
) {
    if (*self_0).field_count == 0 as libc::c_int as libc::c_uint {
        *start = 0 as *const TSFieldMapEntry;
        *end = 0 as *const TSFieldMapEntry;
        return;
    }
    let mut slice: TSFieldMapSlice = *((*self_0).field_map_slices).offset(production_id as isize);
    *start = &*((*self_0).field_map_entries).offset(slice.index as isize) as *const TSFieldMapEntry;
    *end = (&*((*self_0).field_map_entries).offset(slice.index as isize) as *const TSFieldMapEntry)
        .offset(slice.length as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_is_entry_visible(
    mut self_0: *const TreeCursor,
    mut index: uint32_t,
) -> bool {
    let mut entry: *mut TreeCursorEntry =
        &mut *((*self_0).stack.contents).offset(index as isize) as *mut TreeCursorEntry;
    if index == 0 as libc::c_int as libc::c_uint
        || ts_subtree_visible(*(*entry).subtree) as libc::c_int != 0
    {
        return 1 as libc::c_int != 0;
    } else if !ts_subtree_extra(*(*entry).subtree) {
        let mut parent_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
            .offset(index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as *mut TreeCursorEntry;
        return ts_language_alias_at(
            (*(*self_0).tree).language,
            (*(*(*parent_entry).subtree).ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .production_id as uint32_t,
            (*entry).structural_child_index,
        ) != 0;
    } else {
        return 0 as libc::c_int != 0;
    };
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_iterate_children(
    mut self_0: *const TreeCursor,
) -> CursorChildIterator {
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut last_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry;
    if ts_subtree_child_count(*(*last_entry).subtree) == 0 as libc::c_int as libc::c_uint {
        return {
            let mut init = CursorChildIterator {
                parent: Subtree {
                    ptr: 0 as *const SubtreeHeapData,
                },
                tree: (*self_0).tree,
                position: length_zero(),
                child_index: 0 as libc::c_int as uint32_t,
                structural_child_index: 0 as libc::c_int as uint32_t,
                descendant_index: 0 as libc::c_int as uint32_t,
                alias_sequence: 0 as *const TSSymbol,
            };
            init
        };
    }
    let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
        (*(*self_0).tree).language,
        (*(*(*last_entry).subtree).ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .production_id as uint32_t,
    );
    let mut descendant_index: uint32_t = (*last_entry).descendant_index;
    if ts_tree_cursor_is_entry_visible(
        self_0,
        ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) {
        descendant_index = (descendant_index as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t
            as uint32_t;
    }
    return {
        let mut init = CursorChildIterator {
            parent: *(*last_entry).subtree,
            tree: (*self_0).tree,
            position: (*last_entry).position,
            child_index: 0 as libc::c_int as uint32_t,
            structural_child_index: 0 as libc::c_int as uint32_t,
            descendant_index: descendant_index,
            alias_sequence: alias_sequence,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_child_iterator_next(
    mut self_0: *mut CursorChildIterator,
    mut result: *mut TreeCursorEntry,
    mut visible: *mut bool,
) -> bool {
    if ((*self_0).parent.ptr).is_null()
        || (*self_0).child_index == (*(*self_0).parent.ptr).child_count
    {
        return 0 as libc::c_int != 0;
    }
    let mut child: *const Subtree = &mut *if ((*self_0).parent.data).is_inline() as libc::c_int != 0
    {
        0 as *mut Subtree
    } else {
        ((*self_0).parent.ptr as *mut Subtree)
            .offset(-((*(*self_0).parent.ptr).child_count as isize))
    }
    .offset((*self_0).child_index as isize) as *mut Subtree;
    *result = {
        let mut init = TreeCursorEntry {
            subtree: child,
            position: (*self_0).position,
            child_index: (*self_0).child_index,
            structural_child_index: (*self_0).structural_child_index,
            descendant_index: (*self_0).descendant_index,
        };
        init
    };
    *visible = ts_subtree_visible(*child);
    let mut extra: bool = ts_subtree_extra(*child);
    if !extra {
        if !((*self_0).alias_sequence).is_null() {
            *visible = (*visible as libc::c_int
                | *((*self_0).alias_sequence).offset((*self_0).structural_child_index as isize)
                    as libc::c_int)
                != 0;
        }
        (*self_0).structural_child_index = ((*self_0).structural_child_index).wrapping_add(1);
    }
    (*self_0).descendant_index = ((*self_0).descendant_index as libc::c_uint)
        .wrapping_add(ts_subtree_visible_descendant_count(*child))
        as uint32_t as uint32_t;
    if *visible {
        (*self_0).descendant_index = ((*self_0).descendant_index as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
    (*self_0).position = length_add((*self_0).position, ts_subtree_size(*child));
    (*self_0).child_index = ((*self_0).child_index).wrapping_add(1);
    if (*self_0).child_index < (*(*self_0).parent.ptr).child_count {
        let mut next_child: Subtree = *if ((*self_0).parent.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            ((*self_0).parent.ptr as *mut Subtree)
                .offset(-((*(*self_0).parent.ptr).child_count as isize))
        }
        .offset((*self_0).child_index as isize);
        (*self_0).position = length_add((*self_0).position, ts_subtree_padding(next_child));
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn length_backtrack(mut a: Length, mut b: Length) -> Length {
    if length_is_undefined(a) as libc::c_int != 0
        || b.extent.row != 0 as libc::c_int as libc::c_uint
    {
        return LENGTH_UNDEFINED;
    }
    let mut result: Length = Length {
        bytes: 0,
        extent: TSPoint { row: 0, column: 0 },
    };
    result.bytes = (a.bytes).wrapping_sub(b.bytes);
    result.extent.row = a.extent.row;
    result.extent.column = (a.extent.column).wrapping_sub(b.extent.column);
    return result;
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_child_iterator_previous(
    mut self_0: *mut CursorChildIterator,
    mut result: *mut TreeCursorEntry,
    mut visible: *mut bool,
) -> bool {
    if ((*self_0).parent.ptr).is_null()
        || (*self_0).child_index as int8_t as libc::c_int == -(1 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    let mut child: *const Subtree = &mut *if ((*self_0).parent.data).is_inline() as libc::c_int != 0
    {
        0 as *mut Subtree
    } else {
        ((*self_0).parent.ptr as *mut Subtree)
            .offset(-((*(*self_0).parent.ptr).child_count as isize))
    }
    .offset((*self_0).child_index as isize) as *mut Subtree;
    *result = {
        let mut init = TreeCursorEntry {
            subtree: child,
            position: (*self_0).position,
            child_index: (*self_0).child_index,
            structural_child_index: (*self_0).structural_child_index,
            descendant_index: 0,
        };
        init
    };
    *visible = ts_subtree_visible(*child);
    let mut extra: bool = ts_subtree_extra(*child);
    if !extra && !((*self_0).alias_sequence).is_null() {
        *visible = (*visible as libc::c_int
            | *((*self_0).alias_sequence).offset((*self_0).structural_child_index as isize)
                as libc::c_int)
            != 0;
        (*self_0).structural_child_index = ((*self_0).structural_child_index).wrapping_sub(1);
    }
    (*self_0).position = length_backtrack((*self_0).position, ts_subtree_padding(*child));
    (*self_0).child_index = ((*self_0).child_index).wrapping_sub(1);
    if (*self_0).child_index < (*(*self_0).parent.ptr).child_count {
        let mut previous_child: Subtree =
            *if ((*self_0).parent.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                ((*self_0).parent.ptr as *mut Subtree)
                    .offset(-((*(*self_0).parent.ptr).child_count as isize))
            }
            .offset((*self_0).child_index as isize);
        let mut size: Length = ts_subtree_size(previous_child);
        (*self_0).position = length_backtrack((*self_0).position, size);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_new(mut node: TSNode) -> TSTreeCursor {
    let mut self_0: TSTreeCursor = {
        let mut init = TSTreeCursor {
            tree: 0 as *const libc::c_void,
            id: 0 as *const libc::c_void,
            context: [
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            ],
        };
        init
    };
    ts_tree_cursor_init(&mut self_0 as *mut TSTreeCursor as *mut TreeCursor, node);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_reset(mut _self: *mut TSTreeCursor, mut node: TSNode) {
    ts_tree_cursor_init(_self as *mut TreeCursor, node);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_init(mut self_0: *mut TreeCursor, mut node: TSNode) {
    (*self_0).tree = node.tree;
    (*self_0).root_alias_symbol = node.context[3 as libc::c_int as usize] as TSSymbol;
    (*self_0).stack.size = 0 as libc::c_int as uint32_t;
    _array__grow(
        &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
        1 as libc::c_int as uint32_t,
        ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
    );
    let fresh0 = (*self_0).stack.size;
    (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
    *((*self_0).stack.contents).offset(fresh0 as isize) = {
        let mut init = TreeCursorEntry {
            subtree: node.id as *const Subtree,
            position: {
                let mut init = Length {
                    bytes: ts_node_start_byte(node),
                    extent: ts_node_start_point(node),
                };
                init
            },
            child_index: 0 as libc::c_int as uint32_t,
            structural_child_index: 0 as libc::c_int as uint32_t,
            descendant_index: 0 as libc::c_int as uint32_t,
        };
        init
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_delete(mut _self: *mut TSTreeCursor) {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    _array__delete(&mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_first_child_internal(
    mut _self: *mut TSTreeCursor,
) -> TreeCursorStep {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut visible: bool = false;
    let mut entry: TreeCursorEntry = TreeCursorEntry {
        subtree: 0 as *const Subtree,
        position: Length {
            bytes: 0,
            extent: TSPoint { row: 0, column: 0 },
        },
        child_index: 0,
        structural_child_index: 0,
        descendant_index: 0,
    };
    let mut iterator: CursorChildIterator = ts_tree_cursor_iterate_children(self_0);
    while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry, &mut visible) {
        if visible {
            _array__grow(
                &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                1 as libc::c_int as uint32_t,
                ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
            );
            let fresh1 = (*self_0).stack.size;
            (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
            *((*self_0).stack.contents).offset(fresh1 as isize) = entry;
            return TreeCursorStepVisible;
        }
        if ts_subtree_visible_child_count(*entry.subtree) > 0 as libc::c_int as libc::c_uint {
            _array__grow(
                &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                1 as libc::c_int as uint32_t,
                ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
            );
            let fresh2 = (*self_0).stack.size;
            (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
            *((*self_0).stack.contents).offset(fresh2 as isize) = entry;
            return TreeCursorStepHidden;
        }
    }
    return TreeCursorStepNone;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_first_child(mut self_0: *mut TSTreeCursor) -> bool {
    loop {
        match ts_tree_cursor_goto_first_child_internal(self_0) as libc::c_uint {
            1 => {}
            2 => return 1 as libc::c_int != 0,
            _ => return 0 as libc::c_int != 0,
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_last_child_internal(
    mut _self: *mut TSTreeCursor,
) -> TreeCursorStep {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut visible: bool = false;
    let mut entry: TreeCursorEntry = TreeCursorEntry {
        subtree: 0 as *const Subtree,
        position: Length {
            bytes: 0,
            extent: TSPoint { row: 0, column: 0 },
        },
        child_index: 0,
        structural_child_index: 0,
        descendant_index: 0,
    };
    let mut iterator: CursorChildIterator = ts_tree_cursor_iterate_children(self_0);
    if (iterator.parent.ptr).is_null()
        || (*iterator.parent.ptr).child_count == 0 as libc::c_int as libc::c_uint
    {
        return TreeCursorStepNone;
    }
    let mut last_entry: TreeCursorEntry = {
        let mut init = TreeCursorEntry {
            subtree: 0 as *const Subtree,
            position: Length {
                bytes: 0,
                extent: TSPoint { row: 0, column: 0 },
            },
            child_index: 0,
            structural_child_index: 0,
            descendant_index: 0,
        };
        init
    };
    let mut last_step: TreeCursorStep = TreeCursorStepNone;
    while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry, &mut visible) {
        if visible {
            last_entry = entry;
            last_step = TreeCursorStepVisible;
        } else if ts_subtree_visible_child_count(*entry.subtree) > 0 as libc::c_int as libc::c_uint
        {
            last_entry = entry;
            last_step = TreeCursorStepHidden;
        }
    }
    if !(last_entry.subtree).is_null() {
        _array__grow(
            &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
            1 as libc::c_int as uint32_t,
            ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
        );
        let fresh3 = (*self_0).stack.size;
        (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
        *((*self_0).stack.contents).offset(fresh3 as isize) = last_entry;
        return last_step;
    }
    return TreeCursorStepNone;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_last_child(mut self_0: *mut TSTreeCursor) -> bool {
    loop {
        match ts_tree_cursor_goto_last_child_internal(self_0) as libc::c_uint {
            1 => {}
            2 => return 1 as libc::c_int != 0,
            _ => return 0 as libc::c_int != 0,
        }
    }
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_goto_first_child_for_byte_and_point(
    mut _self: *mut TSTreeCursor,
    mut goal_byte: uint32_t,
    mut goal_point: TSPoint,
) -> int64_t {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut initial_size: uint32_t = (*self_0).stack.size;
    let mut visible_child_index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut did_descend: bool = false;
    loop {
        did_descend = 0 as libc::c_int != 0;
        let mut visible: bool = false;
        let mut entry: TreeCursorEntry = TreeCursorEntry {
            subtree: 0 as *const Subtree,
            position: Length {
                bytes: 0,
                extent: TSPoint { row: 0, column: 0 },
            },
            child_index: 0,
            structural_child_index: 0,
            descendant_index: 0,
        };
        let mut iterator: CursorChildIterator = ts_tree_cursor_iterate_children(self_0);
        while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry, &mut visible) {
            let mut entry_end: Length = length_add(entry.position, ts_subtree_size(*entry.subtree));
            let mut at_goal: bool = entry_end.bytes >= goal_byte
                && point_gte(entry_end.extent, goal_point) as libc::c_int != 0;
            let mut visible_child_count: uint32_t = ts_subtree_visible_child_count(*entry.subtree);
            if at_goal {
                if visible {
                    _array__grow(
                        &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                        1 as libc::c_int as uint32_t,
                        ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
                    );
                    let fresh4 = (*self_0).stack.size;
                    (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
                    *((*self_0).stack.contents).offset(fresh4 as isize) = entry;
                    return visible_child_index as int64_t;
                }
                if !(visible_child_count > 0 as libc::c_int as libc::c_uint) {
                    continue;
                }
                _array__grow(
                    &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                    1 as libc::c_int as uint32_t,
                    ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
                );
                let fresh5 = (*self_0).stack.size;
                (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
                *((*self_0).stack.contents).offset(fresh5 as isize) = entry;
                did_descend = 1 as libc::c_int != 0;
                break;
            } else if visible {
                visible_child_index = visible_child_index.wrapping_add(1);
            } else {
                visible_child_index = (visible_child_index as libc::c_uint)
                    .wrapping_add(visible_child_count)
                    as uint32_t as uint32_t;
            }
        }
        if !did_descend {
            break;
        }
    }
    (*self_0).stack.size = initial_size;
    return -(1 as libc::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_first_child_for_byte(
    mut self_0: *mut TSTreeCursor,
    mut goal_byte: uint32_t,
) -> int64_t {
    return ts_tree_cursor_goto_first_child_for_byte_and_point(self_0, goal_byte, {
        let mut init = TSPoint {
            row: 0 as libc::c_int as uint32_t,
            column: 0 as libc::c_int as uint32_t,
        };
        init
    });
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_first_child_for_point(
    mut self_0: *mut TSTreeCursor,
    mut goal_point: TSPoint,
) -> int64_t {
    return ts_tree_cursor_goto_first_child_for_byte_and_point(
        self_0,
        0 as libc::c_int as uint32_t,
        goal_point,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_sibling_internal(
    mut _self: *mut TSTreeCursor,
    mut advance: Option<
        unsafe extern "C" fn(*mut CursorChildIterator, *mut TreeCursorEntry, *mut bool) -> bool,
    >,
) -> TreeCursorStep {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut initial_size: uint32_t = (*self_0).stack.size;
    while (*self_0).stack.size > 1 as libc::c_int as libc::c_uint {
        (*self_0).stack.size = ((*self_0).stack.size).wrapping_sub(1);
        let mut entry: TreeCursorEntry =
            *((*self_0).stack.contents).offset((*self_0).stack.size as isize);
        let mut iterator: CursorChildIterator = ts_tree_cursor_iterate_children(self_0);
        iterator.child_index = entry.child_index;
        iterator.structural_child_index = entry.structural_child_index;
        iterator.position = entry.position;
        iterator.descendant_index = entry.descendant_index;
        let mut visible: bool = 0 as libc::c_int != 0;
        advance.expect("non-null function pointer")(&mut iterator, &mut entry, &mut visible);
        if visible as libc::c_int != 0
            && ((*self_0).stack.size).wrapping_add(1 as libc::c_int as libc::c_uint) < initial_size
        {
            break;
        }
        while advance.expect("non-null function pointer")(&mut iterator, &mut entry, &mut visible) {
            if visible {
                _array__grow(
                    &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                    1 as libc::c_int as uint32_t,
                    ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
                );
                let fresh6 = (*self_0).stack.size;
                (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
                *((*self_0).stack.contents).offset(fresh6 as isize) = entry;
                return TreeCursorStepVisible;
            }
            if ts_subtree_visible_child_count(*entry.subtree) != 0 {
                _array__grow(
                    &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                    1 as libc::c_int as uint32_t,
                    ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
                );
                let fresh7 = (*self_0).stack.size;
                (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
                *((*self_0).stack.contents).offset(fresh7 as isize) = entry;
                return TreeCursorStepHidden;
            }
        }
    }
    (*self_0).stack.size = initial_size;
    return TreeCursorStepNone;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_next_sibling_internal(
    mut _self: *mut TSTreeCursor,
) -> TreeCursorStep {
    return ts_tree_cursor_goto_sibling_internal(
        _self,
        Some(
            ts_tree_cursor_child_iterator_next
                as unsafe extern "C" fn(
                    *mut CursorChildIterator,
                    *mut TreeCursorEntry,
                    *mut bool,
                ) -> bool,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_next_sibling(mut self_0: *mut TSTreeCursor) -> bool {
    match ts_tree_cursor_goto_next_sibling_internal(self_0) as libc::c_uint {
        1 => {
            ts_tree_cursor_goto_first_child(self_0);
            return 1 as libc::c_int != 0;
        }
        2 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_previous_sibling_internal(
    mut _self: *mut TSTreeCursor,
) -> TreeCursorStep {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut step: TreeCursorStep = ts_tree_cursor_goto_sibling_internal(
        _self,
        Some(
            ts_tree_cursor_child_iterator_previous
                as unsafe extern "C" fn(
                    *mut CursorChildIterator,
                    *mut TreeCursorEntry,
                    *mut bool,
                ) -> bool,
        ),
    );
    if step as libc::c_uint == TreeCursorStepNone as libc::c_int as libc::c_uint {
        return step;
    }
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    if !length_is_undefined(
        (*&mut *((*self_0).stack.contents).offset(
            ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ))
            .position,
    ) {
        return step;
    }
    let mut parent: *const TreeCursorEntry = &mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry;
    let mut position: Length = (*parent).position;
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut child_index: uint32_t = (*(&mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry))
        .child_index;
    let mut children: *const Subtree =
        if ((*(*parent).subtree).data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            ((*(*parent).subtree).ptr as *mut Subtree)
                .offset(-((*(*(*parent).subtree).ptr).child_count as isize))
        };
    if child_index > 0 as libc::c_int as libc::c_uint {
        position = length_add(
            position,
            ts_subtree_size(*children.offset(0 as libc::c_int as isize)),
        );
        let mut i: uint32_t = 1 as libc::c_int as uint32_t;
        while i < child_index {
            position = length_add(
                position,
                ts_subtree_total_size(*children.offset(i as isize)),
            );
            i = i.wrapping_add(1);
        }
        position = length_add(
            position,
            ts_subtree_padding(*children.offset(child_index as isize)),
        );
    }
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    (*(&mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry))
        .position = position;
    return step;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_previous_sibling(
    mut self_0: *mut TSTreeCursor,
) -> bool {
    match ts_tree_cursor_goto_previous_sibling_internal(self_0) as libc::c_uint {
        1 => {
            ts_tree_cursor_goto_last_child(self_0);
            return 1 as libc::c_int != 0;
        }
        2 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_parent(mut _self: *mut TSTreeCursor) -> bool {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut i: libc::c_uint = ((*self_0).stack.size).wrapping_sub(2 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) > 0 as libc::c_int as libc::c_uint {
        if ts_tree_cursor_is_entry_visible(self_0, i) {
            (*self_0).stack.size = i.wrapping_add(1 as libc::c_int as libc::c_uint);
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_sub(1);
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_descendant(
    mut _self: *mut TSTreeCursor,
    mut goal_descendant_index: uint32_t,
) {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    loop {
        let mut i: uint32_t = ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
        let mut entry: *mut TreeCursorEntry =
            &mut *((*self_0).stack.contents).offset(i as isize) as *mut TreeCursorEntry;
        let mut next_descendant_index: uint32_t = ((*entry).descendant_index)
            .wrapping_add(
                (if ts_tree_cursor_is_entry_visible(self_0, i) as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            )
            .wrapping_add(ts_subtree_visible_descendant_count(*(*entry).subtree));
        if (*entry).descendant_index <= goal_descendant_index
            && next_descendant_index > goal_descendant_index
        {
            break;
        }
        if (*self_0).stack.size <= 1 as libc::c_int as libc::c_uint {
            return;
        } else {
            (*self_0).stack.size = ((*self_0).stack.size).wrapping_sub(1);
        }
    }
    let mut did_descend: bool = 1 as libc::c_int != 0;
    loop {
        did_descend = 0 as libc::c_int != 0;
        let mut visible: bool = false;
        let mut entry_0: TreeCursorEntry = TreeCursorEntry {
            subtree: 0 as *const Subtree,
            position: Length {
                bytes: 0,
                extent: TSPoint { row: 0, column: 0 },
            },
            child_index: 0,
            structural_child_index: 0,
            descendant_index: 0,
        };
        let mut iterator: CursorChildIterator = ts_tree_cursor_iterate_children(self_0);
        if iterator.descendant_index > goal_descendant_index {
            return;
        }
        while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry_0, &mut visible) {
            if !(iterator.descendant_index > goal_descendant_index) {
                continue;
            }
            _array__grow(
                &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut Array,
                1 as libc::c_int as uint32_t,
                ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
            );
            let fresh8 = (*self_0).stack.size;
            (*self_0).stack.size = ((*self_0).stack.size).wrapping_add(1);
            *((*self_0).stack.contents).offset(fresh8 as isize) = entry_0;
            if visible as libc::c_int != 0 && entry_0.descendant_index == goal_descendant_index {
                return;
            } else {
                did_descend = 1 as libc::c_int != 0;
                break;
            }
        }
        if !did_descend {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_descendant_index(
    mut _self: *const TSTreeCursor,
) -> uint32_t {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut last_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry;
    return (*last_entry).descendant_index;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_node(mut _self: *const TSTreeCursor) -> TSNode {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut last_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry;
    let mut alias_symbol: TSSymbol = (*self_0).root_alias_symbol;
    if (*self_0).stack.size > 1 as libc::c_int as libc::c_uint
        && !ts_subtree_extra(*(*last_entry).subtree)
    {
        let mut parent_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
            .offset(((*self_0).stack.size).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
            as *mut TreeCursorEntry;
        alias_symbol = ts_language_alias_at(
            (*(*self_0).tree).language,
            (*(*(*parent_entry).subtree).ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .production_id as uint32_t,
            (*last_entry).structural_child_index,
        );
    }
    return ts_node_new(
        (*self_0).tree,
        (*last_entry).subtree,
        (*last_entry).position,
        alias_symbol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_status(
    mut _self: *const TSTreeCursor,
    mut field_id: *mut TSFieldId,
    mut has_later_siblings: *mut bool,
    mut has_later_named_siblings: *mut bool,
    mut can_have_later_siblings_with_this_field: *mut bool,
    mut supertypes: *mut TSSymbol,
    mut supertype_count: *mut libc::c_uint,
) {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    let mut max_supertypes: libc::c_uint = *supertype_count;
    *field_id = 0 as libc::c_int as TSFieldId;
    *supertype_count = 0 as libc::c_int as libc::c_uint;
    *has_later_siblings = 0 as libc::c_int != 0;
    *has_later_named_siblings = 0 as libc::c_int != 0;
    *can_have_later_siblings_with_this_field = 0 as libc::c_int != 0;
    let mut i: libc::c_uint = ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > 0 as libc::c_int as libc::c_uint {
        let mut entry: *mut TreeCursorEntry =
            &mut *((*self_0).stack.contents).offset(i as isize) as *mut TreeCursorEntry;
        let mut parent_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as *mut TreeCursorEntry;
        let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
            (*(*self_0).tree).language,
            (*(*(*parent_entry).subtree).ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .production_id as uint32_t,
        );
        let mut entry_symbol: TSSymbol = (if !ts_subtree_extra(*(*entry).subtree)
            && !alias_sequence.is_null()
            && *alias_sequence.offset((*entry).structural_child_index as isize) as libc::c_int != 0
        {
            *alias_sequence.offset((*entry).structural_child_index as isize) as libc::c_int
        } else {
            ts_subtree_symbol(*(*entry).subtree) as libc::c_int
        }) as TSSymbol;
        let mut entry_metadata: TSSymbolMetadata =
            ts_language_symbol_metadata((*(*self_0).tree).language, entry_symbol);
        if i != ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            && entry_metadata.visible as libc::c_int != 0
        {
            break;
        }
        if entry_metadata.supertype as libc::c_int != 0 && *supertype_count < max_supertypes {
            *supertypes.offset(*supertype_count as isize) = entry_symbol;
            *supertype_count = (*supertype_count).wrapping_add(1);
        }
        if !*has_later_siblings {
            let mut sibling_count: libc::c_uint = (*(*(*parent_entry).subtree).ptr).child_count;
            let mut structural_child_index: libc::c_uint = (*entry).structural_child_index;
            if !ts_subtree_extra(*(*entry).subtree) {
                structural_child_index = structural_child_index.wrapping_add(1);
            }
            let mut j: libc::c_uint =
                ((*entry).child_index).wrapping_add(1 as libc::c_int as libc::c_uint);
            while j < sibling_count {
                let mut sibling: Subtree =
                    *if ((*(*parent_entry).subtree).data).is_inline() as libc::c_int != 0 {
                        0 as *mut Subtree
                    } else {
                        ((*(*parent_entry).subtree).ptr as *mut Subtree)
                            .offset(-((*(*(*parent_entry).subtree).ptr).child_count as isize))
                    }
                    .offset(j as isize);
                let mut sibling_metadata: TSSymbolMetadata = ts_language_symbol_metadata(
                    (*(*self_0).tree).language,
                    (if !ts_subtree_extra(sibling)
                        && !alias_sequence.is_null()
                        && *alias_sequence.offset(structural_child_index as isize) as libc::c_int
                            != 0
                    {
                        *alias_sequence.offset(structural_child_index as isize) as libc::c_int
                    } else {
                        ts_subtree_symbol(sibling) as libc::c_int
                    }) as TSSymbol,
                );
                if sibling_metadata.visible {
                    *has_later_siblings = 1 as libc::c_int != 0;
                    if *has_later_named_siblings {
                        break;
                    }
                    if sibling_metadata.named {
                        *has_later_named_siblings = 1 as libc::c_int != 0;
                        break;
                    }
                } else if ts_subtree_visible_child_count(sibling) > 0 as libc::c_int as libc::c_uint
                {
                    *has_later_siblings = 1 as libc::c_int != 0;
                    if *has_later_named_siblings {
                        break;
                    }
                    if (*sibling.ptr)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .named_child_count
                        > 0 as libc::c_int as libc::c_uint
                    {
                        *has_later_named_siblings = 1 as libc::c_int != 0;
                        break;
                    }
                }
                if !ts_subtree_extra(sibling) {
                    structural_child_index = structural_child_index.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
        }
        if !ts_subtree_extra(*(*entry).subtree) {
            let mut field_map: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
            let mut field_map_end: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
            ts_language_field_map(
                (*(*self_0).tree).language,
                (*(*(*parent_entry).subtree).ptr)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .production_id as uint32_t,
                &mut field_map,
                &mut field_map_end,
            );
            if *field_id == 0 {
                let mut map: *const TSFieldMapEntry = field_map;
                while map < field_map_end {
                    if !(*map).inherited
                        && (*map).child_index as libc::c_uint == (*entry).structural_child_index
                    {
                        *field_id = (*map).field_id;
                        break;
                    } else {
                        map = map.offset(1);
                    }
                }
            }
            if *field_id != 0 {
                let mut map_0: *const TSFieldMapEntry = field_map;
                while map_0 < field_map_end {
                    if (*map_0).field_id as libc::c_int == *field_id as libc::c_int
                        && (*map_0).child_index as libc::c_uint > (*entry).structural_child_index
                    {
                        *can_have_later_siblings_with_this_field = 1 as libc::c_int != 0;
                        break;
                    } else {
                        map_0 = map_0.offset(1);
                    }
                }
            }
        }
        i = i.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_depth(mut _self: *const TSTreeCursor) -> uint32_t {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    let mut depth: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while i < (*self_0).stack.size {
        if ts_tree_cursor_is_entry_visible(self_0, i) {
            depth = depth.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return depth;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_parent_node(mut _self: *const TSTreeCursor) -> TSNode {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    let mut i: libc::c_int = (*self_0).stack.size as libc::c_int - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut entry: *mut TreeCursorEntry =
            &mut *((*self_0).stack.contents).offset(i as isize) as *mut TreeCursorEntry;
        let mut is_visible: bool = 1 as libc::c_int != 0;
        let mut alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
        if i > 0 as libc::c_int {
            let mut parent_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
                .offset((i - 1 as libc::c_int) as isize)
                as *mut TreeCursorEntry;
            alias_symbol = ts_language_alias_at(
                (*(*self_0).tree).language,
                (*(*(*parent_entry).subtree).ptr)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .production_id as uint32_t,
                (*entry).structural_child_index,
            );
            is_visible = alias_symbol as libc::c_int != 0 as libc::c_int
                || ts_subtree_visible(*(*entry).subtree) as libc::c_int != 0;
        }
        if is_visible {
            return ts_node_new(
                (*self_0).tree,
                (*entry).subtree,
                (*entry).position,
                alias_symbol,
            );
        }
        i -= 1;
    }
    return ts_node_new(
        0 as *const TSTree,
        0 as *const Subtree,
        length_zero(),
        0 as libc::c_int as TSSymbol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_field_id(
    mut _self: *const TSTreeCursor,
) -> TSFieldId {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    let mut i: libc::c_uint = ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > 0 as libc::c_int as libc::c_uint {
        let mut entry: *mut TreeCursorEntry =
            &mut *((*self_0).stack.contents).offset(i as isize) as *mut TreeCursorEntry;
        let mut parent_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as *mut TreeCursorEntry;
        if i != ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            && ts_tree_cursor_is_entry_visible(self_0, i) as libc::c_int != 0
        {
            break;
        }
        if ts_subtree_extra(*(*entry).subtree) {
            break;
        }
        let mut field_map: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
        let mut field_map_end: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
        ts_language_field_map(
            (*(*self_0).tree).language,
            (*(*(*parent_entry).subtree).ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .production_id as uint32_t,
            &mut field_map,
            &mut field_map_end,
        );
        let mut map: *const TSFieldMapEntry = field_map;
        while map < field_map_end {
            if !(*map).inherited
                && (*map).child_index as libc::c_uint == (*entry).structural_child_index
            {
                return (*map).field_id;
            }
            map = map.offset(1);
        }
        i = i.wrapping_sub(1);
    }
    return 0 as libc::c_int as TSFieldId;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_field_name(
    mut _self: *const TSTreeCursor,
) -> *const libc::c_char {
    let mut id: TSFieldId = ts_tree_cursor_current_field_id(_self);
    if id != 0 {
        let mut self_0: *const TreeCursor = _self as *const TreeCursor;
        return *((*(*(*self_0).tree).language).field_names).offset(id as isize);
    } else {
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_copy(mut _cursor: *const TSTreeCursor) -> TSTreeCursor {
    let mut cursor: *const TreeCursor = _cursor as *const TreeCursor;
    let mut res: TSTreeCursor = {
        let mut init = TSTreeCursor {
            tree: 0 as *const libc::c_void,
            id: 0 as *const libc::c_void,
            context: [
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0,
            ],
        };
        init
    };
    let mut copy: *mut TreeCursor = &mut res as *mut TSTreeCursor as *mut TreeCursor;
    (*copy).tree = (*cursor).tree;
    (*copy).root_alias_symbol = (*cursor).root_alias_symbol;
    (*copy).stack.size = 0 as libc::c_int as uint32_t;
    (*copy).stack.capacity = 0 as libc::c_int as uint32_t;
    (*copy).stack.contents = 0 as *mut TreeCursorEntry;
    _array__splice(
        &mut (*copy).stack as *mut C2RustUnnamed_7 as *mut Array,
        ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
        (*copy).stack.size,
        0 as libc::c_int as uint32_t,
        (*cursor).stack.size,
        (*cursor).stack.contents as *const libc::c_void,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_reset_to(
    mut _dst: *mut TSTreeCursor,
    mut _src: *const TSTreeCursor,
) {
    let mut cursor: *const TreeCursor = _src as *const TreeCursor;
    let mut copy: *mut TreeCursor = _dst as *mut TreeCursor;
    (*copy).tree = (*cursor).tree;
    (*copy).root_alias_symbol = (*cursor).root_alias_symbol;
    (*copy).stack.size = 0 as libc::c_int as uint32_t;
    _array__splice(
        &mut (*copy).stack as *mut C2RustUnnamed_7 as *mut Array,
        ::core::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
        (*copy).stack.size,
        0 as libc::c_int as uint32_t,
        (*cursor).stack.size,
        (*cursor).stack.contents as *const libc::c_void,
    );
}
pub const TreeCursorStep_TreeCursorStepVisible: TreeCursorStep = TreeCursorStepVisible;
pub const TreeCursorStep_TreeCursorStepHidden: TreeCursorStep = TreeCursorStepHidden;
pub const TreeCursorStep_TreeCursorStepNone: TreeCursorStep = TreeCursorStepNone;
