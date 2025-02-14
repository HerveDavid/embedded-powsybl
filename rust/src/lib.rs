use libloading::Library;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::path::PathBuf;

// Importation des bindings générés
mod bindings;
use bindings::*;

pub struct EmbeddedPowsybl {
    lib: Library,
    isolate: *mut graal_isolate_t,
    thread: *mut graal_isolatethread_t,
    is_thread_detached: bool,
}

impl EmbeddedPowsybl {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let lib_path = Self::get_library_path()?;
        unsafe {
            let lib = Library::new(&lib_path)?;
            let (isolate, thread) = Self::create_isolate(&lib)?;

            Ok(Self {
                lib,
                isolate,
                thread,
                is_thread_detached: false,
            })
        }
    }

    fn get_library_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Embarquer la bibliothèque directement dans l'exécutable
        static LIB_BYTES: &[u8] = include_bytes!("../../java/target/embedded-powsybl.so");

        // Créer un répertoire temporaire pour extraire la bibliothèque
        let temp_dir = std::env::temp_dir().join("embedded-powsybl");
        fs::create_dir_all(&temp_dir)?;

        let lib_path = temp_dir.join("embedded-powsybl.so");

        // Écrire la bibliothèque seulement si elle n'existe pas déjà
        if !lib_path.exists() {
            fs::write(&lib_path, LIB_BYTES)?;
        }

        Ok(lib_path)
    }

    unsafe fn create_isolate(
        lib: &Library,
    ) -> Result<(*mut graal_isolate_t, *mut graal_isolatethread_t), Box<dyn std::error::Error>>
    {
        let create_isolate = lib.get::<unsafe extern "C" fn(
            *const graal_create_isolate_params_t,
            *mut *mut graal_isolate_t,
            *mut *mut graal_isolatethread_t,
        ) -> i32>(b"graal_create_isolate")?;

        let params = graal_create_isolate_params_t {
            version: 1,
            ..Default::default() // Utilisation du trait Default généré par bindgen
        };

        let mut isolate: *mut graal_isolate_t = std::ptr::null_mut();
        let mut thread: *mut graal_isolatethread_t = std::ptr::null_mut();

        let result = create_isolate(&params, &mut isolate, &mut thread);
        if result != 0 {
            return Err(format!("Failed to create isolate: error code {}", result).into());
        }

        Ok((isolate, thread))
    }

    pub fn detach(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            if !self.is_thread_detached && !self.thread.is_null() {
                let detach_thread =
                    self.lib
                        .get::<unsafe extern "C" fn(*mut graal_isolatethread_t) -> i32>(
                            b"graal_detach_thread",
                        )?;

                let result = detach_thread(self.thread);
                if result == 0 {
                    self.is_thread_detached = true;
                    self.thread = std::ptr::null_mut();
                } else {
                    return Err(format!("Failed to detach thread: error code {}", result).into());
                }
            }
            Ok(())
        }
    }

    pub fn read_xiidm_file(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        unsafe {
            let read_xiidm = self
                .lib
                .get::<unsafe extern "C" fn(*mut graal_isolatethread_t, *const i8) -> *const i8>(
                    b"readXiidmFile",
                )?;

            let c_file_path = CString::new(file_path)?;
            let result = read_xiidm(self.thread, c_file_path.as_ptr());

            if result.is_null() {
                return Err("Failed to read XIIDM file".into());
            }

            // Convertir le résultat en String
            let content = CStr::from_ptr(result).to_string_lossy().into_owned();

            if content.starts_with("Error:") {
                return Err(content.into());
            }

            Ok(content)
        }
    }
}

impl Drop for EmbeddedPowsybl {
    fn drop(&mut self) {
        if let Ok(lib_path) = Self::get_library_path() {
            let _ = std::fs::remove_file(lib_path);
        }
    }
}
