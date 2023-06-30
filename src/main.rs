use std::io::Error;

// use windows::core::GUID;
use windows::{
    core::GUID,
    Win32::System::{Com::*, Ole::VariantInit, TaskScheduler::*},
};

fn main() -> Result<(), Error> {
    unsafe {
        // solve Error: Os { code: -2147467262, kind: Uncategorized, message: "No such interface supported" }
        // @link https://github.com/microsoft/windows-rs/issues/1946
        #[allow(non_upper_case_globals)]
        const CLSID_TaskService: GUID = GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd); // or use CLSIDFromProgID(w!("Schedule.Service"))

        //  Initialize COM.
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
        // set general COM security levels.
        // TODO handle failure
        CoInitializeSecurity(
            None,
            -1,
            None,
            None,
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOLE_AUTHENTICATION_CAPABILITIES(0),
            None,
        )?;

        //  Create a name for the task.
        // todo!();
        //  Get the windows directory and set the path to notepad.exe.
        // todo!();

        // ---------
        // create instance of task service
        let service: ITaskService =
            CoCreateInstance(&CLSID_TaskService, None, CLSCTX_INPROC_SERVER)?;

        service.Connect(VariantInit(), VariantInit(), VariantInit(), VariantInit())?;

        let name = service.Connected()?;
        println!("{}", name.as_bool());
        let u = service.ConnectedUser()?;
        println!("🪵 [main.rs:46]~ token ~ \x1b[0;32mu\x1b[0m = {}", u);

        // connect to task service
    }
    return Ok(());
}
