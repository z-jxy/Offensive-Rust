
use std::{ffi::CString, os::raw::c_void};
use std::mem;

use windows::{
    core::*, 

    Win32::Foundation::*, Win32::System::{Threading::*, Diagnostics::Debug::ReadProcessMemory},
    Win32::UI::WindowsAndMessaging::*, Win32::System::{LibraryLoader::*, Diagnostics::Debug::WriteProcessMemory}, 
    Win32::System::{LibraryLoader::LoadLibraryA, WindowsProgramming::GetComputerNameA},
    Win32::System::SystemInformation::*,
};

fn main()  {
    unsafe {
        let target_dll: PCSTR = windows::core::PCSTR("user32.dll".as_ptr());
        
        let dll_addr = target_dll;

        let handle_func: Result<HINSTANCE> = LoadLibraryA(target_dll);

        let z: u32 = GetCurrentProcessId();
   
        
        println!("targeting dll: {:#?}", target_dll);

        println!("found dll address at: {:#?}", dll_addr);

        println!("Current Process ID: {:#?}", z);


        println!("Handle outcome: {:#?}", handle_func);
    }
}