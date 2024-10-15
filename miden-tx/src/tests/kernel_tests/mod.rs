use alloc::string::String;

use miden_lib::transaction::memory::{
    NOTE_MEM_SIZE, NUM_OUTPUT_NOTES_PTR, OUTPUT_NOTE_ASSETS_OFFSET, OUTPUT_NOTE_METADATA_OFFSET,
    OUTPUT_NOTE_NUM_ASSETS_OFFSET, OUTPUT_NOTE_RECIPIENT_OFFSET, OUTPUT_NOTE_SECTION_OFFSET,
};
use miden_objects::{
    notes::Note,
    testing::{prepare_word, storage::prepare_assets},
    vm::StackInputs,
    Felt, Hasher, Word, ONE, ZERO,
};
use vm_processor::{ContextId, Host, Process, ProcessState};

mod test_account;
mod test_asset;
mod test_asset_vault;
mod test_epilogue;
mod test_faucet;
mod test_note;
mod test_prologue;
mod test_tx;

// HELPER MACROS
// ================================================================================================

#[macro_export]
macro_rules! assert_execution_error {
    ($execution_result:expr, $expected_err_code:expr) => {
        match $execution_result {
            Err(vm_processor::ExecutionError::FailedAssertion { clk: _, err_code, err_msg: _ }) => {
                assert!(
                    err_code == $expected_err_code,
                    "Execution failed on assertion with an unexpected error code (Actual err_code: {}, expected {}).",
                    err_code, $expected_err_code
                );
            },
            Ok(_) => panic!("Execution was unexpectedly successful"),
            Err(_) => panic!("Execution error was not as expected"),
        }
    };
}

// HELPER FUNCTIONS
// ================================================================================================

pub fn read_root_mem_value<H: Host>(process: &Process<H>, addr: u32) -> Word {
    process.get_mem_value(ContextId::root(), addr).unwrap()
}

pub fn try_read_root_mem_value<H: Host>(process: &Process<H>, addr: u32) -> Option<Word> {
    process.get_mem_value(ContextId::root(), addr)
}

pub fn output_notes_data_procedure(notes: &[Note]) -> String {
    let note_0_metadata = prepare_word(&notes[0].metadata().into());
    let note_0_recipient = prepare_word(&notes[0].recipient().digest());
    let note_0_assets = prepare_assets(notes[0].assets());
    let note_0_num_assets = 1;

    let note_1_metadata = prepare_word(&notes[1].metadata().into());
    let note_1_recipient = prepare_word(&notes[1].recipient().digest());
    let note_1_assets = prepare_assets(notes[1].assets());
    let note_1_num_assets = 1;

    let note_2_metadata = prepare_word(&notes[2].metadata().into());
    let note_2_recipient = prepare_word(&notes[2].recipient().digest());
    let note_2_assets = prepare_assets(notes[2].assets());
    let note_2_num_assets = 1;

    const NOTE_1_OFFSET: u32 = NOTE_MEM_SIZE;
    const NOTE_2_OFFSET: u32 = NOTE_MEM_SIZE * 2;

    format!(
        "
        proc.create_mock_notes
            # remove padding from prologue
            dropw dropw dropw dropw

            # populate note 0
            push.{note_0_metadata}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{OUTPUT_NOTE_METADATA_OFFSET} add mem_storew dropw

            push.{note_0_recipient}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{OUTPUT_NOTE_RECIPIENT_OFFSET} add mem_storew dropw

            push.{note_0_num_assets}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{OUTPUT_NOTE_NUM_ASSETS_OFFSET} add mem_store

            push.{}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{OUTPUT_NOTE_ASSETS_OFFSET} add mem_storew dropw

            # populate note 1
            push.{note_1_metadata}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_1_OFFSET}.{OUTPUT_NOTE_METADATA_OFFSET} add add mem_storew dropw

            push.{note_1_recipient}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_1_OFFSET}.{OUTPUT_NOTE_RECIPIENT_OFFSET} add add mem_storew dropw

            push.{note_1_num_assets}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_1_OFFSET}.{OUTPUT_NOTE_NUM_ASSETS_OFFSET} add add mem_store

            push.{}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_1_OFFSET}.{OUTPUT_NOTE_ASSETS_OFFSET} add add mem_storew dropw

            # populate note 2
            push.{note_2_metadata}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_2_OFFSET}.{OUTPUT_NOTE_METADATA_OFFSET} add add mem_storew dropw

            push.{note_2_recipient}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_2_OFFSET}.{OUTPUT_NOTE_RECIPIENT_OFFSET} add add mem_storew dropw

            push.{note_2_num_assets}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_2_OFFSET}.{OUTPUT_NOTE_NUM_ASSETS_OFFSET} add add mem_store

            push.{}
            push.{OUTPUT_NOTE_SECTION_OFFSET}.{NOTE_2_OFFSET}.{OUTPUT_NOTE_ASSETS_OFFSET} add add mem_storew dropw

            # set num output notes
            push.{}.{NUM_OUTPUT_NOTES_PTR} mem_store
        end
        ",
        note_0_assets[0],
        note_1_assets[0],
        note_2_assets[0],
        notes.len()
    )
}
