use std::io::Error;
use windows::{
    core::{ComInterface, BSTR, GUID},
    Win32::System::{Com::*, Ole::VariantInit, TaskScheduler::*},
};

pub fn win_schedule_daily() -> Result<(), Error> {
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

        // connect task service
        service.Connect(VariantInit(), VariantInit(), VariantInit(), VariantInit())?;

        //  ------------------------------------------------------
        //  Get the pointer to the root task folder.  This folder will hold the
        //  new task that is registered.
        let folder: ITaskFolder = service.GetFolder(&BSTR::from(r"\"))?;
        let tasks = folder.GetTasks(TASK_ENUM_HIDDEN.0)?;

        let count = tasks.Count()?;
        println!("Number of tasks in root folder: {count}");

        let task_name = BSTR::from(r"RunGHUBDAily");
        // if the same task exists remove it
        // folder.DeleteTask(&task_name, 0)?;

        //  Create the task definition object to create the task.
        let task = service.NewTask(0)?;

        //  ------------------------------------------------------
        //  Get the registration info for setting the identification.
        let reg_info = task.RegistrationInfo()?;

        reg_info.SetAuthor(&BSTR::from("nagy-nabil"))?;

        //  ------------------------------------------------------
        //  Create the principal for the task - these credentials
        //  are overwritten with the credentials passed to RegisterTaskDefinition
        // let principal = task.Principal()?;

        //  Set up principal logon type to interactive logon
        // principal.SetLogonType(TASK_LOGON_INTERACTIVE_TOKEN)?;

        //  ------------------------------------------------------
        //  Create the settings for the task
        // let task_settings = task.Settings()?;

        // //  Set setting values for the task.
        // task_settings.SetStartWhenAvailable(VARIANT_TRUE)?;

        //  ------------------------------------------------------
        //  Get the trigger collection to insert the daily trigger
        let trigger_collection = task.Triggers()?;

        //  Add the time trigger to the task.
        let trigger = trigger_collection.Create(TASK_TRIGGER_DAILY)?;
        let daily_trigger = trigger.cast::<IDailyTrigger>()?;
        daily_trigger.SetId(&BSTR::from(r"TriggerDailyNagyus"))?;

        daily_trigger.SetStartBoundary(&BSTR::from(r"2023-05-02T03:00:00"))?;
        daily_trigger.SetEndBoundary(&BSTR::from(r"2024-05-02T03:00:00"))?;
        daily_trigger.SetDaysInterval(1)?;
        // Add a repetition to the trigger so that it repeats
        // five times.
        let repeat_pattern = daily_trigger.Repetition()?;
        repeat_pattern.SetDuration(&BSTR::from(r"PT4M"))?;
        repeat_pattern.SetInterval(&BSTR::from(r"PT1M"))?;
        //  ------------------------------------------------------
        //  Add an action to the task. This task will execute notepad.exe.

        //  Get the task action collection pointer.
        let action_collection = task.Actions()?;

        //  Create the action, specifying that it is an executable action.
        let action = action_collection.Create(TASK_ACTION_EXEC)?;
        let exec_action = action.cast::<IExecAction>()?;
        //  Set the path of the executable to notepad.exe.
        exec_action.SetPath(&BSTR::from(r"C:\Program Files\LGHUB\lghub.exe"))?;

        //  ------------------------------------------------------
        //  Save the task in the root folder.
        folder.RegisterTaskDefinition(
            &task_name,
            &task,
            6,
            VariantInit(),
            VariantInit(),
            TASK_LOGON_INTERACTIVE_TOKEN,
            VariantInit(),
        )?;

        println!("\n Success! Task successfully registered. ");
    }
    return Ok(());
}
