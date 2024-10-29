use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::Path};

use eyre::{bail, Result};
use windows::Win32::System::Threading::OpenProcessToken;
use windows::Win32::{
  Foundation::{CloseHandle, BOOL, HANDLE, NTSTATUS},
  Security::{DuplicateToken, ImpersonateLoggedOnUser, RevertToSelf, TOKEN_DUPLICATE, TOKEN_QUERY},
  System::{
    ProcessStatus::K32GetProcessImageFileNameW,
    Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
  },
};

#[link(name = "ntdll")]
extern "system" {
  fn RtlAdjustPrivilege(
    privilege: i32,
    enable: BOOL,
    current_thread: BOOL,
    previous_value: *mut BOOL,
  ) -> NTSTATUS;
}

fn enable_privilege() -> Result<()> {
  use windows::Wdk::System::SystemServices::SE_DEBUG_PRIVILEGE;
  let mut previous_value = BOOL(0);
  let status =
    unsafe { RtlAdjustPrivilege(SE_DEBUG_PRIVILEGE, BOOL(1), BOOL(0), &mut previous_value) };
  if status.0 != 0 {
    bail!("Invalid status from RtlAdjustPrivilege")
  }
  Ok(())
}

fn get_process_pids() -> Result<Vec<u32>> {
  use windows::Win32::System::ProcessStatus::EnumProcesses;
  let mut cb_needed: u32 = 0;
  let mut a_processes: Vec<u32> = Vec::with_capacity(1024);

  unsafe {
    EnumProcesses(a_processes.as_mut_ptr(), 1024 * 4, &mut cb_needed)?;
    a_processes.set_len((cb_needed / 4) as usize);
  };
  let mut _c_processes: u32 = 0;
  _c_processes = cb_needed / 4;

  let mut processes = Vec::new();
  let mut count: u32 = 0;
  while count < _c_processes {
    let pid = a_processes[count as usize];
    processes.push(pid);
    count += 1;
  }

  Ok(processes)
}

fn get_process_name(pid: u32) -> Result<String> {
  unsafe {
    // Open the process with permissions to query information and read VM
    let process_handle: HANDLE =
      OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)?;
    if process_handle.is_invalid() {
      return Err(windows::core::Error::from_win32().into());
    }
    let mut buffer = vec![0u16; 260]; // 260 is the max path length in Windows

    // Get the process image file name
    let length = K32GetProcessImageFileNameW(process_handle, &mut buffer) as usize;
    CloseHandle(process_handle)?;

    // Convert the buffer to a Rust String and trim the null terminator
    let full_path = OsString::from_wide(&buffer[..length])
      .to_string_lossy()
      .into_owned();
    let executable_name = Path::new(&full_path)
      .file_name()
      .and_then(|name| name.to_str())
      .unwrap_or("")
      .to_string();
    Ok(executable_name)
  }
}

fn get_lsass_pid() -> Result<u32> {
  for pid in get_process_pids()? {
    if get_process_name(pid).unwrap_or_default() == "lsass.exe" {
      return Ok(pid);
    }
  }
  bail!("lsass.exe not found!")
}

fn get_process_handle(pid: u32) -> Result<HANDLE> {
  unsafe {
    // Open the process with PROCESS_QUERY_INFORMATION permission
    let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid)?;

    // Check if the handle is valid
    if process_handle.is_invalid() {
      Err(windows::core::Error::from_win32().into())
    } else {
      Ok(process_handle)
    }
  }
}

fn close_handle(handle: HANDLE) -> Result<()> {
  unsafe { CloseHandle(handle)? };
  Ok(())
}

fn get_system_token(lsass_handle: HANDLE) -> Result<HANDLE> {
  let mut token_handle = HANDLE::default();
  unsafe {
    OpenProcessToken(
      lsass_handle,
      TOKEN_DUPLICATE | TOKEN_QUERY,
      &mut token_handle,
    )?;
  }

  let mut duplicate_token = HANDLE::default();
  unsafe {
    DuplicateToken(
      token_handle,
      windows::Win32::Security::SECURITY_IMPERSONATION_LEVEL(2), // win32security.SecurityImpersonation
      &mut duplicate_token,
    )?;
    CloseHandle(token_handle)?;
  }

  Ok(duplicate_token)
}

pub fn start_impersonate() -> Result<HANDLE> {
  enable_privilege()?;
  let pid = get_lsass_pid()?;
  let lsass_handle = get_process_handle(pid)?;
  let duplicated_token = get_system_token(lsass_handle)?;
  close_handle(lsass_handle)?;
  unsafe {
    ImpersonateLoggedOnUser(duplicated_token)?;
  }
  Ok(duplicated_token)
}

pub fn stop_impersonate(duplicated_token: HANDLE) -> Result<()> {
  unsafe {
    CloseHandle(duplicated_token)?;
    RevertToSelf()?;
  }
  Ok(())
}
