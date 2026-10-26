#![allow(non_upper_case_globals)]

use crate::r_bsp_bindings::viewwidth;
use crate::r_bsp_bindings::SCREENWIDTH;
use crate::r_bsp_bindings::MAXDRAWSEGS;
use crate::r_bsp_bindings::seg_t;
use crate::r_bsp_bindings::side_t;
use crate::r_bsp_bindings::line_t;
use crate::r_bsp_bindings::sector_t;
use crate::r_bsp_bindings::drawseg_t;

/*

seg_t*		curline;
side_t*		sidedef;
line_t*		linedef;
sector_t*	frontsector;
sector_t*	backsector;

drawseg_t	drawsegs[MAXDRAWSEGS];
drawseg_t*	ds_p;


*/

#[unsafe(no_mangle)]
pub static mut      curline: *mut seg_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut      sidedef: *mut side_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut      linedef: *mut line_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut      frontsector: *mut sector_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut      backsector: *mut sector_t = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut      drawsegs: [drawseg_t; MAXDRAWSEGS as usize] = [const { unsafe { std::mem::zeroed::<drawseg_t>() } }; MAXDRAWSEGS as usize];

#[unsafe(no_mangle)]
pub static mut      ds_p: *mut drawseg_t = std::ptr::null_mut();


// nämä kuuluvat r_bsp.c tiedoston sisälle määriteltyihin
#[repr(C)]
struct cliprange_t {
    first: i32,
    last: i32,
}

const MAXSEGS: usize = SCREENWIDTH as usize / 2 + 1;

static mut     solidsegs: [cliprange_t; MAXSEGS as usize] = [const { cliprange_t { first: 0, last: 0 } }; MAXSEGS as usize];
static mut     newend: *mut cliprange_t = std::ptr::null_mut();


#[unsafe(no_mangle)]
extern "C" fn R_AddLine(line: *mut line_t) {
    // Oma Rust-toteutus
}


#[unsafe(no_mangle)]
pub extern "C" fn R_ClearDrawSegs() {
  unsafe {
    ds_p = &raw mut drawsegs[0];
  }
}


#[unsafe(no_mangle)]
pub extern "C" fn R_RenderBSPNode(node: i32) {
    // Oma Rust-toteutus
}

pub unsafe extern "C" fn R_ClearClipSegs() {
    // Oma Rust-toteutus
      
    solidsegs[0].first = -0x7fffffff;
    solidsegs[0].last = -1;
    solidsegs[1].first = viewwidth as i32;
    solidsegs[1].last = viewwidth as i32;
    let solidsegs_ptr: *mut cliprange_t = unsafe { (&raw mut solidsegs).cast::<cliprange_t>().add(2) };
    unsafe { newend = solidsegs_ptr; }
}

#[unsafe(no_mangle)]
pub extern "C" fn R_ClipSolidWallSegment(first: i32, last: i32) {
    let mut next: *mut cliprange_t = std::ptr::null_mut();
    let mut start: *mut cliprange_t = std::ptr::null_mut();

    unsafe {
        start = &raw mut solidsegs[0];
        while (*start).last < first - 1 {
            start = start.add(1);
        }
        if first < (*start).first {
            if last < (*start).first - 1 {

            }
        }
        // JATKUU
        
    }

}

/*
void
R_ClipSolidWallSegment
( int			first,
  int			last )
{
    cliprange_t*	next;
    cliprange_t*	start;

    // Find the first range that touches the range
    //  (adjacent pixels are touching).
    start = solidsegs;
    while (start->last < first-1)
	start++;

    if (first < start->first)
    {
	if (last < start->first-1)
	{
	    // Post is entirely visible (above start),
	    //  so insert a new clippost.
	    R_StoreWallRange (first, last);
	    next = newend;
	    newend++;
	    
	    while (next != start)
	    {
		*next = *(next-1);
		next--;
	    }
	    next->first = first;
	    next->last = last;
	    return;
	}
		
	// There is a fragment above *start.
	R_StoreWallRange (first, start->first - 1);
	// Now adjust the clip size.
	start->first = first;	
    }

    // Bottom contained in start?
    if (last <= start->last)
	return;			
		
    next = start;
    while (last >= (next+1)->first-1)
    {
	// There is a fragment between two posts.
	R_StoreWallRange (next->last + 1, (next+1)->first - 1);
	next++;
	
	if (last <= next->last)
	{
	    // Bottom is contained in next.
	    // Adjust the clip size.
	    start->last = next->last;	
	    goto crunch;
	}
    }
	
    // There is a fragment after *next.
    R_StoreWallRange (next->last + 1, last);
    // Adjust the clip size.
    start->last = last;
	
    // Remove start+1 to next from the clip list,
    // because start now covers their area.
  crunch:
    if (next == start)
    {
	// Post just extended past the bottom of one post.
	return;
    }
    

    while (next++ != newend)
    {
	// Remove a post.
	*++start = *next;
    }

    newend = start+1;
}

*/