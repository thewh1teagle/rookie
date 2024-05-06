use windows::{
  core::{HSTRING, PCWSTR, PWSTR},
  Win32::{
    Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS, WIN32_ERROR},
    System::RestartManager::{
      RmEndSession, RmForceShutdown, RmGetList, RmRegisterResources, RmShutdown, RmStartSession,
      CCH_RM_SESSION_KEY, RM_PROCESS_INFO,
    },
  },
};

/// https://learn.microsoft.com/en-us/windows/win32/rstmgr/restart-manager-portal
/// Release file locking by seamlessly restart the process which lock the file
/// Most of the times the process will keep running smoothly after restart
/// It might take some time up to a minute
pub unsafe fn release_file_lock(file_path: &str) -> bool {
  let file_path = HSTRING::from(file_path);
  let mut session: u32 = 0;
  let mut session_key_buffer = [0_u16; (CCH_RM_SESSION_KEY as usize) + 1];
  let session_key = PWSTR(session_key_buffer.as_mut_ptr());
  let result = RmStartSession(&mut session, 0, session_key);
  if WIN32_ERROR(result) == ERROR_SUCCESS {
    let result = RmRegisterResources(session, Some(&[PCWSTR(file_path.as_ptr())]), None, None);
    if WIN32_ERROR(result) == ERROR_SUCCESS {
      let mut pnprocinfoneeded: u32 = 0;
      let mut rgaffectedapps: [RM_PROCESS_INFO; 1] = [RM_PROCESS_INFO {
        ..Default::default()
      }];
      let mut lpdwrebootreasons: u32 = 0;
      let mut pnprocinfo: u32 = 0;
      let result = RmGetList(
        session,
        &mut pnprocinfoneeded,
        &mut pnprocinfo,
        Some(rgaffectedapps.as_mut_ptr()),
        &mut lpdwrebootreasons,
      );
      if WIN32_ERROR(result) == ERROR_SUCCESS || WIN32_ERROR(result) == ERROR_MORE_DATA {
        if pnprocinfoneeded > 0 {
          // If current process does not have enough privileges to close one of
          // the "offending" processes, you'll get ERROR_FAIL_NOACTION_REBOOT
          let result = RmShutdown(session, RmForceShutdown.0 as u32, None);
          if WIN32_ERROR(result) == ERROR_SUCCESS {
            // success
            RmEndSession(session);
            return true;
          }
        } else {
          // success
          RmEndSession(session);
          return true;
        }
      }
    }
    RmEndSession(session);
    return false;
  }
  false
}
