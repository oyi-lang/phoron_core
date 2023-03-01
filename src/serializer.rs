//! Module to construct a Java (JVM) raw class file bytes from the class fileobject model.

use crate::{
    error::SerializeError,
    model::{attributes::*, constant_pool::types::*, *},
    rw::writer::Writer,
};
use std::io::Write;

pub type SerializeResult<T> = Result<T, SerializeError>;

/// The Serializer takes in the JVM class file object model, and writes a stream of valid
/// JVM bytecode to the supplied writer.
pub struct Serializer<'a, W: Write> {
    writer: Writer<'a, W>,
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(writer: Writer<'a, W>) -> Self {
        Serializer { writer }
    }

    fn serialize_target_info(&mut self, target_info: &TargetInfo) -> SerializeResult<()> {
        match target_info {
            TargetInfo::TypeParameterTarget {
                type_parameter_index,
            } => self.writer.write_unsigned_byte(*type_parameter_index)?,

            TargetInfo::SuperTypeTarget { supertype_index } => {
                self.writer.write_unsigned_short(*supertype_index)?
            }

            TargetInfo::TypeParameterBoundTarget {
                type_parameter_index,
                bound_index,
            } => {
                self.writer.write_unsigned_byte(*type_parameter_index)?;
                self.writer.write_unsigned_byte(*bound_index)?;
            }

            TargetInfo::EmptyTarget => {}

            TargetInfo::FormalParameterTarget {
                formal_parameter_index,
            } => self.writer.write_unsigned_byte(*formal_parameter_index)?,

            TargetInfo::ThrowsTarget { throws_type_index } => {
                self.writer.write_unsigned_short(*throws_type_index)?
            }

            TargetInfo::LocalVarTarget {
                table_length,
                table,
            } => {
                self.writer.write_unsigned_short(*table_length)?;
                for local_var in table {
                    self.writer.write_unsigned_short(local_var.start_pc)?;
                    self.writer.write_unsigned_short(local_var.length)?;
                    self.writer.write_unsigned_short(local_var.index)?;
                }
            }

            TargetInfo::CatchTarget {
                exception_table_index,
            } => self.writer.write_unsigned_short(*exception_table_index)?,

            TargetInfo::OffsetTarget { offset } => self.writer.write_unsigned_short(*offset)?,

            TargetInfo::TypeArgumentTarget {
                offset,
                type_argument_index,
            } => {
                self.writer.write_unsigned_short(*offset)?;
                self.writer.write_unsigned_byte(*type_argument_index)?;
            }
        }

        Ok(())
    }

    fn serialize_type_path(&mut self, type_path: &TypePath) -> SerializeResult<()> {
        self.writer.write_unsigned_byte(type_path.path_length)?;
        for path in &type_path.path {
            self.writer.write_unsigned_byte(path.type_path_kind)?;
            self.writer.write_unsigned_byte(path.type_argument_index)?;
        }

        Ok(())
    }

    fn serialize_type_annotation(
        &mut self,
        type_annotation: &TypeAnnotation,
    ) -> SerializeResult<()> {
        self.writer
            .write_unsigned_byte(type_annotation.target_type)?;
        self.serialize_target_info(&type_annotation.target_info)?;
        self.serialize_type_path(&type_annotation.target_path)?;
        self.writer
            .write_unsigned_short(type_annotation.type_index)?;
        self.writer
            .write_unsigned_short(type_annotation.num_element_value_pairs)?;

        for ev_pair in &type_annotation.element_value_pairs {
            self.serialize_element_value_pair(ev_pair)?;
        }

        Ok(())
    }

    fn serialize_element_value(&mut self, value: &ElementValue) -> SerializeResult<()> {
        match value {
            ElementValue::ConstValueIndex {
                tag,
                const_value_index,
            } => {
                self.writer.write_unsigned_byte(*tag)?;
                self.writer.write_unsigned_short(*const_value_index)?;
            }

            ElementValue::ClassInfoIndex {
                tag,
                class_info_index,
            } => {
                self.writer.write_unsigned_byte(*tag)?;
                self.writer.write_unsigned_short(*class_info_index)?;
            }

            ElementValue::EnumConstValue {
                tag,
                type_name_index,
                const_name_index,
            } => {
                self.writer.write_unsigned_byte(*tag)?;
                self.writer.write_unsigned_short(*type_name_index)?;
                self.writer.write_unsigned_short(*const_name_index)?;
            }

            ElementValue::AnnotationValue { tag, annotation } => {
                self.writer.write_unsigned_byte(*tag)?;
                self.serialize_annotation(&annotation)?;
            }

            ElementValue::ArrayValue {
                tag,
                num_values,
                values,
            } => {
                self.writer.write_unsigned_byte(*tag)?;

                self.writer.write_unsigned_short(*num_values)?;
                for value in values {
                    self.serialize_element_value(value)?;
                }
            }
        }

        Ok(())
    }

    fn serialize_element_value_pair(&mut self, ev_pair: &ElementValuePair) -> SerializeResult<()> {
        self.writer
            .write_unsigned_short(ev_pair.element_name_index)?;
        self.serialize_element_value(&ev_pair.value)?;
        Ok(())
    }

    fn serialize_annotation(&mut self, annotation: &Annotation) -> SerializeResult<()> {
        self.writer.write_unsigned_short(annotation.type_index)?;
        self.writer
            .write_unsigned_short(annotation.num_element_value_pairs)?;

        for ev_pair in &annotation.element_value_pairs {
            self.serialize_element_value_pair(&ev_pair)?;
        }

        Ok(())
    }

    fn serialize_verification_type_info(
        &mut self,
        ver_type_info: &VerificationTypeInfo,
    ) -> SerializeResult<()> {
        match ver_type_info {
            VerificationTypeInfo::TopVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }

            VerificationTypeInfo::IntegerVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }

            VerificationTypeInfo::FloatVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }

            VerificationTypeInfo::NullVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }

            VerificationTypeInfo::UninitializedThisVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }

            VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index } => {
                self.writer.write_unsigned_byte(*tag)?;
                self.writer.write_unsigned_short(*cpool_index)?;
            }

            VerificationTypeInfo::UninitializedVariableInfo { tag, offset } => {
                self.writer.write_unsigned_byte(*tag)?;
                self.writer.write_unsigned_short(*offset)?;
            }

            VerificationTypeInfo::LongVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }

            VerificationTypeInfo::DoubleVariableInfo { tag } => {
                self.writer.write_unsigned_byte(*tag)?
            }
        }

        Ok(())
    }

    /// Serialize the attributes of the class file.
    fn serialize_attributes(&mut self, attributes: &[AttributeInfo]) -> SerializeResult<()> {
        for attribute in attributes {
            match attribute {
                AttributeInfo::SourceFile {
                    attribute_name_index,
                    attribute_length,
                    sourcefile_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*sourcefile_index)?;
                }

                AttributeInfo::ConstantValue {
                    attribute_name_index,
                    attribute_length,
                    constantvalue_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*constantvalue_index)?;
                }

                AttributeInfo::Code {
                    attribute_name_index,
                    attribute_length,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table_length,
                    exception_table,
                    code_attributes_count,
                    code_attributes,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*max_stack)?;
                    self.writer.write_unsigned_short(*max_locals)?;

                    self.writer.write_unsigned_int(*code_length)?;
                    for b in code {
                        self.writer.write_unsigned_byte(*b)?;
                    }

                    self.writer.write_unsigned_short(*exception_table_length)?;
                    for ehandler in exception_table {
                        self.writer.write_unsigned_short(ehandler.start_pc)?;
                        self.writer.write_unsigned_short(ehandler.end_pc)?;
                        self.writer.write_unsigned_short(ehandler.handler_pc)?;

                        let mut catch_type = ehandler.catch_type;
                        if catch_type != 0 {
                            catch_type += 1;
                        }
                        self.writer.write_unsigned_short(catch_type)?;
                    }

                    self.writer.write_unsigned_short(*code_attributes_count)?;
                    self.serialize_attributes(code_attributes)?;
                }

                AttributeInfo::Exceptions {
                    attribute_name_index,
                    attribute_length,
                    number_of_exceptions,
                    exception_index_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*number_of_exceptions)?;

                    for idx in exception_index_table {
                        self.writer
                            .write_unsigned_short(if *idx == 0 { 0 } else { idx + 1 })?;
                    }
                }

                AttributeInfo::LineNumberTable {
                    attribute_name_index,
                    attribute_length,
                    line_number_table_length,
                    line_number_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer
                        .write_unsigned_short(*line_number_table_length)?;

                    for line_number in line_number_table {
                        self.writer.write_unsigned_short(line_number.start_pc)?;
                        self.writer.write_unsigned_short(line_number.line_number)?;
                    }
                }

                AttributeInfo::LocalVariableTable {
                    attribute_name_index,
                    attribute_length,
                    local_variable_table_length,
                    local_variable_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer
                        .write_unsigned_short(*local_variable_table_length)?;

                    for local_var in local_variable_table {
                        self.writer.write_unsigned_short(local_var.start_pc)?;
                        self.writer.write_unsigned_short(local_var.length)?;
                        self.writer.write_unsigned_short(local_var.name_index)?;
                        self.writer
                            .write_unsigned_short(local_var.descriptor_index)?;
                        self.writer.write_unsigned_short(local_var.index)?;
                    }
                }

                AttributeInfo::StackMapTable {
                    attribute_name_index,
                    attribute_length,
                    number_of_entries,
                    entries,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*number_of_entries)?;
                    for entry in entries {
                        match entry {
                            StackMapFrame::SameFrame { frame_type } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                            }

                            StackMapFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                                self.serialize_verification_type_info(&stack[0])?;
                            }

                            StackMapFrame::SameLocals1StackItemFrameExtended {
                                frame_type,
                                offset_delta,
                                stack,
                            } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                                self.writer.write_unsigned_short(*offset_delta)?;
                                self.serialize_verification_type_info(&stack[0])?;
                            }

                            StackMapFrame::ChopFrame {
                                frame_type,
                                offset_delta,
                            } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                                self.writer.write_unsigned_short(*offset_delta)?;
                            }

                            StackMapFrame::SameFrameExtended {
                                frame_type,
                                offset_delta,
                            } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                                self.writer.write_unsigned_short(*offset_delta)?;
                            }

                            StackMapFrame::AppendFrame {
                                frame_type,
                                offset_delta,
                                locals,
                            } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                                self.writer.write_unsigned_short(*offset_delta)?;

                                for local in locals {
                                    self.serialize_verification_type_info(local)?;
                                }
                            }

                            StackMapFrame::FullFrame {
                                frame_type,
                                offset_delta,
                                number_of_locals,
                                locals,
                                number_of_stack_items,
                                stack,
                            } => {
                                self.writer.write_unsigned_byte(*frame_type)?;
                                self.writer.write_unsigned_short(*offset_delta)?;

                                self.writer.write_unsigned_short(*number_of_locals)?;
                                for local in locals {
                                    self.serialize_verification_type_info(local)?;
                                }

                                self.writer.write_unsigned_short(*number_of_stack_items)?;
                                for st_item in stack {
                                    self.serialize_verification_type_info(st_item)?;
                                }
                            }
                        }
                    }
                }

                AttributeInfo::InnerClasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*number_of_classes)?;
                    for Class {
                        inner_class_info_index,
                        outer_class_info_index,
                        inner_name_index,
                        inner_class_access_flags,
                    } in classes
                    {
                        self.writer.write_unsigned_short(*inner_class_info_index)?;
                        self.writer.write_unsigned_short(*outer_class_info_index)?;
                        self.writer.write_unsigned_short(*inner_name_index)?;
                        self.writer
                            .write_unsigned_short(*inner_class_access_flags)?;
                    }
                }

                AttributeInfo::EnclosingMethod {
                    attribute_name_index,
                    attribute_length,
                    class_index,
                    method_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*class_index)?;
                    self.writer.write_unsigned_short(*method_index)?;
                }

                AttributeInfo::Synthetic {
                    attribute_name_index,
                    attribute_length,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                }

                AttributeInfo::Signature {
                    attribute_name_index,
                    attribute_length,
                    signature_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*signature_index)?;
                }

                AttributeInfo::SourceDebugExtension {
                    attribute_name_index,
                    attribute_length,
                    debug_extension,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    for b in debug_extension {
                        self.writer.write_unsigned_byte(*b)?;
                    }
                }

                AttributeInfo::LocalVariableTypeTable {
                    attribute_name_index,
                    attribute_length,
                    local_variable_type_table_length,
                    local_variable_type_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer
                        .write_unsigned_short(*local_variable_type_table_length)?;
                    for LocalVariableType {
                        start_pc,
                        length,
                        name_index,
                        signature_index,
                        index,
                    } in local_variable_type_table
                    {
                        self.writer.write_unsigned_short(*start_pc)?;
                        self.writer.write_unsigned_short(*length)?;
                        self.writer.write_unsigned_short(*name_index)?;
                        self.writer.write_unsigned_short(*signature_index)?;
                        self.writer.write_unsigned_short(*index)?;
                    }
                }

                AttributeInfo::Deprecated {
                    attribute_name_index,
                    attribute_length,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                }

                AttributeInfo::RuntimeVisibleAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*num_annotations)?;
                    for annotation in annotations {
                        self.serialize_annotation(annotation)?;
                    }
                }

                AttributeInfo::RuntimeInvisibleAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*num_annotations)?;
                    for annotation in annotations {
                        self.serialize_annotation(annotation)?;
                    }
                }

                AttributeInfo::RuntimeVisibleParameterAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_parameters,
                    parameter_annotations,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_byte(*num_parameters)?;
                    for parameter_annotation in parameter_annotations {
                        self.writer
                            .write_unsigned_short(parameter_annotation.num_annotations)?;
                        for annotation in &parameter_annotation.annotations {
                            self.serialize_annotation(annotation)?;
                        }
                    }
                }

                AttributeInfo::RuntimeInvisibleParameterAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_parameters,
                    parameter_annotations,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_byte(*num_parameters)?;

                    for parameter_annotation in parameter_annotations {
                        self.writer
                            .write_unsigned_short(parameter_annotation.num_annotations)?;
                        for annotation in &parameter_annotation.annotations {
                            self.serialize_annotation(annotation)?;
                        }
                    }
                }

                AttributeInfo::RuntimeVisibleTypeAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*num_annotations)?;

                    for type_annotation in annotations {
                        self.serialize_type_annotation(type_annotation)?;
                    }
                }

                AttributeInfo::RuntimeInvisibleTypeAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*num_annotations)?;

                    for type_annotation in annotations {
                        self.serialize_type_annotation(type_annotation)?;
                    }
                }

                AttributeInfo::AnnotationDefault {
                    attribute_name_index,
                    attribute_length,
                    default_value,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.serialize_element_value(default_value)?;
                }

                AttributeInfo::BootstrapMethods {
                    attribute_name_index,
                    attribute_length,
                    num_bootstrap_methods,
                    bootstrap_methods,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*num_bootstrap_methods)?;
                    for bootstrap_method in bootstrap_methods {
                        self.writer
                            .write_unsigned_short(bootstrap_method.bootstrap_method_ref)?;

                        self.writer
                            .write_unsigned_short(bootstrap_method.num_bootstrap_arguments)?;
                        for bootstrap_arg in &bootstrap_method.bootstrap_arguments {
                            self.writer.write_unsigned_short(*bootstrap_arg)?;
                        }
                    }
                }

                AttributeInfo::MethodParameters {
                    attribute_name_index,
                    attribute_length,
                    parameters_count,
                    parameters,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_byte(*parameters_count)?;
                    for param in parameters {
                        self.writer.write_unsigned_short(param.name_index)?;
                        self.writer.write_unsigned_short(param.access_flags)?;
                    }
                }

                AttributeInfo::Module {
                    attribute_name_index,
                    attribute_length,
                    module_name_index,
                    module_flags,
                    module_version_index,
                    requires_count,
                    requires,
                    exports_count,
                    exports,
                    opens_count,
                    opens,
                    uses_count,
                    uses_index,
                    provides_count,
                    provides,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*module_name_index)?;
                    self.writer.write_unsigned_short(*module_flags)?;
                    self.writer.write_unsigned_short(*module_version_index)?;

                    self.writer.write_unsigned_short(*requires_count)?;
                    for require in requires {
                        self.writer.write_unsigned_short(require.requires_index)?;
                        self.writer.write_unsigned_short(require.requires_flags)?;
                        self.writer
                            .write_unsigned_short(require.requires_version_index)?;
                    }

                    self.writer.write_unsigned_short(*exports_count)?;
                    for export in exports {
                        self.writer.write_unsigned_short(export.exports_index)?;
                        self.writer.write_unsigned_short(export.exports_flags)?;
                        self.writer.write_unsigned_short(export.exports_to_count)?;

                        for s in &export.exports_to_index {
                            self.writer.write_unsigned_short(*s)?;
                        }
                    }

                    self.writer.write_unsigned_short(*opens_count)?;
                    for open in opens {
                        self.writer.write_unsigned_short(open.opens_index)?;
                        self.writer.write_unsigned_short(open.opens_flags)?;
                        self.writer.write_unsigned_short(open.opens_to_count)?;

                        for s in &open.opens_to_index {
                            self.writer.write_unsigned_short(*s)?;
                        }
                    }

                    self.writer.write_unsigned_short(*uses_count)?;
                    for s in uses_index {
                        self.writer.write_unsigned_short(*s)?;
                    }

                    self.writer.write_unsigned_short(*provides_count)?;
                    for provide in provides {
                        self.writer.write_unsigned_short(provide.provides_index)?;
                        self.writer
                            .write_unsigned_short(provide.provides_with_count)?;

                        for s in &provide.provides_with_index {
                            self.writer.write_unsigned_short(*s)?;
                        }
                    }
                }

                AttributeInfo::ModulePackages {
                    attribute_name_index,
                    attribute_length,
                    package_count,
                    package_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*package_count)?;
                    for s in package_index {
                        self.writer.write_unsigned_short(*s)?;
                    }
                }

                AttributeInfo::ModuleMainClass {
                    attribute_name_index,
                    attribute_length,
                    main_class_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*main_class_index)?;
                }

                AttributeInfo::NestHost {
                    attribute_name_index,
                    attribute_length,
                    host_class_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*host_class_index)?;
                }

                AttributeInfo::NestMembers {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*number_of_classes)?;
                    for s in classes {
                        self.writer.write_unsigned_short(*s)?;
                    }
                }

                AttributeInfo::Record {
                    attribute_name_index,
                    attribute_length,
                    components_count,
                    components,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*components_count)?;
                    for comp in components {
                        self.writer.write_unsigned_short(comp.name_index)?;
                        self.writer.write_unsigned_short(comp.descriptor_index)?;
                        self.writer.write_unsigned_short(comp.attributes_count)?;
                        self.serialize_attributes(&comp.attributes)?;
                    }
                }

                AttributeInfo::PermittedSubclasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;

                    self.writer.write_unsigned_short(*number_of_classes)?;
                    for s in classes {
                        self.writer.write_unsigned_short(*s)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Serialize the fields of the class file.
    fn serialize_fields(&mut self, fields: &[FieldInfo]) -> SerializeResult<()> {
        for field in fields {
            self.writer.write_unsigned_short(field.access_flags)?;
            self.writer.write_unsigned_short(field.name_index)?;
            self.writer.write_unsigned_short(field.descriptor_index)?;
            self.writer.write_unsigned_short(field.attributes_count)?;
            self.serialize_attributes(&field.attributes)?;
        }

        Ok(())
    }

    /// Deserialize the methods of the class file.
    fn serialize_methods(&mut self, methods: &[MethodInfo]) -> SerializeResult<()> {
        for method in methods {
            self.writer.write_unsigned_short(method.access_flags)?;
            self.writer.write_unsigned_short(method.name_index)?;
            self.writer.write_unsigned_short(method.descriptor_index)?;
            self.writer.write_unsigned_short(method.attributes_count)?;
            self.serialize_attributes(&method.attributes)?;
        }
        Ok(())
    }

    /// Serialize the contents of the Constant Pool.
    fn serialize_constant_pool(
        &mut self,
        constant_pool: &Vec<Option<CpInfo>>,
    ) -> SerializeResult<()> {
        for cp_info in constant_pool {
            if let Some(cp_info) = cp_info {
                match cp_info {
                    CpInfo::ConstantMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*class_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantClassInfo { tag, name_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                    }

                    CpInfo::ConstantFieldrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*class_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*class_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantStringInfo { tag, string_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*string_index)?;
                    }

                    CpInfo::ConstantIntegerInfo { tag, bytes } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*bytes)?;
                    }

                    CpInfo::ConstantFloatInfo { tag, bytes } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*bytes)?;
                    }

                    CpInfo::ConstantLongInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*high_bytes)?;
                        self.writer.write_unsigned_int(*low_bytes)?;
                    }

                    CpInfo::ConstantDoubleInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*high_bytes)?;
                        self.writer.write_unsigned_int(*low_bytes)?;
                    }

                    CpInfo::ConstantNameAndTypeInfo {
                        tag,
                        name_index,
                        descriptor_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                        self.writer.write_unsigned_short(*descriptor_index)?;
                    }

                    CpInfo::ConstantUtf8Info { tag, length, bytes } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*length)?;

                        for b in bytes {
                            self.writer.write_unsigned_byte(*b)?;
                        }
                    }

                    CpInfo::ConstantMethodHandleInfo {
                        tag,
                        reference_kind,
                        reference_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_byte(*reference_kind)?;
                        self.writer.write_unsigned_short(*reference_index)?;
                    }

                    CpInfo::ConstantMethodTypeInfo {
                        tag,
                        descriptor_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*descriptor_index)?;
                    }

                    CpInfo::ConstantDynamicInfo {
                        tag,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer
                            .write_unsigned_short(*bootstrap_method_attr_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantInvokeDynamicInfo {
                        tag,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer
                            .write_unsigned_short(*bootstrap_method_attr_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantModuleInfo { tag, name_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                    }

                    CpInfo::ConstantPackageInfo { tag, name_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Serialize the ClassFile object into a stream of raw JVM bytecode bytes.
    pub fn serialize(&mut self, classfile: &ClassFile) -> SerializeResult<()> {
        // Headers
        self.writer.write_unsigned_int(classfile.magic)?;
        self.writer.write_unsigned_short(classfile.minor_version)?;
        self.writer.write_unsigned_short(classfile.major_version)?;

        // Constant Pool
        assert!(classfile.constant_pool_count > 0);
        self.writer
            .write_unsigned_short(classfile.constant_pool_count)?;
        self.serialize_constant_pool(&classfile.constant_pool)?;

        self.writer.write_unsigned_short(classfile.access_flags)?;
        self.writer.write_unsigned_short(classfile.this_class)?;
        self.writer.write_unsigned_short(classfile.super_class)?;

        self.writer
            .write_unsigned_short(classfile.interfaces_count)?;
        for idx in 0..classfile.interfaces_count as usize {
            self.writer
                .write_unsigned_short(classfile.interfaces[idx])?;
        }

        // Fields
        self.writer.write_unsigned_short(classfile.fields_count)?;
        self.serialize_fields(&classfile.fields)?;

        // methods
        self.writer.write_unsigned_short(classfile.methods_count)?;
        self.serialize_methods(&classfile.methods)?;

        // class attributes
        self.writer
            .write_unsigned_short(classfile.attributes_count)?;
        self.serialize_attributes(&classfile.attributes)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // The byte buffer `buf` corresponds to this disassembled class file:
    //Classfile /Users/z0ltan/dev/playground/Minimal.class
    //  Last modified 27-Jan-2023; size 217 bytes
    //  SHA-256 checksum b590391a0a1f08067e66f237803225d0246d178484e0608543ea4fd12180dc2a
    //  Compiled from "Minimal.java"
    //public class Minimal
    //  minor version: 3
    //  major version: 45
    //  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
    //  this_class: #12                         // Minimal
    //  super_class: #13                        // java/lang/Object
    //  interfaces: 0, fields: 0, methods: 2, attributes: 1
    //Constant pool:
    //   #1 = Methodref          #13.#7         // java/lang/Object."<init>":()V
    //   #2 = Utf8               java/lang/Object
    //   #3 = Utf8               SourceFile
    //   #4 = Utf8               <init>
    //   #5 = Utf8               main
    //   #6 = Utf8               Minimal
    //   #7 = NameAndType        #4:#11         // "<init>":()V
    //   #8 = Utf8               Code
    //   #9 = Utf8               Minimal.java
    //  #10 = Utf8               ([Ljava/lang/String;)V
    //  #11 = Utf8               ()V
    //  #12 = Class              #6             // Minimal
    //  #13 = Class              #2             // java/lang/Object
    //{
    //  public Minimal();
    //    descriptor: ()V
    //    flags: (0x0001) ACC_PUBLIC
    //    Code:
    //      stack=1, locals=1, args_size=1
    //         0: aload_0
    //         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
    //         4: return
    //
    //  public static void main(java.lang.String[]);
    //    descriptor: ([Ljava/lang/String;)V
    //    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
    //    Code:
    //      stack=1, locals=1, args_size=1
    //         0: return
    //}
    //SourceFile: "Minimal.java"
    fn test_serialize_minimal() {
        use crate::model::{attributes::AttributeInfo::*, constant_pool::types::CpInfo::*};

        let expected_bytes = [
            0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x0f, 0x0a, 0x00, 0x02, 0x00,
            0x03, 0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61,
            0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74,
            0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29,
            0x56, 0x07, 0x00, 0x08, 0x01, 0x00, 0x07, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c,
            0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65,
            0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04,
            0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61,
            0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29,
            0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65,
            0x01, 0x00, 0x0c, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x6a, 0x61, 0x76,
            0x61, 0x00, 0x21, 0x00, 0x07, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x01, 0x00, 0x05, 0x00, 0x06, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x1d, 0x00,
            0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x09, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x19,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00,
            0x0d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x0e,
        ];

        let classfile = ClassFile {
            magic: 3405691582,
            minor_version: 0,
            major_version: 65,
            constant_pool_count: 15,
            constant_pool: vec![
                None,
                Some(ConstantMethodrefInfo {
                    tag: 10,
                    class_index: 2,
                    name_and_type_index: 3,
                }),
                Some(ConstantClassInfo {
                    tag: 7,
                    name_index: 4,
                }),
                Some(ConstantNameAndTypeInfo {
                    tag: 12,
                    name_index: 5,
                    descriptor_index: 6,
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 16,
                    bytes: vec![
                        106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 6,
                    bytes: vec![60, 105, 110, 105, 116, 62],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 3,
                    bytes: vec![40, 41, 86],
                }),
                Some(ConstantClassInfo {
                    tag: 7,
                    name_index: 8,
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 7,
                    bytes: vec![77, 105, 110, 105, 109, 97, 108],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 4,
                    bytes: vec![67, 111, 100, 101],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 15,
                    bytes: vec![
                        76, 105, 110, 101, 78, 117, 109, 98, 101, 114, 84, 97, 98, 108, 101,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 4,
                    bytes: vec![109, 97, 105, 110],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 22,
                    bytes: vec![
                        40, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                        110, 103, 59, 41, 86,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 10,
                    bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 12,
                    bytes: vec![77, 105, 110, 105, 109, 97, 108, 46, 106, 97, 118, 97],
                }),
            ],
            access_flags: 33,
            this_class: 7,
            super_class: 2,
            interfaces_count: 0,
            interfaces: vec![],
            fields_count: 0,
            fields: vec![],
            methods_count: 2,
            methods: vec![
                MethodInfo {
                    access_flags: 1,
                    name_index: 5,
                    descriptor_index: 6,
                    attributes_count: 1,
                    attributes: vec![Code {
                        attribute_name_index: 9,
                        attribute_length: 29,
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 5,
                        code: vec![42, 183, 0, 1, 177],
                        exception_table_length: 0,
                        exception_table: vec![],
                        code_attributes_count: 1,
                        code_attributes: vec![LineNumberTable {
                            attribute_name_index: 10,
                            attribute_length: 6,
                            line_number_table_length: 1,
                            line_number_table: vec![LineNumber {
                                start_pc: 0,
                                line_number: 1,
                            }],
                        }],
                    }],
                },
                MethodInfo {
                    access_flags: 9,
                    name_index: 11,
                    descriptor_index: 12,
                    attributes_count: 1,
                    attributes: vec![Code {
                        attribute_name_index: 9,
                        attribute_length: 25,
                        max_stack: 0,
                        max_locals: 1,
                        code_length: 1,
                        code: vec![177],
                        exception_table_length: 0,
                        exception_table: vec![],
                        code_attributes_count: 1,
                        code_attributes: vec![LineNumberTable {
                            attribute_name_index: 10,
                            attribute_length: 6,
                            line_number_table_length: 1,
                            line_number_table: vec![LineNumber {
                                start_pc: 0,
                                line_number: 2,
                            }],
                        }],
                    }],
                },
            ],
            attributes_count: 1,
            attributes: vec![SourceFile {
                attribute_name_index: 13,
                attribute_length: 2,
                sourcefile_index: 14,
            }],
        };

        let mut bytes: Vec<u8> = Vec::new();
        let mut serializer = Serializer::new(Writer::new(&mut bytes));
        serializer.serialize(&classfile).unwrap();
        assert_eq!(expected_bytes, &bytes[..]);
    }
}
