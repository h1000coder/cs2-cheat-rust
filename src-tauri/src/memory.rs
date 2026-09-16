use serde::de::value;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::EnumProcessModules;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next, MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS};
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE};
use std::mem;
use std::ptr;

use crate::math::ViewMatrix;
use crate::offsets::Offsets;

pub struct GameProcess {
    pub pid: u32,
    handle: HANDLE,
    pub modules: Vec<ModuleInfo>,
}

#[derive(Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub base: usize,
    pub size: u32,
}

impl GameProcess {
    pub fn find() -> Option<Self> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot.is_null() {
                return None;
            }
            
            let mut pe: PROCESSENTRY32 = mem::zeroed();
            pe.dwSize = mem::size_of::<PROCESSENTRY32>() as DWORD;
            
            if Process32First(snapshot, &mut pe) == FALSE {
                CloseHandle(snapshot);
                return None;
            }
            
            loop {
                // Converter de [i8; 260] para String
                let name_bytes = &pe.szExeFile;
                let mut name_vec = Vec::new();
                for &c in name_bytes.iter() {
                    if c == 0 {
                        break;
                    }
                    name_vec.push(c as u8);
                }
                let name = String::from_utf8_lossy(&name_vec).to_lowercase();
                
                if name.contains("cs2") {
                    let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pe.th32ProcessID);
                    CloseHandle(snapshot);
                    
                    if handle.is_null() {
                        return None;
                    }
                    
                    let mut game = GameProcess {
                        pid: pe.th32ProcessID,
                        handle,
                        modules: Vec::new(),
                    };
                    game.load_modules();
                    return Some(game);
                }
                
                if Process32Next(snapshot, &mut pe) == FALSE {
                    break;
                }
            }
            
            CloseHandle(snapshot);
            None
        }
    }
    
    fn load_modules(&mut self) {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, self.pid);
            if snapshot.is_null() {
                return;
            }
            
            let mut me: MODULEENTRY32 = mem::zeroed();
            me.dwSize = mem::size_of::<MODULEENTRY32>() as DWORD;
            
            if Module32First(snapshot, &mut me) != FALSE {
                loop {
                    // Converter de [i8; 256] para String
                    let name_bytes = &me.szModule;
                    let mut name_vec = Vec::new();
                    for &c in name_bytes.iter() {
                        if c == 0 {
                            break;
                        }
                        name_vec.push(c as u8);
                    }
                    let name = String::from_utf8_lossy(&name_vec).to_string();
                    
                    self.modules.push(ModuleInfo {
                        name,
                        base: me.modBaseAddr as usize,
                        size: me.modBaseSize,
                    });
                    
                    if Module32Next(snapshot, &mut me) == FALSE {
                        break;
                    }
                }
            }
            
            CloseHandle(snapshot);
        }
    }
    
    pub fn get_module_base(&self, name: &str) -> Option<usize> {
        self.modules
            .iter()
            .find(|m| m.name.to_lowercase() == name.to_lowercase())
            .map(|m| m.base)
    }
    
    pub fn is_running(&self) -> bool {
        unsafe {
            let mut exit_code = 0u32;
            let result = winapi::um::processthreadsapi::GetExitCodeProcess(self.handle, &mut exit_code);
            result != FALSE && exit_code == 259 // STILL_ACTIVE
        }
    }
    
    pub fn read_ptr(&self, address: usize) -> Option<usize> {
        self.read::<usize>(address)
    }
    
    pub fn read_i32(&self, address: usize) -> Option<i32> {
        self.read::<i32>(address)
    }
    
    pub fn read_float(&self, address: usize) -> Option<f32> {
        self.read::<f32>(address)
    }
    
    pub fn read_bool(&self, address: usize) -> Option<bool> {
        self.read::<bool>(address)
    }
    
    pub fn read_vec3(&self, address: usize) -> Option<glam::Vec3> {
        let x = self.read_float(address)?;
        let y = self.read_float(address + 4)?;
        let z = self.read_float(address + 8)?;
        Some(glam::Vec3::new(x, y, z))
    }
    
    pub fn read_view_matrix(&self, client_base: usize) -> Option<ViewMatrix> {
            let view_matrix_addr = client_base + Offsets::dwViewMatrix;
            let mut matrix = [[0.0f32; 4]; 4];
            
            for i in 0..4 {
                for j in 0..4 {
                    let offset = (i * 4 + j) * 4;
                    if let Some(val) = self.read_float(view_matrix_addr + offset) {
                        matrix[i][j] = val;
                    } else {
                        return None;
                    }
                }
            }
            
            Some(ViewMatrix::new(matrix))
        }
    
    pub fn read_string(&self, address: usize, max_len: usize) -> Option<String> {
        unsafe {
            let mut buffer = vec![0u8; max_len];
            let mut bytes_read = 0usize;
            
            let success = ReadProcessMemory(
                self.handle,
                address as _,
                buffer.as_mut_ptr() as _,
                max_len,
                &mut bytes_read,
            );
            
            if success == FALSE {
                return None;
            }
            
            let mut string_bytes = Vec::new();
            for &b in buffer.iter() {
                if b == 0 {
                    break;
                }
                string_bytes.push(b);
            }
            
            String::from_utf8(string_bytes).ok()
        }
    }
    
    pub fn read<T: Copy>(&self, address: usize) -> Option<T> {
        unsafe {
            let mut value: T = mem::zeroed();
            let mut bytes_read = 0usize;
            
            let success = ReadProcessMemory(
                self.handle,
                address as _,
                (&mut value as *mut T) as _,
                mem::size_of::<T>(),
                &mut bytes_read,
            );
            
            if success == FALSE {
                return None;
            }
            
            Some(value)
        }
    }

    pub fn write<T: Copy>(&self, address: usize, value: T) -> bool {
        unsafe {
            let mut bytes_written = 0usize;
            let sucess = WriteProcessMemory(
                self.handle, 
                address as _, 
                &value as *const _ as _, 
                std::mem::size_of::<T>(), 
            &mut bytes_written);
            sucess != FALSE
        }
    }

    pub fn write_bytes(&self, address: usize, data: &[u8]) -> bool {
        unsafe {
            let mut bytes_written = 0usize;
            let sucess = WriteProcessMemory(
                self.handle, 
                address as _, 
                data.as_ptr() as _, 
                data.len(), 
            &mut bytes_written);
            sucess != FALSE
        }
    }
}

impl Drop for GameProcess {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}