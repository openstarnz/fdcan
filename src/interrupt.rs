//! Interrupt types.

use core::ops;

#[allow(unused_imports)] // for intra-doc links only
use crate::{FdCan, Rx};

macro_rules! declare_interrupts {
    ($([$name:ident, $index:literal, $doc:expr],)*) => {
        /// FdCAN interrupt sources.
        ///
        /// These can be individually enabled and disabled in the FdCAN
        /// peripheral. Note that each FdCAN peripheral only exposes 2
        /// interrupts to the microcontroller:
        ///
        /// FDCANx_INTR0,
        /// FDCANx_INTR1,
        ///
        /// The interrupts available on each line can be configured using the
        /// `[crate::config::FdCanConfig]` struct.
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[non_exhaustive]
        #[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
        pub enum Interrupt {
            $(
                #[doc = $doc]
                $name = 1 << $index
            ),*
        }

        paste::paste! {
            bitflags::bitflags! {
                /// A set of FdCAN interrupts.
                #[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
                pub struct Interrupts: u32 {
                    $(
                        #[doc = $doc]
                        const [< $name:snake:upper >] = 1 << $index;
                    )*
                }
            }
        }
    };
}

// interrupts for g0 g4 l5
#[cfg(feature = "fdcan_g0_g4_l5")]
declare_interrupts!(
    [RxFifo0NewMsg, 0, "Rx FIFO 0 has a new message"],
    [RxFifo0Full, 1, "Rx FIFO 0 is full"],
    [RxFifo0MsgLost, 2, "Rx FIFO 0 has lost a message"],
    [RxFifo1NewMsg, 3, "Rx FIFO 1 has a new message"],
    [RxFifo1Full, 4, "Rx FIFO 1 is full"],
    [RxFifo1MsgLost, 5, "Rx FIFO 1 has lost a message"],
    [
        RxHighPrio,
        6,
        "A High Priority Message has been flagged by a filter"
    ],
    [TxComplete, 7, "Transmit has been completed"],
    [TxCancel, 8, "Tx message has been cancelled"],
    [TxEmpty, 9, "Tx Fifo is empty"],
    [
        TxEventNew,
        10,
        "An new Event has been received in the Tx Event Fifo"
    ],
    [TxEventFull, 11, "The TxEvent Fifo is full"],
    [TxEventLost, 12, "An Tx Event has been lost"],
    [TsWrapAround, 13, "Timestamp wrap around has occurred"],
    [MsgRamAccessFailure, 14, "Message RAM access failure.
The flag is set when the Rx handler:
has not completed acceptance filtering or storage of an accepted message until the
arbitration field of the following message has been received. In this case acceptance
filtering or message storage is aborted and the Rx handler starts processing of the
following message. was unable to write a message to the message RAM. In this case
message storage is aborted.
In both cases the FIFO put index is not updated. The partly stored message is overwritten
when the next message is stored to this location.
The flag is also set when the Tx Handler was not able to read a message from the Message
RAM in time. In this case message transmission is aborted. In case of a Tx Handler access
failure the FDCAN is switched into Restricted operation Mode (see Restricted operation
mode)."],
    [TimeoutOccurred, 15, "Timeout Occurred"],
    [
        ErrLogOverflow,
        16,
        "Overflow of CAN error logging counter occurred"
    ],
    [ErrPassive, 17, "Errr Passive"],
    [WarningStatus, 18, "Warning Status"],
    [BusOff, 19, "Bus_Off status"],
    [WatchdogInt, 20, " Watchdog interrupt"],
    [
        ProtErrArbritation,
        21,
        "Protocol error in arbitration phase (nominal bit time is used)"
    ],
    [
        ProtErrData,
        22,
        "Protocol error in data phase (data bit time is used)"
    ],
    [ReservedAccess, 23, "Access to reserved address"],
);

// interrupts for h7
#[cfg(feature = "fdcan_h7")]
declare_interrupts!(
    [RxFifo0NewMsg, 0, "Rx FIFO 0 has a new message"],
    [RxFifo0Watermark, 1, "Rx FIFO 0 watermark reached"],
    [RxFifo0Full, 2, "Rx FIFO 0 is full"],
    [RxFifo0MsgLost, 3, "Rx FIFO 0 has lost a message"],

    [RxFifo1NewMsg, 4, "Rx FIFO 1 has a new message"],
    [RxFifo1Watermark, 5, "Rx FIFO 1 watermark reached"],
    [RxFifo1Full, 6, "Rx FIFO 1 is full"],
    [RxFifo1MsgLost, 7, "Rx FIFO 1 has lost a message"],

    [
        RxHighPrio,
        8,
        "A High Priority Message has been flagged by a filter"
    ],
    [TxComplete, 9, "Transmit has been completed"],
    [TxCancel, 10, "Tx message has been cancelled"],
    [TxEmpty, 11, "Tx Fifo is empty"],
    [
        TxEventNew,
        12,
        "An new Event has been received in the Tx Event Fifo"
    ],
    [TxWatermark, 13, "TxEvent FIFO watermark reached"],
    [TxEventFull, 14, "The TxEvent Fifo is full"],
    [TxEventLost, 15, "An Tx Event has been lost"],
    [TsWrapAround, 16, "Timestamp wrap around has occurred"],

    [MsgRamAccessFailure, 17, "Message RAM access failure.
The flag is set when the Rx handler:
has not completed acceptance filtering or storage of an accepted message until the
arbitration field of the following message has been received. In this case acceptance
filtering or message storage is aborted and the Rx handler starts processing of the
following message. was unable to write a message to the message RAM. In this case
message storage is aborted.
In both cases the FIFO put index is not updated. The partly stored message is overwritten
when the next message is stored to this location.
The flag is also set when the Tx Handler was not able to read a message from the Message
RAM in time. In this case message transmission is aborted. In case of a Tx Handler access
failure the FDCAN is switched into Restricted operation Mode (see Restricted operation
mode)."],
    [TimeoutOccurred, 18, "Timeout Occurred"],
    [
        ErrLogOverflow,
        22,
        "Overflow of CAN error logging counter occurred"
    ],
    [ErrPassive, 23, "Errr Passive"],
    [WarningStatus, 24, "Warning Status"],
    [BusOff, 25, "Bus_Off status"],
    [WatchdogInt, 26, " Watchdog interrupt"],
    [
        ProtErrArbritation,
        27,
        "Protocol error in arbitration phase (nominal bit time is used)"
    ],
    [
        ProtErrData,
        28,
        "Protocol error in data phase (data bit time is used)"
    ],
    [ReservedAccess, 29, "Access to reserved address"],
);

impl Interrupts {
    /// No Interrupt masks selected
    pub fn none() -> Self {
        Self::from_bits_truncate(0)
    }

    /// Translates this interrupt set into the value to write to the `ILS`
    /// (Interrupt Line Select) register to route these interrupts to line 1.
    ///
    /// On H7 the `ILS` register has one bit per interrupt, in the same layout
    /// as the `IE`/`IR` registers (and therefore the same layout as
    /// [`Interrupts`]), so this is the identity mapping.
    #[cfg(feature = "fdcan_h7")]
    pub(crate) fn ils_bits(self) -> u32 {
        self.bits()
    }

    /// Translates this interrupt set into the value to write to the `ILS`
    /// (Interrupt Line Select) register to route these interrupts to line 1.
    ///
    /// Unlike H7, the G0/G4/L5 `ILS` register does **not** have one bit per
    /// interrupt. Instead the interrupt sources are arranged into seven groups,
    /// each represented by a single "parent" bit in `ILS` (see the `FDCAN_ILS`
    /// register in RM0440). All interrupts within a group share an interrupt
    /// line and cannot be assigned individually.
    ///
    /// A group's parent bit is set whenever *any* of its member interrupts is
    /// present in `self`, so requesting line 1 for a single interrupt
    /// implicitly routes its entire group to line 1.
    #[cfg(feature = "fdcan_g0_g4_l5")]
    pub(crate) fn ils_bits(self) -> u32 {
        // Mapping of each `ILS` parent bit to the interrupts it groups, per the
        // `FDCAN_ILS` register description in RM0440.
        let groups = [
            // RXFIFO0
            (
                Interrupts::RX_FIFO0_NEW_MSG
                    | Interrupts::RX_FIFO0_FULL
                    | Interrupts::RX_FIFO0_MSG_LOST,
                1 << 0,
            ),
            // RXFIFO1
            (
                Interrupts::RX_FIFO1_NEW_MSG
                    | Interrupts::RX_FIFO1_FULL
                    | Interrupts::RX_FIFO1_MSG_LOST,
                1 << 1,
            ),
            // SMSG (status message)
            (
                Interrupts::RX_HIGH_PRIO
                    | Interrupts::TX_COMPLETE
                    | Interrupts::TX_CANCEL,
                1 << 2,
            ),
            // TFERR (Tx FIFO error)
            (
                Interrupts::TX_EMPTY
                    | Interrupts::TX_EVENT_NEW
                    | Interrupts::TX_EVENT_FULL
                    | Interrupts::TX_EVENT_LOST,
                1 << 3,
            ),
            // MISC
            (
                Interrupts::TS_WRAP_AROUND
                    | Interrupts::MSG_RAM_ACCESS_FAILURE
                    | Interrupts::TIMEOUT_OCCURRED,
                1 << 4,
            ),
            // BERR (bit and line error)
            (
                Interrupts::ERR_LOG_OVERFLOW | Interrupts::ERR_PASSIVE,
                1 << 5,
            ),
            // PERR (protocol error)
            (
                Interrupts::WARNING_STATUS
                    | Interrupts::BUS_OFF
                    | Interrupts::WATCHDOG_INT
                    | Interrupts::PROT_ERR_ARBRITATION
                    | Interrupts::PROT_ERR_DATA
                    | Interrupts::RESERVED_ACCESS,
                1 << 6,
            ),
        ];

        let mut ils = 0;
        for &(members, parent) in &groups {
            if self.intersects(members) {
                ils |= parent;
            }
        }
        ils
    }
}

impl From<Interrupt> for Interrupts {
    #[inline]
    fn from(i: Interrupt) -> Self {
        Self::from_bits_truncate(i as u32)
    }
}

/// Adds an interrupt to the interrupt set.
impl ops::BitOrAssign<Interrupt> for Interrupts {
    #[inline]
    fn bitor_assign(&mut self, rhs: Interrupt) {
        *self |= Self::from(rhs);
    }
}

/// There are two interrupt lines for the FdCan
/// The events linked to these can be configured
/// see `[config::FdCanConfig]`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InterruptLine {
    /// Interrupt Line 0
    _0 = 0,
    /// Interrupt Line 1
    _1 = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interrupt_flags() {
        assert_eq!(
            Interrupts::from(Interrupt::TxComplete),
            Interrupts::TX_COMPLETE
        );
        assert_eq!(Interrupts::from(Interrupt::TxEmpty), Interrupts::TX_EMPTY);

        let mut ints = Interrupts::RX_FIFO0_FULL;
        ints |= Interrupt::RxFifo1Full;
        assert_eq!(ints, Interrupts::RX_FIFO0_FULL | Interrupts::RX_FIFO1_FULL);
    }
}

#[cfg(all(test, feature = "fdcan_g0_g4_l5"))]
mod g0_g4_l5_ils_tests {
    use super::*;

    // ILS parent-bit positions, per the `FDCAN_ILS` register in RM0440.
    const RXFIFO0: u32 = 1 << 0;
    const RXFIFO1: u32 = 1 << 1;
    const SMSG: u32 = 1 << 2;
    const TFERR: u32 = 1 << 3;
    const MISC: u32 = 1 << 4;
    const BERR: u32 = 1 << 5;
    const PERR: u32 = 1 << 6;

    #[test]
    fn empty_set_maps_to_no_lines() {
        assert_eq!(Interrupts::none().ils_bits(), 0);
    }

    #[test]
    fn each_interrupt_maps_to_its_group() {
        let cases = [
            (Interrupt::RxFifo0NewMsg, RXFIFO0),
            (Interrupt::RxFifo0Full, RXFIFO0),
            (Interrupt::RxFifo0MsgLost, RXFIFO0),
            (Interrupt::RxFifo1NewMsg, RXFIFO1),
            (Interrupt::RxFifo1Full, RXFIFO1),
            (Interrupt::RxFifo1MsgLost, RXFIFO1),
            (Interrupt::RxHighPrio, SMSG),
            (Interrupt::TxComplete, SMSG),
            (Interrupt::TxCancel, SMSG),
            (Interrupt::TxEmpty, TFERR),
            (Interrupt::TxEventNew, TFERR),
            (Interrupt::TxEventFull, TFERR),
            (Interrupt::TxEventLost, TFERR),
            (Interrupt::TsWrapAround, MISC),
            (Interrupt::MsgRamAccessFailure, MISC),
            (Interrupt::TimeoutOccurred, MISC),
            (Interrupt::ErrLogOverflow, BERR),
            (Interrupt::ErrPassive, BERR),
            // RM0440 groups Warning Status and Bus Off under PERR, not BERR.
            (Interrupt::WarningStatus, PERR),
            (Interrupt::BusOff, PERR),
            (Interrupt::WatchdogInt, PERR),
            (Interrupt::ProtErrArbritation, PERR),
            (Interrupt::ProtErrData, PERR),
            (Interrupt::ReservedAccess, PERR),
        ];
        for (int, expected) in cases {
            assert_eq!(
                Interrupts::from(int).ils_bits(),
                expected,
                "{:?} mapped to the wrong ILS group",
                int
            );
        }
    }

    #[test]
    fn any_member_routes_the_whole_group() {
        // A single interrupt and the full group collapse to the same parent bit.
        let whole_rxfifo0 = Interrupts::RX_FIFO0_NEW_MSG
            | Interrupts::RX_FIFO0_FULL
            | Interrupts::RX_FIFO0_MSG_LOST;
        assert_eq!(whole_rxfifo0.ils_bits(), RXFIFO0);
        assert_eq!(Interrupts::RX_FIFO0_FULL.ils_bits(), RXFIFO0);
    }

    #[test]
    fn interrupts_in_different_groups_set_distinct_bits() {
        let ints = Interrupts::RX_FIFO0_NEW_MSG
            | Interrupts::TX_COMPLETE
            | Interrupts::ERR_PASSIVE
            | Interrupts::BUS_OFF;
        assert_eq!(ints.ils_bits(), RXFIFO0 | SMSG | BERR | PERR);
    }
}
