// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Drive;
use Error;
use File;
use Icon;
use Mount;
use MountMountFlags;
use MountOperation;
use MountUnmountFlags;
#[cfg(feature = "futures")]
use futures::future;
use gio_sys;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Volume(Interface<gio_sys::GVolume>);

    match fn {
        get_type => || gio_sys::g_volume_get_type(),
    }
}

pub const NONE_VOLUME: Option<&Volume> = None;

pub trait VolumeExt: 'static {
    fn can_eject(&self) -> bool;

    fn can_mount(&self) -> bool;

    fn eject_with_operation<P: IsA<MountOperation>, Q: IsA<Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: Option<&P>, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(&self, flags: MountUnmountFlags, mount_operation: Option<&P>) -> Box_<future::Future<Output = Result<(), Error>> + std::marker::Unpin>;

    fn enumerate_identifiers(&self) -> Vec<GString>;

    fn get_activation_root(&self) -> Option<File>;

    fn get_drive(&self) -> Option<Drive>;

    fn get_icon(&self) -> Option<Icon>;

    fn get_identifier(&self, kind: &str) -> Option<GString>;

    fn get_mount(&self) -> Option<Mount>;

    fn get_name(&self) -> Option<GString>;

    fn get_sort_key(&self) -> Option<GString>;

    fn get_symbolic_icon(&self) -> Option<Icon>;

    fn get_uuid(&self) -> Option<GString>;

    fn mount<P: IsA<MountOperation>, Q: IsA<Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountMountFlags, mount_operation: Option<&P>, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn mount_future<P: IsA<MountOperation> + Clone + 'static>(&self, flags: MountMountFlags, mount_operation: Option<&P>) -> Box_<future::Future<Output = Result<(), Error>> + std::marker::Unpin>;

    fn should_automount(&self) -> bool;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Volume>> VolumeExt for O {
    fn can_eject(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_volume_can_eject(self.as_ref().to_glib_none().0))
        }
    }

    fn can_mount(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_volume_can_mount(self.as_ref().to_glib_none().0))
        }
    }

    fn eject_with_operation<P: IsA<MountOperation>, Q: IsA<Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: Option<&P>, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn eject_with_operation_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_volume_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<R>;
        unsafe {
            gio_sys::g_volume_eject_with_operation(self.as_ref().to_glib_none().0, flags.to_glib(), mount_operation.map(|p| p.as_ref()).to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(&self, flags: MountUnmountFlags, mount_operation: Option<&P>) -> Box_<future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use GioFuture;
        use fragile::Fragile;

        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.eject_with_operation(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn enumerate_identifiers(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_volume_enumerate_identifiers(self.as_ref().to_glib_none().0))
        }
    }

    fn get_activation_root(&self) -> Option<File> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_activation_root(self.as_ref().to_glib_none().0))
        }
    }

    fn get_drive(&self) -> Option<Drive> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_drive(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_identifier(&self, kind: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_identifier(self.as_ref().to_glib_none().0, kind.to_glib_none().0))
        }
    }

    fn get_mount(&self) -> Option<Mount> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_mount(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_sort_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_volume_get_sort_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_symbolic_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_symbolic_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uuid(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_volume_get_uuid(self.as_ref().to_glib_none().0))
        }
    }

    fn mount<P: IsA<MountOperation>, Q: IsA<Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountMountFlags, mount_operation: Option<&P>, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn mount_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_volume_mount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = mount_trampoline::<R>;
        unsafe {
            gio_sys::g_volume_mount(self.as_ref().to_glib_none().0, flags.to_glib(), mount_operation.map(|p| p.as_ref()).to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn mount_future<P: IsA<MountOperation> + Clone + 'static>(&self, flags: MountMountFlags, mount_operation: Option<&P>) -> Box_<future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        use GioFuture;
        use fragile::Fragile;

        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.mount(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn should_automount(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_volume_should_automount(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"removed\0".as_ptr() as *const _,
                Some(transmute(removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GVolume, f: glib_sys::gpointer)
where P: IsA<Volume> {
    let f: &F = &*(f as *const F);
    f(&Volume::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn removed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GVolume, f: glib_sys::gpointer)
where P: IsA<Volume> {
    let f: &F = &*(f as *const F);
    f(&Volume::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Volume")
    }
}
