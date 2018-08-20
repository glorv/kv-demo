// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct KVPair {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KVPair {}

impl KVPair {
    pub fn new() -> KVPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KVPair {
        static mut instance: ::protobuf::lazy::Lazy<KVPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KVPair,
        };
        unsafe {
            instance.get(KVPair::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for KVPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KVPair {
    fn new() -> KVPair {
        KVPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<KVPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KVPair::get_key_for_reflect,
                    KVPair::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    KVPair::get_value_for_reflect,
                    KVPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KVPair>(
                    "KVPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KVPair {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KVPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KVPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeyRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyRequest {}

impl KeyRequest {
    pub fn new() -> KeyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyRequest {
        static mut instance: ::protobuf::lazy::Lazy<KeyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyRequest,
        };
        unsafe {
            instance.get(KeyRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for KeyRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyRequest {
    fn new() -> KeyRequest {
        KeyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KeyRequest::get_key_for_reflect,
                    KeyRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyRequest>(
                    "KeyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetResponse {
    // message fields
    pub status: Status,
    pub error: ::std::string::String,
    pub exist: bool,
    pub data: ::protobuf::SingularPtrField<KVPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetResponse {}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetResponse,
        };
        unsafe {
            instance.get(GetResponse::new)
        }
    }

    // .kv.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = Status::OK;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn get_status_for_reflect(&self) -> &Status {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut Status {
        &mut self.status
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // bool exist = 3;

    pub fn clear_exist(&mut self) {
        self.exist = false;
    }

    // Param is passed by value, moved
    pub fn set_exist(&mut self, v: bool) {
        self.exist = v;
    }

    pub fn get_exist(&self) -> bool {
        self.exist
    }

    fn get_exist_for_reflect(&self) -> &bool {
        &self.exist
    }

    fn mut_exist_for_reflect(&mut self) -> &mut bool {
        &mut self.exist
    }

    // .kv.KVPair data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: KVPair) {
        self.data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut KVPair {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> KVPair {
        self.data.take().unwrap_or_else(|| KVPair::new())
    }

    pub fn get_data(&self) -> &KVPair {
        self.data.as_ref().unwrap_or_else(|| KVPair::default_instance())
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularPtrField<KVPair> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KVPair> {
        &mut self.data
    }
}

impl ::protobuf::Message for GetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.exist = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != Status::OK {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        if self.exist != false {
            my_size += 2;
        }
        if let Some(ref v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != Status::OK {
            os.write_enum(1, self.status.value())?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
        }
        if self.exist != false {
            os.write_bool(3, self.exist)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetResponse {
    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    GetResponse::get_status_for_reflect,
                    GetResponse::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    GetResponse::get_error_for_reflect,
                    GetResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "exist",
                    GetResponse::get_exist_for_reflect,
                    GetResponse::mut_exist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KVPair>>(
                    "data",
                    GetResponse::get_data_for_reflect,
                    GetResponse::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetResponse>(
                    "GetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.clear_exist();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetResponse {
    // message fields
    pub status: Status,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetResponse {}

impl SetResponse {
    pub fn new() -> SetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetResponse,
        };
        unsafe {
            instance.get(SetResponse::new)
        }
    }

    // .kv.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = Status::OK;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn get_status_for_reflect(&self) -> &Status {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut Status {
        &mut self.status
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for SetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != Status::OK {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != Status::OK {
            os.write_enum(1, self.status.value())?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetResponse {
    fn new() -> SetResponse {
        SetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    SetResponse::get_status_for_reflect,
                    SetResponse::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    SetResponse::get_error_for_reflect,
                    SetResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetResponse>(
                    "SetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanRequset {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub can_equal: bool,
    pub count: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanRequset {}

impl ScanRequset {
    pub fn new() -> ScanRequset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanRequset {
        static mut instance: ::protobuf::lazy::Lazy<ScanRequset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanRequset,
        };
        unsafe {
            instance.get(ScanRequset::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bool can_equal = 2;

    pub fn clear_can_equal(&mut self) {
        self.can_equal = false;
    }

    // Param is passed by value, moved
    pub fn set_can_equal(&mut self, v: bool) {
        self.can_equal = v;
    }

    pub fn get_can_equal(&self) -> bool {
        self.can_equal
    }

    fn get_can_equal_for_reflect(&self) -> &bool {
        &self.can_equal
    }

    fn mut_can_equal_for_reflect(&mut self) -> &mut bool {
        &mut self.can_equal
    }

    // int32 count = 3;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i32) {
        self.count = v;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &i32 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.count
    }
}

impl ::protobuf::Message for ScanRequset {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.can_equal = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.count = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if self.can_equal != false {
            my_size += 2;
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if self.can_equal != false {
            os.write_bool(2, self.can_equal)?;
        }
        if self.count != 0 {
            os.write_int32(3, self.count)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScanRequset {
    fn new() -> ScanRequset {
        ScanRequset::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanRequset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    ScanRequset::get_key_for_reflect,
                    ScanRequset::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "can_equal",
                    ScanRequset::get_can_equal_for_reflect,
                    ScanRequset::mut_can_equal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "count",
                    ScanRequset::get_count_for_reflect,
                    ScanRequset::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanRequset>(
                    "ScanRequset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanRequset {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_can_equal();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanRequset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanRequset {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanResponse {
    // message fields
    pub status: Status,
    pub error: ::std::string::String,
    pub data: ::protobuf::RepeatedField<KVPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanResponse {}

impl ScanResponse {
    pub fn new() -> ScanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanResponse {
        static mut instance: ::protobuf::lazy::Lazy<ScanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanResponse,
        };
        unsafe {
            instance.get(ScanResponse::new)
        }
    }

    // .kv.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = Status::OK;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn get_status_for_reflect(&self) -> &Status {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut Status {
        &mut self.status
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // repeated .kv.KVPair data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::protobuf::RepeatedField<KVPair>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::protobuf::RepeatedField<KVPair> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::protobuf::RepeatedField<KVPair> {
        ::std::mem::replace(&mut self.data, ::protobuf::RepeatedField::new())
    }

    pub fn get_data(&self) -> &[KVPair] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::protobuf::RepeatedField<KVPair> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KVPair> {
        &mut self.data
    }
}

impl ::protobuf::Message for ScanResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != Status::OK {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != Status::OK {
            os.write_enum(1, self.status.value())?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
        }
        for v in &self.data {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScanResponse {
    fn new() -> ScanResponse {
        ScanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ScanResponse::get_status_for_reflect,
                    ScanResponse::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ScanResponse::get_error_for_reflect,
                    ScanResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KVPair>>(
                    "data",
                    ScanResponse::get_data_for_reflect,
                    ScanResponse::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanResponse>(
                    "ScanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    OK = 0,
    FAILED = 1,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::OK),
            1 => ::std::option::Option::Some(Status::FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::OK,
            Status::FAILED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

impl ::std::default::Default for Status {
    fn default() -> Self {
        Status::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08kv.proto\x12\x02kv\"0\n\x06KVPair\x12\x10\n\x03key\x18\x01\x20\x01\
    (\x0cR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\"\x1e\n\
    \nKeyRequest\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\"}\n\x0bGetRe\
    sponse\x12\"\n\x06status\x18\x01\x20\x01(\x0e2\n.kv.StatusR\x06status\
    \x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05error\x12\x14\n\x05exist\x18\
    \x03\x20\x01(\x08R\x05exist\x12\x1e\n\x04data\x18\x04\x20\x01(\x0b2\n.kv\
    .KVPairR\x04data\"G\n\x0bSetResponse\x12\"\n\x06status\x18\x01\x20\x01(\
    \x0e2\n.kv.StatusR\x06status\x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05e\
    rror\"R\n\x0bScanRequset\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\
    \x12\x1b\n\tcan_equal\x18\x02\x20\x01(\x08R\x08canEqual\x12\x14\n\x05cou\
    nt\x18\x03\x20\x01(\x05R\x05count\"h\n\x0cScanResponse\x12\"\n\x06status\
    \x18\x01\x20\x01(\x0e2\n.kv.StatusR\x06status\x12\x14\n\x05error\x18\x02\
    \x20\x01(\tR\x05error\x12\x1e\n\x04data\x18\x03\x20\x03(\x0b2\n.kv.KVPai\
    rR\x04data*\x1c\n\x06Status\x12\x06\n\x02OK\x10\0\x12\n\n\x06FAILED\x10\
    \x012\xb5\x01\n\tKVService\x12(\n\x03Get\x12\x0e.kv.KeyRequest\x1a\x0f.k\
    v.GetResponse\"\0\x12$\n\x03Put\x12\n.kv.KVPair\x1a\x0f.kv.SetResponse\"\
    \0\x12+\n\x06Remove\x12\x0e.kv.KeyRequest\x1a\x0f.kv.SetResponse\"\0\x12\
    +\n\x04Scan\x12\x0f.kv.ScanRequset\x1a\x10.kv.ScanResponse\"\0J\xa3\x0c\
    \n\x06\x12\x04\0\0/\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\n\n\n\n\x02\x05\0\x12\x04\x04\0\x07\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x04\x05\x0b\n\x0b\n\x04\x05\0\x02\0\x12\x03\x05\x02\t\n\x0c\
    \n\x05\x05\0\x02\0\x01\x12\x03\x05\x02\x04\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x03\x05\x07\x08\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x06\x02\r\n\x0c\n\
    \x05\x05\0\x02\x01\x01\x12\x03\x06\x02\x08\n\x0c\n\x05\x05\0\x02\x01\x02\
    \x12\x03\x06\x0b\x0c\n\n\n\x02\x04\0\x12\x04\t\0\x0c\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\t\x08\x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x02\x10\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\n\x02\t\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\n\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x08\x0b\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\n\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0b\
    \x02\x12\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0b\x02\n\x10\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x0b\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x10\x11\n\n\n\
    \x02\x04\x01\x12\x04\x0e\0\x10\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\
    \x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0f\x02\x10\n\r\n\x05\x04\x01\x02\
    \0\x04\x12\x04\x0f\x02\x0e\x14\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0f\
    \x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0f\x08\x0b\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x0f\x0e\x0f\n\n\n\x02\x04\x02\x12\x04\x12\0\
    \x17\x01\n\n\n\x03\x04\x02\x01\x12\x03\x12\x08\x13\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x13\x02\x14\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x13\x02\
    \x12\x15\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x13\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x13\t\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\x13\x12\x13\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x14\x02\x13\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04\x14\x02\x13\x14\n\x0c\n\x05\x04\x02\x02\
    \x01\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x14\
    \t\x0e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x14\x11\x12\n\x0b\n\x04\
    \x04\x02\x02\x02\x12\x03\x15\x02\x11\n\r\n\x05\x04\x02\x02\x02\x04\x12\
    \x04\x15\x02\x14\x13\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x15\x02\x06\
    \n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x15\x07\x0c\n\x0c\n\x05\x04\x02\
    \x02\x02\x03\x12\x03\x15\x0f\x10\n\x0b\n\x04\x04\x02\x02\x03\x12\x03\x16\
    \x02\x12\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04\x16\x02\x15\x11\n\x0c\n\
    \x05\x04\x02\x02\x03\x06\x12\x03\x16\x02\x08\n\x0c\n\x05\x04\x02\x02\x03\
    \x01\x12\x03\x16\t\r\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\x16\x10\x11\
    \n\n\n\x02\x04\x03\x12\x04\x19\0\x1c\x01\n\n\n\x03\x04\x03\x01\x12\x03\
    \x19\x08\x13\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1a\x02\x14\n\r\n\x05\x04\
    \x03\x02\0\x04\x12\x04\x1a\x02\x19\x15\n\x0c\n\x05\x04\x03\x02\0\x06\x12\
    \x03\x1a\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1a\t\x0f\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03\x1a\x12\x13\n\x0b\n\x04\x04\x03\x02\x01\
    \x12\x03\x1b\x02\x13\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x1b\x02\x1a\
    \x14\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\x04\
    \x03\x02\x01\x01\x12\x03\x1b\t\x0e\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\
    \x03\x1b\x11\x12\n\n\n\x02\x04\x04\x12\x04\x1e\0\"\x01\n\n\n\x03\x04\x04\
    \x01\x12\x03\x1e\x08\x13\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x1f\x02\x10\n\
    \r\n\x05\x04\x04\x02\0\x04\x12\x04\x1f\x02\x1e\x15\n\x0c\n\x05\x04\x04\
    \x02\0\x05\x12\x03\x1f\x02\x07\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x1f\
    \x08\x0b\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1f\x0e\x0f\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03\x20\x02\x15\n\r\n\x05\x04\x04\x02\x01\x04\x12\
    \x04\x20\x02\x1f\x10\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x20\x02\x06\
    \n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x20\x07\x10\n\x0c\n\x05\x04\x04\
    \x02\x01\x03\x12\x03\x20\x13\x14\n\x0b\n\x04\x04\x04\x02\x02\x12\x03!\
    \x02\x12\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04!\x02\x20\x15\n\x0c\n\x05\
    \x04\x04\x02\x02\x05\x12\x03!\x02\x07\n\x0c\n\x05\x04\x04\x02\x02\x01\
    \x12\x03!\x08\r\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03!\x10\x11\n\n\n\
    \x02\x04\x05\x12\x04$\0(\x01\n\n\n\x03\x04\x05\x01\x12\x03$\x08\x14\n\
    \x0b\n\x04\x04\x05\x02\0\x12\x03%\x02\x14\n\r\n\x05\x04\x05\x02\0\x04\
    \x12\x04%\x02$\x16\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03%\x02\x08\n\x0c\
    \n\x05\x04\x05\x02\0\x01\x12\x03%\t\x0f\n\x0c\n\x05\x04\x05\x02\0\x03\
    \x12\x03%\x12\x13\n\x0b\n\x04\x04\x05\x02\x01\x12\x03&\x02\x13\n\r\n\x05\
    \x04\x05\x02\x01\x04\x12\x04&\x02%\x14\n\x0c\n\x05\x04\x05\x02\x01\x05\
    \x12\x03&\x02\x08\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03&\t\x0e\n\x0c\n\
    \x05\x04\x05\x02\x01\x03\x12\x03&\x11\x12\n\x0b\n\x04\x04\x05\x02\x02\
    \x12\x03'\x02\x1b\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03'\x02\n\n\x0c\n\
    \x05\x04\x05\x02\x02\x06\x12\x03'\x0b\x11\n\x0c\n\x05\x04\x05\x02\x02\
    \x01\x12\x03'\x12\x16\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03'\x19\x1a\n\
    \n\n\x02\x06\0\x12\x04*\0/\x01\n\n\n\x03\x06\0\x01\x12\x03*\x08\x11\n\
    \x0b\n\x04\x06\0\x02\0\x12\x03+\x02.\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    +\x06\t\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03+\n\x14\n\x0c\n\x05\x06\0\x02\
    \0\x03\x12\x03+\x1f*\n\x0b\n\x04\x06\0\x02\x01\x12\x03,\x02*\n\x0c\n\x05\
    \x06\0\x02\x01\x01\x12\x03,\x06\t\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03,\
    \n\x10\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03,\x1b&\n\x0b\n\x04\x06\0\x02\
    \x02\x12\x03-\x021\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03-\x06\x0c\n\x0c\
    \n\x05\x06\0\x02\x02\x02\x12\x03-\r\x17\n\x0c\n\x05\x06\0\x02\x02\x03\
    \x12\x03-\"-\n\x0b\n\x04\x06\0\x02\x03\x12\x03.\x021\n\x0c\n\x05\x06\0\
    \x02\x03\x01\x12\x03.\x06\n\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03.\x0b\
    \x16\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03.!-b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
