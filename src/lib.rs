pub use ctor::ctor;
use std::sync::Mutex;
pub struct StateFull<T>(Mutex<Option<T>>);

impl<T> StateFull<T> {
    pub const fn new() -> Self {
        Self(Mutex::new(None))
    }
    pub fn set_value<F: Fn(&mut Option<T>)>(&self, f: F) {
        let mut data = self.0.lock().unwrap();
        f(&mut data);
    }

    pub fn take(&self) -> Option<T> {
        self.0.lock().unwrap().take()
    }
}

#[macro_export]
macro_rules! set_state {
    ($id:ident,$fun:expr) => {
        const _: () = {
            #[$crate::ctor]
            static STATIC_DYNAMIC_CTOR: () = {
                $id::get().set_value($fun);
            };
        };
    };
}

#[macro_export]
macro_rules! define_state {
    ($id:ident:$typ:ty = $init:expr) => {
        #[allow(non_camel_case_types)]
        struct $id;
        impl $id {
            pub fn get() -> &'static $crate::StateFull<$typ> {
                static SINGLETON: $crate::StateFull<$typ> = $crate::StateFull::new();
                SINGLETON.set_value(|v| {
                    if v.is_none() {
                        *v = Some($init);
                    }
                });
                &SINGLETON
            }
        }
    };
}
