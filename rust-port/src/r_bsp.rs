#![allow(non_upper_case_globals)]

use std::ffi::c_char;
use std::ffi::c_int;

use crate::r_bsp_bindings::clipangle;
use crate::r_bsp_bindings::fixed_t;
use crate::r_bsp_bindings::nodes;

use crate::r_bsp_bindings::ceilingplane;
use crate::r_bsp_bindings::floorplane;


use crate::r_bsp_bindings::rw_angle1;
use crate::r_bsp_bindings::segs;
use crate::r_bsp_bindings::sscount;
use crate::r_bsp_bindings::subsectors;
use crate::r_bsp_bindings::viewangletox;
use crate::r_bsp_bindings::viewangle;
use crate::r_bsp_bindings::viewwidth;
use crate::r_bsp_bindings::viewx;
use crate::r_bsp_bindings::viewy;
use crate::r_bsp_bindings::viewz;
use crate::r_bsp_bindings::angle_t;
use crate::r_bsp_bindings::drawseg_t;
use crate::r_bsp_bindings::line_t;
use crate::r_bsp_bindings::sector_t;
use crate::r_bsp_bindings::seg_t;
use crate::r_bsp_bindings::side_t;
use crate::r_bsp_bindings::subsector_t;
use crate::r_bsp_bindings::visplane_t;
use crate::r_bsp_bindings::node_t;

use crate::r_bsp_bindings::ANG90;
use crate::r_bsp_bindings::ANG180;
use crate::r_bsp_bindings::ANGLETOFINESHIFT;
use crate::r_bsp_bindings::MAXDRAWSEGS;
use crate::r_bsp_bindings::NF_SUBSECTOR;
use crate::r_bsp_bindings::SCREENWIDTH;

/* PORT LATER */

unsafe extern "C" {
    fn R_StoreWallRange(start: i32, stop: i32);
}

unsafe extern "C" {
    fn R_FindPlane(height: i32, picnum: i32, lightlevel: i32) -> *mut visplane_t;
}

unsafe extern "C" {
    unsafe fn R_PointToAngle(px: fixed_t, py: fixed_t) -> angle_t;
}

unsafe extern "C" {
    fn R_AddSprites(sec: *mut sector_t);
}

unsafe extern "C" {
    fn I_Error(error: *const c_char, ...);
}

unsafe extern "C" {
    fn R_PointOnSide(x: fixed_t, y: fixed_t, node: *mut node_t) -> i32;
}

unsafe extern "C" {
    static mut skyflatnum: c_int;
}

///////
/// This is my own implementation (there's alot of errors to be corrected in this code :D)
/// 
#[unsafe(no_mangle)]
pub static mut curline: *mut seg_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut sidedef: *mut side_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut linedef: *mut line_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut frontsector: *mut sector_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut backsector: *mut sector_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut drawsegs: [drawseg_t; MAXDRAWSEGS as usize] =
    [const { unsafe { std::mem::zeroed::<drawseg_t>() } }; MAXDRAWSEGS as usize];

#[unsafe(no_mangle)]
pub static mut ds_p: *mut drawseg_t = std::ptr::null_mut();

// nämä kuuluvat r_bsp.c tiedoston sisälle määriteltyihin
#[repr(C)]
#[derive(Clone, Copy)]
struct cliprange_t {
    first: i32,
    last: i32,
}

const BOXTOP: usize = 0;
const BOXBOTTOM: usize = 1;
const BOXLEFT: usize = 2;
const BOXRIGHT: usize = 3;

const MAXSEGS: usize = SCREENWIDTH as usize / 2 + 1;

static mut solidsegs: [cliprange_t; MAXSEGS as usize] =
    [const { cliprange_t { first: 0, last: 0 } }; MAXSEGS as usize];

static mut newend: *mut cliprange_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut checkcoord: [[i32; 4]; 12] = [
    [3, 0, 2, 1],
    [3, 0, 2, 0],
    [3, 1, 2, 0],
    [0, 0, 0, 0],
    [2, 0, 2, 1],
    [0, 0, 0, 0],
    [3, 1, 3, 0],
    [0, 0, 0, 0],
    [2, 0, 3, 1],
    [2, 1, 3, 1],
    [2, 1, 3, 0],
    [0, 0, 0, 0]
];

