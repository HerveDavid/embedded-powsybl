use libloading::Library;
use std::path::PathBuf;
use std::sync::Once;

mod graal;
use graal::*;

static INIT: Once = Once::new();

/// A safe wrapper around the embedded Powsybl library
pub struct EmbeddedPowsybl {
    lib: Library,
    isolate: *mut graal_isolate_t,
    thread: *mut graal_isolatethread_t,
    is_thread_detached: bool,  // Pour suivre l'état du thread
}

impl EmbeddedPowsybl {
    /// Creates a new instance of EmbeddedPowsybl
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        INIT.call_once(|| {
            println!("Initializing EmbeddedPowsybl...");
        });

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

    /// Calculates factorial of a given number
    pub fn factorial(&self, n: i64) -> Result<i64, Box<dyn std::error::Error>> {
        if n < 0 {
            return Err("Factorial is not defined for negative numbers".into());
        }

        unsafe {
            let factorial = self.get_factorial_fn()?;
            Ok(factorial(self.thread, n))
        }
    }

    /// Détache explicitement le thread de l'isolate
    /// Cette méthode doit être appelée avant de détruire l'isolate
    pub fn detach(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            if !self.is_thread_detached && !self.thread.is_null() {
                if let Ok(detach_thread) = self.lib.get::<unsafe extern "C" fn(*mut graal_isolatethread_t) -> i32>(b"graal_detach_thread") {
                    let result = detach_thread(self.thread);
                    if result == 0 {
                        self.is_thread_detached = true;
                        self.thread = std::ptr::null_mut();
                    } else {
                        return Err(format!("Failed to detach thread: error code {}", result).into());
                    }
                }
            }
            Ok(())
        }
    }

    /// Détruit proprement l'isolate
    /// Cette méthode ne doit être appelée qu'après avoir détaché le thread
    pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            if !self.isolate.is_null() && self.is_thread_detached {
                if let Ok(tear_down) = self.lib.get::<unsafe extern "C" fn(*mut graal_isolate_t) -> i32>(b"graal_tear_down_isolate") {
                    let result = tear_down(self.isolate);
                    if result == 0 {
                        self.isolate = std::ptr::null_mut();
                    } else {
                        return Err(format!("Failed to tear down isolate: error code {}", result).into());
                    }
                }
            }
            Ok(())
        }
    }

    // Private helper methods
    fn get_library_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("Failed to get parent directory")?
            .join("java")
            .join("target")
            .join("embedded-powsybl.so");

        if !lib_path.exists() {
            return Err(format!("Library not found at {:?}", lib_path).into());
        }

        Ok(lib_path)
    }

    unsafe fn create_isolate(
        lib: &Library
    ) -> Result<(*mut graal_isolate_t, *mut graal_isolatethread_t), Box<dyn std::error::Error>> {
        let params = graal_create_isolate_params_t {
            version: 1,
            reserved_address_space_size: 0,
            auxiliary_image_path: std::ptr::null(),
            auxiliary_image_reserved_space_size: 0,
            reserved_1: 0,
            reserved_2: std::ptr::null_mut(),
            pkey: 0,
            reserved_3: 0,
            reserved_4: 0,
            reserved_5: 0,
        };

        let create_isolate: libloading::Symbol<unsafe extern "C" fn(
            *const graal_create_isolate_params_t,
            *mut *mut graal_isolate_t,
            *mut *mut graal_isolatethread_t,
        ) -> i32> = lib.get(b"graal_create_isolate")?;

        let mut isolate: *mut graal_isolate_t = std::ptr::null_mut();
        let mut thread: *mut graal_isolatethread_t = std::ptr::null_mut();

        let result = create_isolate(&params, &mut isolate, &mut thread);
        
        if result != 0 {
            return Err(format!("Failed to create isolate: error code {}", result).into());
        }

        if isolate.is_null() || thread.is_null() {
            return Err("Isolate or thread pointer is null".into());
        }

        Ok((isolate, thread))
    }

    unsafe fn get_factorial_fn(&self) -> Result<
        libloading::Symbol<unsafe extern "C" fn(*mut graal_isolatethread_t, i64) -> i64>,
        Box<dyn std::error::Error>
    > {
        if self.thread.is_null() {
            return Err("Thread pointer is null".into());
        }

        let factorial_symbol = b"Java_com_rte_france_argus_embedded_powsybl_NativeUtils_factorial\0";
        Ok(self.lib.get(factorial_symbol)?)
    }
}

// impl Drop for EmbeddedPowsybl {
//     fn drop(&mut self) {
//         // On fait un best-effort cleanup en respectant l'ordre
//         let _ = self.detach();  // D'abord détacher le thread
//         let _ = self.destroy(); // Ensuite détruire l'isolate
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        let mut powsybl = EmbeddedPowsybl::init().expect("Failed to initialize");
        let result = powsybl.factorial(5).expect("Failed to calculate factorial");
        assert_eq!(result, 120);
        
        // Cleanup explicite pour le test
        powsybl.detach().expect("Failed to detach thread");
        powsybl.destroy().expect("Failed to destroy isolate");
    }

    #[test]
    fn test_negative_factorial() {
        let mut powsybl = EmbeddedPowsybl::init().expect("Failed to initialize");
        assert!(powsybl.factorial(-1).is_err());
        
        // Cleanup explicite pour le test
        powsybl.detach().expect("Failed to detach thread");
        powsybl.destroy().expect("Failed to destroy isolate");
    }
}