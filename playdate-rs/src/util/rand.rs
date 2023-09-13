use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use rand::{rngs::SmallRng, SeedableRng};

use crate::PLAYDATE;

/// Get global random number generator.
pub fn rng() -> impl rand::Rng {
    unsafe { &mut *RNG }
}

fn generate_seed() -> u64 {
    static mut COUNTER: u64 = 0;
    let mut seed = unsafe {
        COUNTER += 1;
        COUNTER
    };
    seed += PLAYDATE.system.get_timezone_offset().unsigned_abs() as u64;
    // let x = self.get_accelerometer();
    // seed += x.0 as u64;
    // seed += x.1 as u64;
    // seed += x.2 as u64;
    seed += PLAYDATE.system.get_current_time_milliseconds() as u64;
    seed += PLAYDATE.system.get_elapsed_time() as u64;
    seed += PLAYDATE.system.get_battery_percentage() as u64;
    seed += PLAYDATE.system.get_crank_angle() as u64;
    seed += PLAYDATE.system.get_button_state().current.bits() as u64;
    seed
}

static mut RNG: Rng = Rng {
    _p: UnsafeCell::new(None),
};

struct Rng {
    _p: UnsafeCell<Option<SmallRng>>,
}

unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}

impl Rng {
    unsafe fn get(&self) -> *mut SmallRng {
        let ptr = &mut *self._p.get();
        if (*ptr).is_none() {
            *ptr = Some(SmallRng::seed_from_u64(generate_seed()));
        }
        ptr.as_mut().unwrap()
    }
}

impl Deref for Rng {
    type Target = SmallRng;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.get() }
    }
}

impl DerefMut for Rng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.get() }
    }
}
