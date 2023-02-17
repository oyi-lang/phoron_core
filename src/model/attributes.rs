#[derive(Debug)]
pub enum AttributeInfo {
    SourceFile {
        attribute_name_index: u16,
        attribute_length: u32,
        sourcefile_index: u16,
    },

    ConstantValue {
        attribute_name_index: u16,
        attribute_length: u32,
        constantvalue_index: u16,
    },

    Code {
        attribute_name_index: u16,
        attribute_length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionHandler>,
        code_attributes_count: u16,
        code_attributes: Vec<AttributeInfo>,
    },

    Exceptions {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },

    LineNumberTable {
        attribute_name_index: u16,
        attribute_length: u32,
        line_number_table_length: u16,
        line_number_table: Vec<LineNumber>,
    },

    LocalVariableTable {
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_table_length: u16,
        local_variable_table: Vec<LocalVariable>,
    },

    StackMapTable {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_entries: u16,
        entries: Vec<StackMapFrame>,
    },

    InnerClasses {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        classes: Vec<Class>,
    },

    EnclosingMethod {
        attribute_name_index: u16,
        attribute_length: u32,
        class_index: u16,
        method_index: u16,
    },

    Synthetic {
        attribute_name_index: u16,
        attribute_length: u32,
    },

    Signature {
        attribute_name_index: u16,
        attribute_length: u32,
        signature_index: u16,
    },

    SourceDebugExtension {
        attribute_name_index: u16,
        attribute_length: u32,
        debug_extension: Vec<u8>,
    },

    LocalVariableTypeTable {
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_type_table_length: u16,
        local_variable_type_table: Vec<LocalVariableType>,
    },

    Deprecated {
        attribute_name_index: u16,
        attribute_length: u32,
    },

    RuntimeVisibleAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        annotations: Vec<Annotation>,
    },

    RuntimeInvisibleAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        annotations: Vec<Annotation>,
    },

    RuntimeVisibleParameterAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        num_parameters: u8,
        parameter_annotations: Vec<ParameterAnnotation>,
    },

    RuntimeInvisibleParameterAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        num_parameters: u8,
        parameter_annotations: Vec<ParameterAnnotation>,
    },

    RuntimeVisibleTypeAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        annotations: Vec<TypeAnnotation>,
    },

    RuntimeInvisibleTypeAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        num_annotations: u16,
        annotations: Vec<TypeAnnotation>,
    },

    AnnotationDefault {
        attribute_name_index: u16,
        attribute_length: u32,
        default_value: ElementValue,
    },

    BootstrapMethods {
        attribute_name_index: u16,
        attribute_length: u32,
        num_bootstrap_methods: u16,
        bootstrap_methods: Vec<BootstrapMethod>,
    },

    MethodParameters {
        attribute_name_index: u16,
        attribute_length: u32,
        parameters_count: u8,
        parameters: Vec<Parameter>,
    },

    Module {
        attribute_name_index: u16,
        attribute_length: u32,
        module_name_index: u16,
        module_flags: u16,
        module_version_index: u16,
        requires_count: u16,
        requires: Vec<Require>,
        exports_count: u16,
        exports: Vec<Export>,
        opens_count: u16,
        opens: Vec<Open>,
        uses_count: u16,
        uses_index: Vec<u16>,
        provides_count: u16,
        provides: Vec<Provide>,
    },

    ModulePackages {
        attribute_name_index: u16,
        attribute_length: u32,
        package_count: u16,
        package_index: Vec<u16>,
    },

    ModuleMainClass {
        attribute_name_index: u16,
        attribute_length: u32,
        main_class_index: u16,
    },

    NestHost {
        attribute_name_index: u16,
        attribute_length: u32,
        host_class_index: u16,
    },

    NestMembers {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        classes: Vec<u16>,
    },

    Record {
        attribute_name_index: u16,
        attribute_length: u32,
        components_count: u16,
        components: Vec<RecordComponentInfo>,
    },

    PermittedSubclasses {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        classes: Vec<u16>,
    },
}

#[derive(Default, Debug)]
pub struct RecordComponentInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Default, Debug)]
pub struct Provide {
    pub provides_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}

#[derive(Default, Debug)]
pub struct Open {
    pub opens_index: u16,
    pub opens_flags: u16,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

#[derive(Default, Debug)]
pub struct Export {
    pub exports_index: u16,
    pub exports_flags: u16,
    pub exports_to_count: u16,
    pub exports_to_index: Vec<u16>,
}

#[derive(Default, Debug)]
pub struct Require {
    pub requires_index: u16,
    pub requires_flags: u16,
    pub requires_version_index: u16,
}

#[derive(Default, Debug)]
pub struct Parameter {
    pub name_index: u16,
    pub access_flags: u16,
}

#[derive(Default, Debug)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Default, Debug)]
pub enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: u8,
    },
    SuperTypeTarget {
        supertype_index: u16,
    },
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    #[default]
    EmptyTarget,
    FormalParameterTarget {
        formal_parameter_index: u8,
    },
    ThrowsTarget {
        throws_type_index: u16,
    },
    LocalVarTarget {
        table_length: u16,
        table: Vec<LocalVarEntry>,
    },
    CatchTarget {
        exception_table_index: u16,
    },
    OffsetTarget {
        offset: u16,
    },
    TypeArgumentTarget {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Default, Debug)]
