// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Cancellable;
use DataStreamByteOrder;
use FilterOutputStream;
use OutputStream;
use Seekable;

glib_wrapper! {
    pub struct DataOutputStream(Object<gio_sys::GDataOutputStream, gio_sys::GDataOutputStreamClass, DataOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements Seekable;

    match fn {
        get_type => || gio_sys::g_data_output_stream_get_type(),
    }
}

impl DataOutputStream {
    pub fn new<P: IsA<OutputStream>>(base_stream: &P) -> DataOutputStream {
        unsafe {
            from_glib_full(gio_sys::g_data_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct DataOutputStreamBuilder {
    byte_order: Option<DataStreamByteOrder>,
    base_stream: Option<OutputStream>,
    close_base_stream: Option<bool>,
}

impl DataOutputStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DataOutputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref byte_order) = self.byte_order {
            properties.push(("byte-order", byte_order));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        glib::Object::new(DataOutputStream::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn byte_order(mut self, byte_order: DataStreamByteOrder) -> Self {
        self.byte_order = Some(byte_order);
        self
    }

    pub fn base_stream<P: IsA<OutputStream>>(mut self, base_stream: &P) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_DATA_OUTPUT_STREAM: Option<&DataOutputStream> = None;

pub trait DataOutputStreamExt: 'static {
    fn get_byte_order(&self) -> DataStreamByteOrder;

    fn put_byte<P: IsA<Cancellable>>(
        &self,
        data: u8,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_int16<P: IsA<Cancellable>>(
        &self,
        data: i16,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_int32<P: IsA<Cancellable>>(
        &self,
        data: i32,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_int64<P: IsA<Cancellable>>(
        &self,
        data: i64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_string<P: IsA<Cancellable>>(
        &self,
        str: &str,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_uint16<P: IsA<Cancellable>>(
        &self,
        data: u16,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_uint32<P: IsA<Cancellable>>(
        &self,
        data: u32,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn put_uint64<P: IsA<Cancellable>>(
        &self,
        data: u64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error>;

    fn set_byte_order(&self, order: DataStreamByteOrder);

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DataOutputStream>> DataOutputStreamExt for O {
    fn get_byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(gio_sys::g_data_output_stream_get_byte_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn put_byte<P: IsA<Cancellable>>(
        &self,
        data: u8,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_byte(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_int16<P: IsA<Cancellable>>(
        &self,
        data: i16,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_int16(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_int32<P: IsA<Cancellable>>(
        &self,
        data: i32,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_int32(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_int64<P: IsA<Cancellable>>(
        &self,
        data: i64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_int64(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_string<P: IsA<Cancellable>>(
        &self,
        str: &str,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_string(
                self.as_ref().to_glib_none().0,
                str.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_uint16<P: IsA<Cancellable>>(
        &self,
        data: u16,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_uint16(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_uint32<P: IsA<Cancellable>>(
        &self,
        data: u32,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_uint32(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn put_uint64<P: IsA<Cancellable>>(
        &self,
        data: u64,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_data_output_stream_put_uint64(
                self.as_ref().to_glib_none().0,
                data,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            gio_sys::g_data_output_stream_set_byte_order(
                self.as_ref().to_glib_none().0,
                order.to_glib(),
            );
        }
    }

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_byte_order_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GDataOutputStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DataOutputStream>,
        {
            let f: &F = &*(f as *const F);
            f(&DataOutputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::byte-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_byte_order_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DataOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataOutputStream")
    }
}