#[unsafe(no_mangle)]
extern "C" fn R_AddLine(line: *mut seg_t) {
    
    let mut angle1: angle_t = 0;
    let mut angle2: angle_t = 0;
    let mut span: angle_t = 0;
    let mut tspan: angle_t = 0;

    unsafe {
        curline = line;
    }

    unsafe {
        let x1: fixed_t = (*(*line).v1).x;
        let y1: fixed_t = (*(*line).v1).y;
        let x2: fixed_t = (*(*line).v2).x;
        let y2: fixed_t = (*(*line).v2).y;

        angle1 = R_PointToAngle(x1, y1);
        angle2 = R_PointToAngle(x2, y2);
        span = angle1.wrapping_sub(angle2);
        if span >= ANG180 {
            return;
        }
        // Global angle needed by segcalc
        rw_angle1 = angle1 as c_int;
        angle1 = angle1.wrapping_sub(viewangle);
        angle2 = angle2.wrapping_sub(viewangle);
        tspan = angle1.wrapping_add(clipangle);

        if tspan > clipangle.wrapping_mul(2) {
            tspan = tspan.wrapping_sub(clipangle.wrapping_mul(2));

            if tspan >= span {
                return;
            }

        angle1 = clipangle;
        }

        tspan = clipangle.wrapping_sub(angle2);
        if tspan > 2 * clipangle {
            tspan = tspan.wrapping_sub(2 * clipangle);
            //Totally off the left edge?
            if tspan >= span {
                return;
            }
            angle2 = clipangle.wrapping_neg();
        }
    
        // The seg is in the view range, but not necessarily visible
        angle1 = (angle1.wrapping_add(ANG90)) >> ANGLETOFINESHIFT;
        angle2 = (angle2.wrapping_add(ANG90)) >> ANGLETOFINESHIFT;
        let _x1 = viewangletox[angle1 as usize];
        let _x2 = viewangletox[angle2 as usize];
        
        if _x1 == _x2 {
            return;
        }

        backsector = (*line).backsector;

        if backsector == std::ptr::null_mut() {
            R_ClipSolidWallSegment (_x1, _x2 - 1);
            return;
        }

        if (*backsector).ceilingheight <= (*frontsector).floorheight 
        || (*backsector).floorheight >= (*frontsector).ceilingheight {
            R_ClipSolidWallSegment (_x1, _x2 - 1);
            return;
        }

        // window
        if (*backsector).ceilingheight != (*frontsector).ceilingheight
            || (*backsector).floorheight != (*frontsector).floorheight
        {
            R_ClipPassWallSegment(_x1, _x2 - 1);
            return;
        }

        if (*backsector).ceilingpic == (*frontsector).ceilingpic
        && (*backsector).floorpic == (*frontsector).floorpic
        && (*backsector).lightlevel == (*frontsector).lightlevel
        && (*(*curline).sidedef).midtexture == 0 {
            return;
        }

        else {
            R_ClipPassWallSegment(_x1, _x2 - 1);
            return;
        }

    }
}