pub struct Path {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

#[derive(Default, Debug)]
pub struct TypePath {
    pub path_length: u8,
    pub path: Vec<Path>,
}

#[derive(Default, Debug)]
pub struct LocalVarEntry {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Default, Debug)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TargetInfo,
    pub target_path: TypePath,
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Default, Debug)]
pub struct ParameterAnnotation {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub enum ElementValue {
    ConstValueIndex {
        tag: u8,
        const_value_index: u16,
    },
    ClassInfoIndex {
        tag: u8,
        class_info_index: u16,
    },
    EnumConstValue {
        tag: u8,
        type_name_index: u16,
        const_name_index: u16,
    },
    AnnotationValue {
        tag: u8,
        annotation: Annotation,
    },
    ArrayValue {
        tag: u8,
        num_values: u16,
        values: Vec<ElementValue>,
    },
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Default, Debug)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Default, Debug)]
pub struct Class {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVariableInfo { tag: u8 },
    IntegerVariableInfo { tag: u8 },
    FloatVariableInfo { tag: u8 },
    NullVariableInfo { tag: u8 },
    UninitializedThisVariableInfo { tag: u8 },
    ObjectVariableInfo { tag: u8, cpool_index: u16 },
    UninitializedVariableInfo { tag: u8, offset: u16 },
    LongVariableInfo { tag: u8 },
    DoubleVariableInfo { tag: u8 },
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: Vec<VerificationTypeInfo>,
    },

    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: Vec<VerificationTypeInfo>,
    },

    ChopFrame {
        frame_type: u8,
        offset_delta: u16,
    },

    SameFrameExtended {
        frame_type: u8,
        offset_delta: u16,
    },

    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },

    FullFrame {
        frame_type: u8,
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: u16,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Default, Debug)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Default, Debug)]
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Default, Debug)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Default, Debug)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

pub mod predefined_attributes {
    pub const SOURCE_FILE: &'static str = "SourceFile";
    pub const CONSTANT_VALUE: &'static str = "ConstantValue";
    pub const CODE: &'static str = "Code";
    pub const EXCEPTIONS: &'static str = "Exceptions";
    pub const LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
    pub const LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
    pub const STACK_MAP_TABLE: &'static str = "StackMapTable";
    pub const INNER_CLASSES: &'static str = "InnerClasses";
    pub const ENCLOSING_METHOD: &'static str = "EnclosingMethod";
    pub const SYNTHETIC: &'static str = "Synthetic";
    pub const SIGNATURE: &'static str = "Signature";
    pub const SOURCE_DEBUG_EXTENSION: &'static str = "SourceDebugExtension";
    pub const LOCAL_VARIABLE_TYPE_TABLE: &'static str = "LocalVariableTypeTable";
    pub const DEPRECATED: &'static str = "Deprecated";
    pub const RUNTIME_VISIBLE_ANNOTATIONS: &'static str = "RuntimeVisibleAnnotations";
    pub const RUNTIME_INVISIBLE_ANNOTATIONS: &'static str = "RuntimeInvisibleAnnotations";
    pub const RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: &'static str =
        "RuntimeVisibleParameterAnnotations";
    pub const RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: &'static str =
        "RuntimeInvisibleParameterAnnotations";
    pub const RUNTIME_VISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeVisibleTypeAnnotations";
    pub const RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: &'static str = "RuntimeInvisibleTypeAnnotations";
    pub const ANNOTATION_DEFAULT: &'static str = "AnnotationDefault";
    pub const BOOTSTRAP_METHODS: &'static str = "BootstrapMethods";
    pub const METHOD_PARAMETERS: &'static str = "MethodParameters";
    pub const MODULE: &'static str = "Module";
    pub const MODULE_PACKAGES: &'static str = "ModulePackages";
    pub const MODULE_MAIN_CLASS: &'static str = "ModuleMainClass";
    pub const NEST_HOST: &'static str = "NestHost";
    pub const NEST_MEMBERS: &'static str = "NestMembers";
    pub const RECORD: &'static str = "Record";
    pub const PERMITTED_SUBCLASSES: &'static str = "PermittedSubclasses";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rercord_component_info() {
        let _rec_comp_info = RecordComponentInfo::default();
    }

    #[test]
    fn test_default_provide() {
        let _provide = Provide::default();
    }

    #[test]
    fn test_default_open() {
        let _open = Open::default();
    }

    #[test]
    fn test_default_export() {
        let _export = Export::default();
    }

    #[test]
    fn test_default_require() {
        let _require = Require::default();
    }

    #[test]
    fn test_default_parameter() {
        let _parameter = Parameter::default();
    }

    #[test]
    fn test_default_bootstrap_method() {
        let _bootstrap_method = BootstrapMethod::default();
    }

    #[test]
    fn test_default_path() {
        let _path = Path::default();
    }

    #[test]
    fn test_default_type_path() {
        let _type_path = TypePath::default();
    }

    #[test]
    fn test_default_local_var_entry() {
        let _local_var_entry = LocalVarEntry::default();
    }

    #[test]
    fn test_default_type_annotation() {
        let _type_annotation = TypeAnnotation::default();
    }

    #[test]
    fn test_default_parameter_annotation() {
        let _parameter_annotation = ParameterAnnotation::default();
    }

    #[test]
    fn test_default_annotation() {
        let _annot = Annotation::default();
    }

    #[test]
    fn test_default_class() {
        let _class = Class::default();
    }

    #[test]
    fn test_default_local_variable_type() {
        let _local_var_type = LocalVariableType::default();
    }

    #[test]
    fn test_default_exception_handler() {
        let _exception_handler = ExceptionHandler::default();
    }

    #[test]
    fn test_default_line_number() {
        let _line_number = LineNumber::default();
    }

    #[test]
    fn test_default_local_variable() {
        let _local_var = LocalVariable::default();
    }
}
