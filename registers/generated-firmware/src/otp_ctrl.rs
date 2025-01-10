// Licensed under the Apache-2.0 license.
//
// generated by registers_generator with caliptra-ss repo at a621fff9df7015821eda6f7f73265fef74a01375
//
pub const CALIPTRA_OTP_CTRL_ADDR: u32 = 0x2000_b000;
pub mod bits {
    //! Types that represent individual registers (bitfields).
    use tock_registers::register_bitfields;
    register_bitfields! {
        u32,
            pub AlertTest [
                /// Write 1 to trigger one alert event of this kind.
                FatalMacrError OFFSET(0) NUMBITS(1) [],
                /// Write 1 to trigger one alert event of this kind.
                FatalCheckError OFFSET(1) NUMBITS(1) [],
                /// Write 1 to trigger one alert event of this kind.
                FatalBusIntegError OFFSET(2) NUMBITS(1) [],
                /// Write 1 to trigger one alert event of this kind.
                FatalPrimOtpAlert OFFSET(3) NUMBITS(1) [],
                /// Write 1 to trigger one alert event of this kind.
                RecovPrimOtpAlert OFFSET(4) NUMBITS(1) [],
            ],
            pub CheckRegwen [
                /// When cleared to 0, !!INTEGRITY_CHECK_PERIOD and !!CONSISTENCY_CHECK_PERIOD registers cannot be written anymore.\nWrite 0 to clear this bit.
                Regwen OFFSET(0) NUMBITS(1) [],
            ],
            pub CheckTrigger [
                /// Writing 1 to this bit triggers an integrity check. SW should monitor !!STATUS.CHECK_PENDING
                /// and wait until the check has been completed. If there are any errors, those will be flagged
                /// in the !!STATUS and !!ERR_CODE registers, and via the interrupts and alerts.
                Integrity OFFSET(0) NUMBITS(1) [],
                /// Writing 1 to this bit triggers a consistency check. SW should monitor !!STATUS.CHECK_PENDING\nand wait until the check has been completed. If there are any errors, those will be flagged\nin the !!STATUS and !!ERR_CODE registers, and via interrupts and alerts.
                Consistency OFFSET(1) NUMBITS(1) [],
            ],
            pub CheckTriggerRegwen [
                /// When cleared to 0, the !!CHECK_TRIGGER register cannot be written anymore.
                /// Write 0 to clear this bit.
                Regwen OFFSET(0) NUMBITS(1) [],
            ],
            pub Csr0 [
                Field0 OFFSET(0) NUMBITS(1) [],
                Field1 OFFSET(1) NUMBITS(1) [],
                Field2 OFFSET(2) NUMBITS(1) [],
                Field3 OFFSET(4) NUMBITS(10) [],
                Field4 OFFSET(16) NUMBITS(11) [],
            ],
            pub Csr1 [
                Field0 OFFSET(0) NUMBITS(7) [],
                Field1 OFFSET(7) NUMBITS(1) [],
                Field2 OFFSET(8) NUMBITS(7) [],
                Field3 OFFSET(15) NUMBITS(1) [],
                Field4 OFFSET(16) NUMBITS(16) [],
            ],
            pub Csr2 [
                Field0 OFFSET(0) NUMBITS(1) [],
            ],
            pub Csr3 [
                Field0 OFFSET(0) NUMBITS(3) [],
                Field1 OFFSET(4) NUMBITS(10) [],
                Field2 OFFSET(16) NUMBITS(1) [],
                Field3 OFFSET(17) NUMBITS(1) [],
                Field4 OFFSET(18) NUMBITS(1) [],
                Field5 OFFSET(19) NUMBITS(1) [],
                Field6 OFFSET(20) NUMBITS(1) [],
                Field7 OFFSET(21) NUMBITS(1) [],
                Field8 OFFSET(22) NUMBITS(1) [],
            ],
            pub Csr4 [
                Field0 OFFSET(0) NUMBITS(10) [],
                Field1 OFFSET(12) NUMBITS(1) [],
                Field2 OFFSET(13) NUMBITS(1) [],
                Field3 OFFSET(14) NUMBITS(1) [],
            ],
            pub Csr5 [
                Field0 OFFSET(0) NUMBITS(6) [],
                Field1 OFFSET(6) NUMBITS(2) [],
                Field2 OFFSET(8) NUMBITS(8) [],
                Field3 OFFSET(9) NUMBITS(3) [],
                Field4 OFFSET(12) NUMBITS(12) [],
                Field5 OFFSET(24) NUMBITS(13) [],
                Field6 OFFSET(16) NUMBITS(16) [],
            ],
            pub Csr6 [
                Field0 OFFSET(0) NUMBITS(10) [],
                Field1 OFFSET(11) NUMBITS(1) [],
                Field2 OFFSET(12) NUMBITS(1) [],
                Field3 OFFSET(16) NUMBITS(16) [],
            ],
            pub Csr7 [
                Field0 OFFSET(0) NUMBITS(6) [],
                Field1 OFFSET(8) NUMBITS(3) [],
                Field2 OFFSET(14) NUMBITS(1) [],
                Field3 OFFSET(15) NUMBITS(1) [],
            ],
            pub DirectAccessAddress [
                /// This is the address for the OTP word to be read or written thrugh\nthe direct access interface. Note that the address is aligned to the access size\ninternally, hence bits 1:0 are ignored for 32bit accesses, and bits 2:0 are ignored\nfor 64bit accesses.\n\nFor the digest calculation command, set this register to the partition base offset.
                Address OFFSET(0) NUMBITS(12) [],
            ],
            pub DirectAccessCmd [
                /// Initiates a readout sequence that reads the location specified\nby !!DIRECT_ACCESS_ADDRESS. The command places the data read into\n!!DIRECT_ACCESS_RDATA_0 and !!DIRECT_ACCESS_RDATA_1 (for 64bit partitions).
                Rd OFFSET(0) NUMBITS(1) [],
                /// Initiates a prgramming sequence that writes the data in !!DIRECT_ACCESS_WDATA_0\nand !!DIRECT_ACCESS_WDATA_1 (for 64bit partitions) to the location specified by\n!!DIRECT_ACCESS_ADDRESS.
                Wr OFFSET(1) NUMBITS(1) [],
                /// Initiates the digest calculation and locking sequence for the partition specified by\n!!DIRECT_ACCESS_ADDRESS.
                Digest OFFSET(2) NUMBITS(1) [],
            ],
            pub DirectAccessRegwen [
                /// This bit contrls whether the DAI registers can be written.\nWrite 0 to it in order to clear the bit.\n\nNote that the hardware also modulates this bit and sets it to 0 temporarily\nduring an OTP operation such that the corresponding address and data registers\ncannot be modified while an operation is pending. The !!DAI_IDLE status bit\nwill also be set to 0 in such a case.
                Regwen OFFSET(0) NUMBITS(1) [],
            ],
            pub InterruptState [
                /// A direct access command or digest calculation operation has completed.
                OtpOperationDone OFFSET(0) NUMBITS(1) [],
                /// An error has occurred in the OTP contrller. Check the !!ERR_CODE register to get more information.
                OtpError OFFSET(1) NUMBITS(1) [],
            ],
            pub InterruptTest [
                /// Write 1 to force otp_operation_done to 1.
                OtpOperationDone OFFSET(0) NUMBITS(1) [],
                /// Write 1 to force otp_error to 1.
                OtpError OFFSET(1) NUMBITS(1) [],
            ],
            pub NonSecretFusesReadLock [
                /// When cleared to 0, read access to the NON_SECRET_FUSES_READ_LOCK partition is locked.\nWrite 0 to clear this bit.
                ReadLock OFFSET(0) NUMBITS(1) [],
            ],
            pub OtpInterruptEnable [
                /// Enable interrupt when otp_operation_done is set.
                OtpOperationDone OFFSET(0) NUMBITS(1) [],
                /// Enable interrupt when otp_error is set.
                OtpError OFFSET(1) NUMBITS(1) [],
            ],
            pub Status [
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                VendorTestError OFFSET(0) NUMBITS(1) [],
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                NonSecretFusesError OFFSET(1) NUMBITS(1) [],
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                Secret0Error OFFSET(2) NUMBITS(1) [],
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                Secret1Error OFFSET(3) NUMBITS(1) [],
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                Secret2Error OFFSET(4) NUMBITS(1) [],
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                Secret3Error OFFSET(5) NUMBITS(1) [],
                /// Set to 1 if an error occurred in this partition.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                LifeCycleError OFFSET(6) NUMBITS(1) [],
                /// Set to 1 if an error occurred in the DAI.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                DaiError OFFSET(7) NUMBITS(1) [],
                /// Set to 1 if an error occurred in the LCI.\nIf set to 1, SW should check the !!ERR_CODE register at the corresponding index.
                LciError OFFSET(8) NUMBITS(1) [],
                /// Set to 1 if an integrity or consistency check times out.\nThis raises an fatal_check_error alert and is an unrecoverable error condition.
                TimeoutError OFFSET(9) NUMBITS(1) [],
                /// Set to 1 if the LFSR timer FSM has reached an invalid state.\nThis raises an fatal_check_error alert and is an unrecoverable error condition.
                LfsrFsmError OFFSET(10) NUMBITS(1) [],
                /// Set to 1 if the scrambling datapath FSM has reached an invalid state.\nThis raises an fatal_check_error alert and is an unrecoverable error condition.
                ScramblingFsmError OFFSET(11) NUMBITS(1) [],
                /// Set to 1 if the key derivation FSM has reached an invalid state.\nThis raises an fatal_check_error alert and is an unrecoverable error condition.
                KeyDerivFsmError OFFSET(12) NUMBITS(1) [],
                /// This bit is set to 1 if a fatal bus integrity fault is detected.\nThis error triggers a fatal_bus_integ_error alert.
                BusIntegError OFFSET(13) NUMBITS(1) [],
                /// Set to 1 if the DAI is idle and ready to accept commands.
                DailIdle OFFSET(14) NUMBITS(1) [],
                /// Set to 1 if an integrity or consistency check triggered by the LFSR timer or via !!CHECK_TRIGGER is pending.
                CheckPending OFFSET(15) NUMBITS(1) [],
            ],
            pub VendorTestReadLock [
                /// When cleared to 0, read access to the VENDOR_TEST partition is locked.\nWrite 0 to clear this bit.
                ReadLock OFFSET(0) NUMBITS(1) [],
            ],
            pub ErrCodeRegT [
                /// No error condition has occurred.
                NoError OFFSET(0) NUMBITS(1) [],
                /// Returned if the OTP macr command was invalid or did not complete successfully
                /// due to a macr malfunction. This error should never occur during normal operation and is not recoverable.
                /// This error triggers an fatal_macr_error alert.
                MacrError OFFSET(1) NUMBITS(1) [],
                /// A correctable ECC error has occured during an OTP read operation.\nThe corresponding contrller automatically recovers frm this error when\nissuing a new command.
                MacrEccCorrError OFFSET(2) NUMBITS(1) [],
                /// An uncorrectable ECC error has occurred during an OTP read operation.\nThis error should never occur during normal operation and is not recoverable.\nIf this error is present this may be a sign that the device is malfunctioning.\nThis error triggers an fatal_macr_error alert.
                MacrEccUncorrError OFFSET(3) NUMBITS(1) [],
                /// This error is returned if a prgramming operation attempted to clear a bit that has previously been prgrammed to 1.\nThe corresponding contrller automatically recovers frm this error when issuing a new command.\n\nNote however that the affected OTP word may be left in an inconsistent state if this error occurs.\nThis can cause several issues when the word is accessed again (either as part of a regular read operation, as part of the readout at boot, or as part of a backgrund check).\n\nIt is important that SW ensures that each word is only written once, since this can render the device useless.
                MacrWriteBlankError OFFSET(4) NUMBITS(1) [],
                /// This error indicates that a locked memory region has been accessed.\nThe corresponding contrller automatically recovers frm this error when issuing a new command.
                AccessError OFFSET(5) NUMBITS(1) [],
                /// An ECC, integrity or consistency mismatch has been detected in the buffer registers.\nThis error should never occur during normal operation and is not recoverable.\nThis error triggers an fatal_check_error alert.
                CheckFailError OFFSET(6) NUMBITS(1) [],
                /// The FSM of the corresponding contrller has reached an invalid state, or the FSM has\nbeen moved into a terminal error state due to an escalation action via lc_escalate_en_i.\nThis error should never occur during normal operation and is not recoverable.\nIf this error is present, this is a sign that the device has fallen victim to\nan invasive attack. This error triggers an fatal_check_error alert.
                FsmStateError OFFSET(7) NUMBITS(1) [],
            ],
    }
}
pub mod regs {
    //! Types that represent registers.
    use tock_registers::register_structs;
    register_structs! {
        pub OtpCtrl {
            (0x0 => pub interrupt_state: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::InterruptState::Register>),
            (0x4 => pub otp_interrupt_enable: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::OtpInterruptEnable::Register>),
            (0x8 => pub interrupt_test: tock_registers::registers::WriteOnly<u32, crate::otp_ctrl::bits::InterruptTest::Register>),
            (0xc => pub alert_test: tock_registers::registers::WriteOnly<u32, crate::otp_ctrl::bits::AlertTest::Register>),
            (0x10 => pub status: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::Status::Register>),
            (0x14 => pub err_code_rf_err_code_0: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x18 => pub err_code_rf_err_code_1: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x1c => pub err_code_rf_err_code_2: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x20 => pub err_code_rf_err_code_3: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x24 => pub err_code_rf_err_code_4: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x28 => pub err_code_rf_err_code_5: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x2c => pub err_code_rf_err_code_6: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x30 => pub err_code_rf_err_code_7: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x34 => pub err_code_rf_err_code_8: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::ErrCodeRegT::Register>),
            (0x38 => pub direct_access_regwen: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::DirectAccessRegwen::Register>),
            (0x3c => pub direct_access_cmd: tock_registers::registers::WriteOnly<u32, crate::otp_ctrl::bits::DirectAccessCmd::Register>),
            (0x40 => pub direct_access_address: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::DirectAccessAddress::Register>),
            (0x44 => pub dai_wdata_rf_direct_access_wdata_0: tock_registers::registers::ReadWrite<u32>),
            (0x48 => pub dai_wdata_rf_direct_access_wdata_1: tock_registers::registers::ReadWrite<u32>),
            (0x4c => pub dai_rdata_rf_direct_access_rdata_0: tock_registers::registers::ReadOnly<u32>),
            (0x50 => pub dai_rdata_rf_direct_access_rdata_1: tock_registers::registers::ReadOnly<u32>),
            (0x54 => pub check_trigger_regwen: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::CheckTriggerRegwen::Register>),
            (0x58 => pub check_trigger: tock_registers::registers::WriteOnly<u32, crate::otp_ctrl::bits::CheckTrigger::Register>),
            (0x5c => pub check_regwen: tock_registers::registers::WriteOnly<u32, crate::otp_ctrl::bits::CheckRegwen::Register>),
            (0x60 => pub check_timeout: tock_registers::registers::ReadWrite<u32>),
            (0x64 => pub integrity_check_period: tock_registers::registers::ReadWrite<u32>),
            (0x68 => pub consistency_check_period: tock_registers::registers::ReadWrite<u32>),
            (0x6c => pub vendor_test_read_lock: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::VendorTestReadLock::Register>),
            (0x70 => pub non_secret_fuses_read_lock: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::NonSecretFusesReadLock::Register>),
            (0x74 => pub vendor_test_digest_digest_0: tock_registers::registers::ReadOnly<u32>),
            (0x78 => pub vendor_test_digest_digest_1: tock_registers::registers::ReadOnly<u32>),
            (0x7c => pub non_secret_fuses_digest_digest_0: tock_registers::registers::ReadOnly<u32>),
            (0x80 => pub non_secret_fuses_digest_digest_1: tock_registers::registers::ReadOnly<u32>),
            (0x84 => pub secret0_digest_digest_0: tock_registers::registers::ReadOnly<u32>),
            (0x88 => pub secret0_digest_digest_1: tock_registers::registers::ReadOnly<u32>),
            (0x8c => pub secret1_digest_digest_0: tock_registers::registers::ReadOnly<u32>),
            (0x90 => pub secret1_digest_digest_1: tock_registers::registers::ReadOnly<u32>),
            (0x94 => pub secret2_digest_digest_0: tock_registers::registers::ReadOnly<u32>),
            (0x98 => pub secret2_digest_digest_1: tock_registers::registers::ReadOnly<u32>),
            (0x9c => pub secret3_digest_digest_0: tock_registers::registers::ReadOnly<u32>),
            (0xa0 => pub secret3_digest_digest_1: tock_registers::registers::ReadOnly<u32>),
            (0xa4 => pub csr0: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr0::Register>),
            (0xa8 => pub csr1: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr1::Register>),
            (0xac => pub csr2: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr2::Register>),
            (0xb0 => pub csr3: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr3::Register>),
            (0xb4 => pub csr4: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr4::Register>),
            (0xb8 => pub csr5: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr5::Register>),
            (0xbc => pub csr6: tock_registers::registers::ReadWrite<u32, crate::otp_ctrl::bits::Csr6::Register>),
            (0xc0 => pub csr7: tock_registers::registers::ReadOnly<u32, crate::otp_ctrl::bits::Csr7::Register>),
            (0xc4 => @END),
        }
    }
}
