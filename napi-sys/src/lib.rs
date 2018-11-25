#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod napi;

pub use self::napi::*;

pub struct Status;

#[allow(non_upper_case_globals)]
impl Status {
    pub const Ok: napi_status = napi_status_napi_ok;
    pub const InvalidArg: napi_status = napi_status_napi_invalid_arg;
    pub const ObjectExpected: napi_status = napi_status_napi_object_expected;
    pub const StringExpected: napi_status = napi_status_napi_string_expected;
    pub const NameExpected: napi_status = napi_status_napi_name_expected;
    pub const FunctionExpected: napi_status = napi_status_napi_function_expected;
    pub const NumberExpected: napi_status = napi_status_napi_number_expected;
    pub const BooleanExpected: napi_status = napi_status_napi_boolean_expected;
    pub const ArrayExpected: napi_status = napi_status_napi_array_expected;
    pub const GenericFailure: napi_status = napi_status_napi_generic_failure;
    pub const PendingException: napi_status = napi_status_napi_pending_exception;
    pub const Cancelled: napi_status = napi_status_napi_cancelled;
    pub const EscapeCalledTwice: napi_status = napi_status_napi_escape_called_twice;
    pub const HandleScopeMismatch: napi_status = napi_status_napi_handle_scope_mismatch;
    pub const CallbackScopeMismatch: napi_status = napi_status_napi_callback_scope_mismatch;
    pub const QueueFull: napi_status = napi_status_napi_queue_full;
    pub const Closing: napi_status = napi_status_napi_closing;
    pub const BigintExpected: napi_status = napi_status_napi_bigint_expected;
}

pub struct ValueType;

#[allow(non_upper_case_globals)]
impl ValueType {
    pub const Undefined: napi_valuetype = napi_valuetype_napi_undefined;
    pub const Null: napi_valuetype = napi_valuetype_napi_null;
    pub const Boolean: napi_valuetype = napi_valuetype_napi_boolean;
    pub const Number: napi_valuetype = napi_valuetype_napi_number;
    pub const String: napi_valuetype = napi_valuetype_napi_string;
    pub const Symbol: napi_valuetype = napi_valuetype_napi_symbol;
    pub const Object: napi_valuetype = napi_valuetype_napi_object;
    pub const Function: napi_valuetype = napi_valuetype_napi_function;
    pub const External: napi_valuetype = napi_valuetype_napi_external;
    pub const Bigint: napi_valuetype = napi_valuetype_napi_bigint;
}

pub struct PropertyAttributes;

#[allow(non_upper_case_globals)]
impl PropertyAttributes {
    pub const Default: napi_property_attributes = napi_property_attributes_napi_default;
    pub const Writable: napi_property_attributes = napi_property_attributes_napi_writable;
    pub const Enumerable: napi_property_attributes = napi_property_attributes_napi_enumerable;
    pub const Configurable: napi_property_attributes = napi_property_attributes_napi_configurable;
    pub const Static: napi_property_attributes = napi_property_attributes_napi_static;
}

pub struct TypedArrayType;

#[allow(non_upper_case_globals)]
impl TypedArrayType {
    pub const Int8: napi_typedarray_type = napi_typedarray_type_napi_int8_array;
    pub const Uint8: napi_typedarray_type = napi_typedarray_type_napi_uint8_array;
    pub const Uint8_clamped: napi_typedarray_type = napi_typedarray_type_napi_uint8_clamped_array;
    pub const Int16: napi_typedarray_type = napi_typedarray_type_napi_int16_array;
    pub const Uint16: napi_typedarray_type = napi_typedarray_type_napi_uint16_array;
    pub const Int32: napi_typedarray_type = napi_typedarray_type_napi_int32_array;
    pub const Uint32: napi_typedarray_type = napi_typedarray_type_napi_uint32_array;
    pub const Float32: napi_typedarray_type = napi_typedarray_type_napi_float32_array;
    pub const Float64: napi_typedarray_type = napi_typedarray_type_napi_float64_array;
    pub const Bigint64: napi_typedarray_type = napi_typedarray_type_napi_bigint64_array;
    pub const Biguint64: napi_typedarray_type = napi_typedarray_type_napi_biguint64_array;
}

pub struct ThreadsafeFunctionCallMode;

#[allow(non_upper_case_globals)]
impl ThreadsafeFunctionCallMode {
    pub const NonBlocking: napi_threadsafe_function_call_mode =
        napi_threadsafe_function_call_mode_napi_tsfn_nonblocking;
    pub const Blocking: napi_threadsafe_function_call_mode =
        napi_threadsafe_function_call_mode_napi_tsfn_blocking;
}
