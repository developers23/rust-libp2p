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
pub struct PublicKey {
    // message fields
    Type: ::std::option::Option<KeyType>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublicKey {}

impl PublicKey {
    pub fn new() -> PublicKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublicKey {
        static mut instance: ::protobuf::lazy::Lazy<PublicKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublicKey,
        };
        unsafe {
            instance.get(PublicKey::new)
        }
    }

    // required .KeyType Type = 1;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: KeyType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type(&self) -> KeyType {
        self.Type.unwrap_or(KeyType::RSA)
    }

    fn get_Type_for_reflect(&self) -> &::std::option::Option<KeyType> {
        &self.Type
    }

    fn mut_Type_for_reflect(&mut self) -> &mut ::std::option::Option<KeyType> {
        &mut self.Type
    }

    // required bytes Data = 2;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        }
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data(&self) -> &[u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_Data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.Data
    }

    fn mut_Data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.Data
    }
}

impl ::protobuf::Message for PublicKey {
    fn is_initialized(&self) -> bool {
        if self.Type.is_none() {
            return false;
        }
        if self.Data.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.Type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Data)?;
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
        if let Some(v) = self.Type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.Data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.Data.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for PublicKey {
    fn new() -> PublicKey {
        PublicKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublicKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<KeyType>>(
                    "Type",
                    PublicKey::get_Type_for_reflect,
                    PublicKey::mut_Type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Data",
                    PublicKey::get_Data_for_reflect,
                    PublicKey::mut_Data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublicKey>(
                    "PublicKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublicKey {
    fn clear(&mut self) {
        self.clear_Type();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublicKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrivateKey {
    // message fields
    Type: ::std::option::Option<KeyType>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrivateKey {}

impl PrivateKey {
    pub fn new() -> PrivateKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrivateKey {
        static mut instance: ::protobuf::lazy::Lazy<PrivateKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrivateKey,
        };
        unsafe {
            instance.get(PrivateKey::new)
        }
    }

    // required .KeyType Type = 1;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: KeyType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type(&self) -> KeyType {
        self.Type.unwrap_or(KeyType::RSA)
    }

    fn get_Type_for_reflect(&self) -> &::std::option::Option<KeyType> {
        &self.Type
    }

    fn mut_Type_for_reflect(&mut self) -> &mut ::std::option::Option<KeyType> {
        &mut self.Type
    }

    // required bytes Data = 2;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        }
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data(&self) -> &[u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_Data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.Data
    }

    fn mut_Data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.Data
    }
}

impl ::protobuf::Message for PrivateKey {
    fn is_initialized(&self) -> bool {
        if self.Type.is_none() {
            return false;
        }
        if self.Data.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.Type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Data)?;
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
        if let Some(v) = self.Type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.Data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.Data.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for PrivateKey {
    fn new() -> PrivateKey {
        PrivateKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrivateKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<KeyType>>(
                    "Type",
                    PrivateKey::get_Type_for_reflect,
                    PrivateKey::mut_Type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Data",
                    PrivateKey::get_Data_for_reflect,
                    PrivateKey::mut_Data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrivateKey>(
                    "PrivateKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrivateKey {
    fn clear(&mut self) {
        self.clear_Type();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrivateKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum KeyType {
    RSA = 0,
    Ed25519 = 1,
    Secp256k1 = 2,
}

impl ::protobuf::ProtobufEnum for KeyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KeyType> {
        match value {
            0 => ::std::option::Option::Some(KeyType::RSA),
            1 => ::std::option::Option::Some(KeyType::Ed25519),
            2 => ::std::option::Option::Some(KeyType::Secp256k1),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [KeyType] = &[
            KeyType::RSA,
            KeyType::Ed25519,
            KeyType::Secp256k1,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<KeyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("KeyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for KeyType {
}

impl ::protobuf::reflect::ProtobufValue for KeyType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nkeys.proto\"=\n\tPublicKey\x12\x1c\n\x04Type\x18\x01\x20\x02(\x0e2\
    \x08.KeyTypeR\x04type\x12\x12\n\x04Data\x18\x02\x20\x02(\x0cR\x04data\">\
    \n\nPrivateKey\x12\x1c\n\x04Type\x18\x01\x20\x02(\x0e2\x08.KeyTypeR\x04t\
    ype\x12\x12\n\x04Data\x18\x02\x20\x02(\x0cR\x04data*.\n\x07KeyType\x12\
    \x07\n\x03RSA\x10\0\x12\x0b\n\x07Ed25519\x10\x01\x12\r\n\tSecp256k1\x10\
    \x02J\xdf\x03\n\x06\x12\x04\0\0\x0e\x01\n\n\n\x02\x05\0\x12\x04\0\0\x04\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\0\x05\x0c\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03\x01\x02\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x01\x02\x05\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03\x01\x08\t\n\x0b\n\x04\x05\0\x02\x01\x12\x03\
    \x02\x02\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x02\x02\t\n\x0c\n\x05\
    \x05\0\x02\x01\x02\x12\x03\x02\x0c\r\n\x0b\n\x04\x05\0\x02\x02\x12\x03\
    \x03\x02\x10\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x03\x02\x0b\n\x0c\n\
    \x05\x05\0\x02\x02\x02\x12\x03\x03\x0e\x0f\n\n\n\x02\x04\0\x12\x04\x06\0\
    \t\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x11\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x07\x02\x1c\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x07\x02\n\n\x0c\
    \n\x05\x04\0\x02\0\x06\x12\x03\x07\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x07\x13\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x1a\x1b\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x02\x1a\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03\x08\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x08\x0b\x10\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\x11\x15\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x08\x18\x19\n\n\n\x02\x04\x01\x12\x04\x0b\0\x0e\x01\n\n\
    \n\x03\x04\x01\x01\x12\x03\x0b\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\
    \x0c\x02\x1c\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\
    \x04\x01\x02\0\x06\x12\x03\x0c\x0b\x12\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x0c\x13\x17\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0c\x1a\x1b\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x03\r\x02\x1a\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03\r\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\r\x0b\x10\n\x0c\
    \n\x05\x04\x01\x02\x01\x01\x12\x03\r\x11\x15\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\r\x18\x19\
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
