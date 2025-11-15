macro_rules! configure_app {
    ($app_name: expr, $app_name_size: expr, $icon_path: expr) => {
        #[allow(unused_imports)]
        #[cfg(target_os = "none")]
        use cortex_m;

        #[cfg(target_os = "none")]
        use eadk::adresses::heap_size;
        #[cfg(target_os = "none")]
        use embedded_alloc::LlffHeap as Heap;
        use crate::eadk::adresses::HEAP_START;

        #[global_allocator]
        #[cfg(target_os = "none")]
        static HEAP: Heap = Heap::empty();

        #[cfg(target_os = "none")]
        extern crate alloc;

        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_name")]
        pub static EADK_APP_NAME: [u8; $app_name_size] = *$app_name;

        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_api_level")]
        pub static EADK_APP_API_LEVEL: u32 = 0;

        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_icon")]
        pub static EADK_APP_ICON: [u8; 3437] = *include_bytes!($icon_path);

    };
}