#[unsafe(no_mangle)]
pub extern "C" fn R_ClearDrawSegs() {
    unsafe {
        ds_p = &raw mut drawsegs[0];
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn R_Subsector(num: i32) {
    unsafe {
        #[cfg(feature = "rangecheck")]
        {
            if num >= numsubsectors {
                I_Error(
                    c"R_Subsector: ss %i with numss = %i".as_ptr(),
                    num,
                    numsubsectors,
                );
            }
        }

        sscount += 1;

        let sub: *mut subsector_t = subsectors.add(num as usize);
        frontsector = (*sub).sector;

        let mut count: i32 = (*sub).numlines.into();
        let mut line: *mut seg_t = segs.add((*sub).firstline as usize);

        // Determine floor plane.
        if (*frontsector).floorheight < viewz {
            floorplane = R_FindPlane(
                (*frontsector).floorheight,
                (*frontsector).floorpic.into(),
                (*frontsector).lightlevel.into(),
            );
        } else {
            floorplane = std::ptr::null_mut();
        }

        // Determine ceiling plane.
        if (*frontsector).ceilingheight > viewz || (*frontsector).ceilingpic as c_int == skyflatnum {
            ceilingplane = R_FindPlane(
                (*frontsector).ceilingheight,
                (*frontsector).ceilingpic.into(),
                (*frontsector).lightlevel.into(),
            );
        } else {
            ceilingplane = std::ptr::null_mut();
        }

        R_AddSprites(frontsector);

        while count > 0 {
            count -= 1;
            R_AddLine(line);
            line = line.add(1);
        }

        // Check for solidsegs overflow.
        if (newend as usize) > ((&raw mut solidsegs[32]) as usize) {
            I_Error(c"R_Subsector: solidsegs overflow (vanilla may crash here)\n".as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn R_ClipPassWallSegment(first: i32, last: i32) {
    unsafe {
        let mut start: *mut cliprange_t = &raw mut solidsegs[0];
        while (*start).last < first - 1 {
            start = start.add(1);
        }

        if first < (*start).first {
            if last < (*start).first - 1 {
                R_StoreWallRange(first, last);
                return;
            }

            R_StoreWallRange(first, (*start).first - 1);
        }

        //Bottom contained in start?
        if last <= (*start).last {
            return;
        }

        while (last >= (*start.add(1)).first - 1) {
            R_StoreWallRange((*start).last + 1, (*start.add(1)).first - 1);

            start = start.add(1);
            if last <= (*start).last {
                return;
            }
        }

        R_StoreWallRange((*start).last + 1, last);
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn R_RenderBSPNode(bspnum: i32) {
    unsafe {
        if (bspnum & NF_SUBSECTOR as i32) != 0 {
            if bspnum == -1 {
                R_Subsector(0);
            } else {
                R_Subsector(bspnum & !(NF_SUBSECTOR as i32));
            }
            return;
        }

        let bsp: *mut node_t = nodes.add(bspnum as usize);

        let side: i32 = R_PointOnSide(viewx, viewy, bsp);

        // Front subtree
        R_RenderBSPNode((*bsp).children[side as usize] as i32);

        // Back subtree, if visible
        if R_CheckBBox((*bsp).bbox[(side ^ 1) as usize].as_mut_ptr()) != 0 {
            R_RenderBSPNode((*bsp).children[(side ^ 1) as usize] as i32);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn R_ClearClipSegs() {
    // Oma Rust-toteutus

    solidsegs[0].first = -0x7fffffff;
    solidsegs[0].last = -1;
    solidsegs[1].first = viewwidth as i32;
    solidsegs[1].last = 0x7fffffff;
    let solidsegs_ptr: *mut cliprange_t =
        unsafe { (&raw mut solidsegs).cast::<cliprange_t>().add(2) };
    unsafe {
        newend = solidsegs_ptr;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn R_ClipSolidWallSegment(first: i32, last: i32) {
    unsafe {
        let mut start: *mut cliprange_t = &raw mut solidsegs[0];
        let mut next: *mut cliprange_t;

        // Find the first range that touches the range.
        while (*start).last < first - 1 {
            start = start.add(1);
        }

        if first < (*start).first {
            if last < (*start).first - 1 {
                // Post is entirely visible.
                R_StoreWallRange(first, last);

                next = newend;
                newend = newend.add(1);

                while next != start {
                    *next = *next.sub(1);
                    next = next.sub(1);
                }

                (*next).first = first;
                (*next).last = last;
                return;
            }

            // There is a fragment above *start.
            R_StoreWallRange(first, (*start).first - 1);

            // Adjust the clip size.
            (*start).first = first;
        }

        // Bottom contained in start?
        if last <= (*start).last {
            return;
        }

        next = start;

        'crunch: {
            while last >= (*next.add(1)).first - 1 {
                // There is a fragment between two posts.
                R_StoreWallRange((*next).last + 1, (*next.add(1)).first - 1);

                next = next.add(1);

                if last <= (*next).last {
                    // Bottom is contained in next.
                    (*start).last = (*next).last;
                    break 'crunch;
                }
            }

            // There is a fragment after *next.
            R_StoreWallRange((*next).last + 1, last);

            // Adjust the clip size.
            (*start).last = last;
        }

        // Remove start+1 to next from the clip list.
        if next == start {
            return;
        }

        while next != newend {
            next = next.add(1);
            start = start.add(1);
            *start = *next;
        }

        newend = start.add(1);
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn R_CheckBBox(bspcoord: *mut fixed_t) -> i32 {
    unsafe {
        let boxx: i32 = if viewx <= *bspcoord.add(BOXLEFT as usize) {
            0
        } else if viewx < *bspcoord.add(BOXRIGHT as usize) {
            1
        } else {
            2
        };

        let boxy: i32 = if viewy >= *bspcoord.add(BOXTOP as usize) {
            0
        } else if viewy > *bspcoord.add(BOXBOTTOM as usize) {
            1
        } else {
            2
        };

        let boxpos = (boxy << 2) + boxx;

        if boxpos == 5 {
            return 1;
        }

        let coords = checkcoord[boxpos as usize];

        let x1 = *bspcoord.add(coords[0] as usize);
        let y1 = *bspcoord.add(coords[1] as usize);
        let x2 = *bspcoord.add(coords[2] as usize);
        let y2 = *bspcoord.add(coords[3] as usize);

        let mut angle1: angle_t =
            R_PointToAngle(x1, y1).wrapping_sub(viewangle);

        let mut angle2: angle_t =
            R_PointToAngle(x2, y2).wrapping_sub(viewangle);

        let span = angle1.wrapping_sub(angle2);

        if span >= ANG180 {
            return 1;
        }

        let double_clipangle = clipangle.wrapping_mul(2);

        let mut tspan = angle1.wrapping_add(clipangle);

        if tspan > double_clipangle {
            tspan = tspan.wrapping_sub(double_clipangle);

            if tspan >= span {
                return 0;
            }

            angle1 = clipangle;
        }

        tspan = clipangle.wrapping_sub(angle2);

        if tspan > double_clipangle {
            tspan = tspan.wrapping_sub(double_clipangle);

            if tspan >= span {
                return 0;
            }

            angle2 = clipangle.wrapping_neg();
        }

        angle1 = angle1.wrapping_add(ANG90) >> ANGLETOFINESHIFT;
        angle2 = angle2.wrapping_add(ANG90) >> ANGLETOFINESHIFT;

        let sx1: i32 = viewangletox[angle1 as usize];
        let mut sx2: i32 = viewangletox[angle2 as usize];

        if sx1 == sx2 {
            return 0;
        }

        sx2 -= 1;

        let mut start: *mut cliprange_t = &raw mut solidsegs[0];

        while (*start).last < sx2 {
            start = start.add(1);
        }

        if sx1 >= (*start).first && sx2 <= (*start).last {
            return 0;
        }

        1
    }
}
