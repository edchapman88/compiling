#![no_std]

use core::fmt;
use core::sync::atomic::{self, Ordering};

/// Registers stacked (pushed onto the stack) during an exception.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

impl ExceptionFrame {
    /// Returns the value of (general purpose) register 0.
    #[inline(always)]
    pub fn r0(&self) -> u32 {
        self.r0
    }

    /// Returns the value of (general purpose) register 1.
    #[inline(always)]
    pub fn r1(&self) -> u32 {
        self.r1
    }

    /// Returns the value of (general purpose) register 2.
    #[inline(always)]
    pub fn r2(&self) -> u32 {
        self.r2
    }

    /// Returns the value of (general purpose) register 3.
    #[inline(always)]
    pub fn r3(&self) -> u32 {
        self.r3
    }

    /// Returns the value of (general purpose) register 12.
    #[inline(always)]
    pub fn r12(&self) -> u32 {
        self.r12
    }

    /// Returns the value of the Link Register.
    #[inline(always)]
    pub fn lr(&self) -> u32 {
        self.lr
    }

    /// Returns the value of the Program Counter.
    #[inline(always)]
    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Returns the value of the Program Status Register.
    #[inline(always)]
    pub fn xpsr(&self) -> u32 {
        self.xpsr
    }

    /// Sets the stacked value of (general purpose) register 0.
    ///
    /// # Safety
    ///
    /// This affects the `r0` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r0(&mut self, value: u32) {
        self.r0 = value;
    }

    /// Sets the stacked value of (general purpose) register 1.
    ///
    /// # Safety
    ///
    /// This affects the `r1` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r1(&mut self, value: u32) {
        self.r1 = value;
    }

    /// Sets the stacked value of (general purpose) register 2.
    ///
    /// # Safety
    ///
    /// This affects the `r2` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r2(&mut self, value: u32) {
        self.r2 = value;
    }

    /// Sets the stacked value of (general purpose) register 3.
    ///
    /// # Safety
    ///
    /// This affects the `r3` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r3(&mut self, value: u32) {
        self.r3 = value;
    }

    /// Sets the stacked value of (general purpose) register 12.
    ///
    /// # Safety
    ///
    /// This affects the `r12` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r12(&mut self, value: u32) {
        self.r12 = value;
    }

    /// Sets the stacked value of the Link Register.
    ///
    /// # Safety
    ///
    /// This affects the `lr` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_lr(&mut self, value: u32) {
        self.lr = value;
    }

    /// Sets the stacked value of the Program Counter.
    ///
    /// # Safety
    ///
    /// This affects the `pc` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    /// Sets the stacked value of the Program Status Register.
    ///
    /// # Safety
    ///
    /// This affects the `xPSR` registers (`IPSR`, `APSR`, and `EPSR`) of the preempted code, which
    /// must not rely on them getting restored to their previous value.
    #[inline(always)]
    pub unsafe fn set_xpsr(&mut self, value: u32) {
        self.xpsr = value;
    }
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("xpsr", &Hex(self.xpsr))
            .finish()
    }
}

// Entry point is Reset.
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[link_section = ".HardFault.default"]
#[no_mangle]
pub unsafe extern "C" fn HardFault_(_: &ExceptionFrame) -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

/* Exceptions */
pub enum Exception {
    NonMaskableInt,

    // Not overridable
    // HardFault,
    MemoryManagement,

    BusFault,

    UsageFault,

    SVCall,

    DebugMonitor,

    PendSV,

    SysTick,
}

// `extern "C" is used to declare function interfaces that allow Rust code to call code outside of
// Rust.
extern "C" {
    fn Reset() -> !;

    fn NonMaskableInt();

    fn HardFaultTrampoline();

    fn MemoryManagement();

    fn BusFault();

    fn UsageFault();

    fn SVCall();

    fn DebugMonitor();

    fn PendSV();

    fn SysTick();
}

pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector {
        handler: HardFaultTrampoline,
    },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: MemoryManagement,
    },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    Vector { handler: BusFault },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: DebugMonitor,
    },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];

// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 240];
