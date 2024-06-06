use std::{collections::HashSet, io::Write};

use super::{Instruction, Runtime};
use crate::runtime::Register;

pub fn align(addr: u32) -> u32 {
    addr - addr % 4
}

macro_rules! assert_valid_memory_access {
    ($addr:expr, $position:expr) => {
        #[cfg(debug_assertions)]
        {
            use p3_baby_bear::BabyBear;
            use p3_field::AbstractField;
            match $position {
                MemoryAccessPosition::Memory => {
                    assert_eq!($addr % 4, 0, "addr is not aligned");
                    BabyBear::from_canonical_u32($addr);
                    assert!($addr > 40);
                }
                _ => {
                    Register::from_u32($addr);
                }
            };
        }

        #[cfg(not(debug_assertions))]
        {}
    };
}

impl Runtime {
    #[inline]
    pub fn log(&mut self, instruction: &Instruction) {
        // Write the current program counter to the trace buffer for the cycle tracer.
        if let Some(ref mut buf) = self.trace_buf {
            if !self.unconstrained {
                buf.write_all(&u32::to_be_bytes(self.state.pc)).unwrap();
            }
        }
        let width = 12;
        let prev_dirty_regs = self
            .last_instructions
            .last()
            .map(|ins| ins.access_regs())
            .unwrap_or_default();
        let curr_dirty_regs = instruction.access_regs();
        let regs_status: String = curr_dirty_regs
            .iter()
            .chain(prev_dirty_regs.iter())
            .collect::<HashSet<&u32>>()
            .iter()
            .map(|reg| {
                format!(
                    "x{}={:<width$} ",
                    reg,
                    self.register(Register::from_u32(**reg))
                )
            })
            .collect::<Vec<String>>()
            .join(" | ");

        // If RUST_LOG is set to "trace", then log the current state of the runtime every cycle.
        log::trace!(
            "clk={} [pc=0x{:x?}] {:<width$?} |  {:?}",
            self.state.global_clk,
            self.state.pc,
            instruction,
            regs_status
        );

        if !self.unconstrained && self.state.global_clk % 10_000_000 == 0 {
            log::info!(
                "clk = {} pc = 0x{:x?}",
                self.state.global_clk,
                self.state.pc
            );
        }
    }
}
