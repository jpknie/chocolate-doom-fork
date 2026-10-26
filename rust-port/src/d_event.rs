#![allow(non_upper_case_globals)]

use crate::d_event_bindings::event_t;

const MAXEVENTS: usize = 64;

static mut events: [event_t; 64] = 
        [const { unsafe { std::mem::zeroed::<event_t>() } }; 64];

static mut eventhead: usize = 0;
static mut eventtail: usize = 0;


#[unsafe(no_mangle)]
pub unsafe extern "C" fn D_PostEvent(ev: *mut event_t) {
 events[eventhead] = *ev;
 eventhead = (eventhead + 1) % MAXEVENTS;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn D_PopEvent() -> *mut event_t {
  let result: *mut event_t;

  unsafe {
    if eventtail == eventhead {
        return std::ptr::null_mut();
    }
  }

  result = &mut events[eventtail];

  eventtail = (eventtail + 1) % MAXEVENTS;

  return result;
}






