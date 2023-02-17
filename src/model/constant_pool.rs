// todo: move ths to a higher-level client such as `phoron_asm`
pub mod tags {
    pub const CONSTANT_INVALID_DEFAULT: u8 = 255;
    pub const CONSTANT_CLASS: u8 = 7;
    pub const CONSTANT_FIELD_REF: u8 = 9;
    pub const CONSTANT_METHOD_REF: u8 = 10;
    pub const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
    pub const CONSTANT_STRING: u8 = 8;
    pub const CONSTANT_INTEGER: u8 = 3;
    pub const CONSTANT_FLOAT: u8 = 4;
    pub const CONSTANT_LONG: u8 = 5;
    pub const CONSTANT_DOUBLE: u8 = 6;
    pub const CONSTANT_NAME_AND_TYPE: u8 = 12;
    pub const CONSTANT_UTF8: u8 = 1;
    pub const CONSTANT_METHOD_HANDLE: u8 = 15;
    pub const CONSTANT_METHOD_TYPE: u8 = 16;
    pub const CONSTANT_DYNAMIC: u8 = 17;
    pub const CONSTANT_INVOKE_DYNAMIC: u8 = 18;
    pub const CONSTANT_MODULE: u8 = 19;
    pub const CONSTANT_PACKAGE: u8 = 20;
}

pub mod types {
    #[derive(Debug, Clone)]
    pub enum CpInfo {
        ConstantClassInfo {
            tag: u8,
            name_index: u16,
        },
        ConstantFieldrefInfo {
            tag: u8,
            class_index: u16,
            name_and_type_index: u16,
        },

        ConstantMethodrefInfo {
            tag: u8,
            class_index: u16,
            name_and_type_index: u16,
        },

        ConstantInterfaceMethodrefInfo {
            tag: u8,
            class_index: u16,
            name_and_type_index: u16,
        },

        ConstantStringInfo {
            tag: u8,
            string_index: u16,
        },

        ConstantIntegerInfo {
            tag: u8,
            bytes: u32,
        },

        ConstantFloatInfo {
            tag: u8,
            bytes: u32,
        },

        ConstantLongInfo {
            tag: u8,
            high_bytes: u32,
            low_bytes: u32,
        },

        ConstantDoubleInfo {
            tag: u8,
            high_bytes: u32,
            low_bytes: u32,
        },

        ConstantNameAndTypeInfo {
            tag: u8,
            name_index: u16,
            descriptor_index: u16,
        },

        ConstantUtf8Info {
            tag: u8,
            length: u16,
            bytes: Vec<u8>,
        },

        ConstantMethodHandleInfo {
            tag: u8,
            reference_kind: u8,
            reference_index: u16,
        },

        ConstantMethodTypeInfo {
            tag: u8,
            descriptor_index: u16,
        },

        ConstantDynamicInfo {
            tag: u8,
            bootstrap_method_attr_index: u16,
            name_and_type_index: u16,
        },

        ConstantInvokeDynamicInfo {
            tag: u8,
            bootstrap_method_attr_index: u16,
            name_and_type_index: u16,
        },

        ConstantModuleInfo {
            tag: u8,
            name_index: u16,
        },

        ConstantPackageInfo {
            tag: u8,
            name_index: u16,
        },
    }
}
