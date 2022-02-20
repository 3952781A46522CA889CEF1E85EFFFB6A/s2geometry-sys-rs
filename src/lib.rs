#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("./bindings.rs");
pub type const_pointer = std::os::raw::c_void;

impl PartialEq for S1Angle {
    fn eq(&self, other: &Self) -> bool {
        self.radians_ == other.radians_
    }
}

unsafe impl Send for ctx {}

unsafe impl Sync for ctx {}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::ffi::CString;
    use crate::{S2CellId, S2CellIndex, S2LatLng};
    use crate::S2LatLng_FromDegrees;

    #[test]
    fn test1() {
        unsafe {
            let a = S2LatLng_FromDegrees(31.2, 112.3);
            let b = S2LatLng_FromDegrees(21.2, 112.4);
            assert_eq!(true, a.is_valid());
            assert_eq!(a.GetDistance(&b), a.GetDistance(&b));
        }
    }

    #[test]
    fn test2() {
        use crate::*;
        unsafe {
            // let mut cell_id = S2CellId_Begin(30);
            // let s2cell_index = S2CellIndex::new();
            // let mut cell_index = S2CellIndex::new();
            // // S2ClosestCellQuery_options();
            // // S2ClosestCellQuery_Options::from()
            // let a = S2LatLng_FromDegrees(31.2, 112.3);
            // let b = S2LatLng_FromDegrees(31.2, 112.3);
            // let c = S2LatLng_FromDegrees(31.2, 112.3);
            // let center = a.ToPoint();
            // let border = b.ToPoint();
            // let center = S2LatLng_ToPoint(&b);
            // let border = S2LatLng_ToPoint(&c);
            // let cell_id1 = S2CellId::new1(&a);
            // let cell_id2 = S2CellId::new1(&b);
            // cell_index.Add(cell_id1, 1);
            // cell_index.Add(cell_id2, 2);
            // let r = S1Angle::new(std::ptr::addr_of!(center), std::ptr::addr_of!(border));
            // let options = wrapper_init_S2ClosestCellQuery_Options();
            // let mut query = S2ClosestCellQuery::new(&cell_index, &options);
            // let mut indexer = S2RegionTermIndexer::new();
            // let cap = S2Cap::new(std::ptr::addr_of!(center), r);
            // let p = CString::new("dsa").unwrap();
            // let view = wrapper_string_view_init(p.as_ptr());
            // indexer.GetIndexTerms(&(cap._base), view);
        }
    }

    #[test]
    fn test3() {
        use crate::*;
        unsafe {
            let mut ctx1 = ctx::new();
            let points: Vec<f32> = vec![112.1, 31.1, 114.2, 31.2, 116.3, 31.3];
            let mut result_len = 0usize;
            ctx1.load(points.as_ptr(), points.len());
            let r = ctx1.search(114.1, 31.2, 100.5, result_len.borrow_mut()) as *mut i32;
            let results_indexes = std::slice::from_raw_parts_mut(r as *mut i32, result_len);
            assert_eq!(results_indexes, vec![1]);
            ctx1.clear();
        }
    }
}