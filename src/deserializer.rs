//! Module to read a Java (JVM) class file and construct the object model from the raw bytes.

use crate::{
    error::DeserializeError,
    model::{
        attributes::*,
        constant_pool::{tags::*, types::CpInfo},
        ClassFile, FieldInfo, MethodInfo,
    },
    rw::reader::Reader,
};
use std::io::Read;

pub type DeserializeResult<T> = Result<T, DeserializeError>;

/// The Deserializer reads a class file byte stream and converts it into the
/// object model repreensting the class file.
pub struct Deserializer<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: Reader<R>) -> Self {
        Deserializer { reader }
    }

    fn deserialize_target_info(&mut self, target_type: u8) -> DeserializeResult<TargetInfo> {
        let target_info = match target_type {
            0x00 | 0x01 => {
                let type_parameter_index = self.reader.read_unsigned_byte()?;
                TargetInfo::TypeParameterTarget {
                    type_parameter_index,
                }
            }

            0x10 => {
                let supertype_index = self.reader.read_unsigned_short()?;
                TargetInfo::SuperTypeTarget { supertype_index }
            }

            0x11 | 0x12 => {
                let type_parameter_index = self.reader.read_unsigned_byte()?;
                let bound_index = self.reader.read_unsigned_byte()?;

                TargetInfo::TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                }
            }

            0x13 | 0x14 | 0x15 => TargetInfo::EmptyTarget,
            0x16 => {
                let formal_parameter_index = self.reader.read_unsigned_byte()?;
                TargetInfo::FormalParameterTarget {
                    formal_parameter_index,
                }
            }
            0x17 => {
                let throws_type_index = self.reader.read_unsigned_short()?;
                TargetInfo::ThrowsTarget { throws_type_index }
            }

            0x40 | 0x41 => {
                let table_length = self.reader.read_unsigned_short()?;

                let mut table = Vec::new();
                for _ in 0..table_length {
                    let start_pc = self.reader.read_unsigned_short()?;
                    let length = self.reader.read_unsigned_short()?;
                    let index = self.reader.read_unsigned_short()?;
                    table.push(LocalVarEntry {
                        start_pc,
                        length,
                        index,
                    });
                }
                TargetInfo::LocalVarTarget {
                    table_length,
                    table,
                }
            }

            0x42 => {
                let exception_table_index = self.reader.read_unsigned_short()?;
                TargetInfo::CatchTarget {
                    exception_table_index,
                }
            }
            043 | 0x44 | 0x45 | 0x46 => {
                let offset = self.reader.read_unsigned_short()?;
                TargetInfo::OffsetTarget { offset }
            }

            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
                let offset = self.reader.read_unsigned_short()?;
                let type_argument_index = self.reader.read_unsigned_byte()?;
                TargetInfo::TypeArgumentTarget {
                    offset,
                    type_argument_index,
                }
            }
            _ => unreachable!(),
        };

        Ok(target_info)
    }

    fn deserialize_type_path(&mut self) -> DeserializeResult<TypePath> {
        let path_length = self.reader.read_unsigned_byte()?;

        let mut path = Vec::new();
        for _ in 0..path_length {
            let type_path_kind = self.reader.read_unsigned_byte()?;
            let type_argument_index = self.reader.read_unsigned_byte()?;
            path.push(Path {
                type_path_kind,
                type_argument_index,
            });
        }

        Ok(TypePath { path_length, path })
    }

    fn deserialize_type_annotation(&mut self) -> DeserializeResult<TypeAnnotation> {
        let target_type = self.reader.read_unsigned_byte()?;
        let target_info = self.deserialize_target_info(target_type)?;
        let target_path = self.deserialize_type_path()?;
        let type_index = self.reader.read_unsigned_short()?;

        let num_element_value_pairs = self.reader.read_unsigned_short()?;
        let mut element_value_pairs = Vec::new();
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(self.deserialize_element_value_pair()?);
        }

        Ok(TypeAnnotation {
            target_type,
            target_info,
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn deserialize_element_value(&mut self) -> DeserializeResult<ElementValue> {
        let tag = self.reader.read_unsigned_byte()?;
        let element_value = match tag {
            // byte
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
                let const_value_index = self.reader.read_unsigned_short()?;
                ElementValue::ConstValueIndex {
                    tag,
                    const_value_index,
                }
            }
            b'e' => {
                let type_name_index = self.reader.read_unsigned_short()?;
                let const_name_index = self.reader.read_unsigned_short()?;
                ElementValue::EnumConstValue {
                    tag,
                    type_name_index,
                    const_name_index,
                }
            }

            b'c' => {
                let class_info_index = self.reader.read_unsigned_short()?;
                ElementValue::ClassInfoIndex {
                    tag,
                    class_info_index,
                }
            }

            b'@' => {
                let annotation = self.deserialize_annotation()?;
                ElementValue::AnnotationValue { tag, annotation }
            }

            b'[' => {
                let num_values = self.reader.read_unsigned_short()?;
                let mut values = Vec::new();
                for _ in 0..num_values {
                    values.push(self.deserialize_element_value()?);
                }

                ElementValue::ArrayValue {
                    tag,
                    num_values,
                    values,
                }
            }
            _ => unreachable!(),
        };

        Ok(element_value)
    }

    fn deserialize_element_value_pair(&mut self) -> DeserializeResult<ElementValuePair> {
        let element_name_index = self.reader.read_unsigned_short()?;
        let value = self.deserialize_element_value()?;

        Ok(ElementValuePair {
            element_name_index,
            value,
        })
    }

    fn deserialize_annotation(&mut self) -> DeserializeResult<Annotation> {
        let type_index = self.reader.read_unsigned_short()?;

        let num_element_value_pairs = self.reader.read_unsigned_short()?;
        let mut element_value_pairs = Vec::new();
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(self.deserialize_element_value_pair()?);
        }

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn deserialize_verification_type_info(&mut self) -> DeserializeResult<VerificationTypeInfo> {
        let tag = self.reader.read_unsigned_byte()?;
        let ver_type_info = match tag {
            0x00 => VerificationTypeInfo::TopVariableInfo { tag },
            0x01 => VerificationTypeInfo::IntegerVariableInfo { tag },
            0x02 => VerificationTypeInfo::FloatVariableInfo { tag },
            0x03 => VerificationTypeInfo::DoubleVariableInfo { tag },
            0x04 => VerificationTypeInfo::LongVariableInfo { tag },
            0x05 => VerificationTypeInfo::NullVariableInfo { tag },
            0x06 => VerificationTypeInfo::UninitializedThisVariableInfo { tag },
            0x07 => {
                let cpool_index = self.reader.read_unsigned_short()?;
                VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index }
            }
            0x08 => {
                let offset = self.reader.read_unsigned_short()?;
                VerificationTypeInfo::UninitializedVariableInfo { tag, offset }
            }
            _ => unreachable!(),
        };

        Ok(ver_type_info)
    }

    /// Deserialize the attributes of the class file.
    fn deserialize_attributes(
        &mut self,
        attributes_count: u16,
        constant_pool: &Vec<Option<CpInfo>>,
    ) -> DeserializeResult<Vec<AttributeInfo>> {
        let mut attributes = Vec::new();

        for _ in 0..attributes_count {
            let attribute_name_index = self.reader.read_unsigned_short()?;
            let attribute_length = self.reader.read_unsigned_int()?;

            match &constant_pool[attribute_name_index as usize] {
                Some(CpInfo::ConstantUtf8Info { bytes, .. }) => {
                    match String::from_utf8_lossy(bytes).into_owned().as_str() {
                        predefined_attributes::SOURCE_FILE => {
                            let sourcefile_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::SourceFile {
                                attribute_name_index,
                                attribute_length,
                                sourcefile_index,
                            });
                        }

                        predefined_attributes::CONSTANT_VALUE => {
                            let constantvalue_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::ConstantValue {
                                attribute_name_index,
                                attribute_length,
                                constantvalue_index,
                            });
                        }

                        predefined_attributes::CODE => {
                            let max_stack = self.reader.read_unsigned_short()?;
                            let max_locals = self.reader.read_unsigned_short()?;

                            let code_length = self.reader.read_unsigned_int()?;
                            assert!(code_length > 0);
                            let mut code = Vec::new();
                            for _ in 0..code_length {
                                code.push(self.reader.read_unsigned_byte()?);
                            }

                            let exception_table_length = self.reader.read_unsigned_short()?;
                            let mut exception_table = Vec::new();
                            for _ in 0..exception_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let end_pc = self.reader.read_unsigned_short()?;
                                let handler_pc = self.reader.read_unsigned_short()?;
                                let mut catch_type = self.reader.read_unsigned_short()?;

                                if catch_type != 0 {
                                    catch_type -= 1;
                                }

                                exception_table.push(ExceptionHandler {
                                    start_pc,
                                    end_pc,
                                    handler_pc,
                                    catch_type,
                                });
                            }

                            let code_attributes_count = self.reader.read_unsigned_short()?;
                            let code_attributes =
                                self.deserialize_attributes(code_attributes_count, constant_pool)?;
                            attributes.push(AttributeInfo::Code {
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
                            });
                        }

                        predefined_attributes::EXCEPTIONS => {
                            let number_of_exceptions = self.reader.read_unsigned_short()?;
                            let mut exception_index_table =
                                Vec::with_capacity(number_of_exceptions as usize);

                            for _ in 0..number_of_exceptions {
                                let mut idx = self.reader.read_unsigned_short()?;
                                if idx != 0 {
                                    idx -= 1;
                                }
                                exception_index_table.push(idx);
                            }
                            attributes.push(AttributeInfo::Exceptions {
                                attribute_name_index,
                                attribute_length,
                                number_of_exceptions,
                                exception_index_table,
                            });
                        }

                        predefined_attributes::LINE_NUMBER_TABLE => {
                            let line_number_table_length = self.reader.read_unsigned_short()?;
                            let mut line_number_table =
                                Vec::with_capacity(line_number_table_length as usize);

                            for _ in 0..line_number_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let line_number = self.reader.read_unsigned_short()?;
                                line_number_table.push(LineNumber {
                                    start_pc,
                                    line_number,
                                });
                            }

                            attributes.push(AttributeInfo::LineNumberTable {
                                attribute_name_index,
                                attribute_length,
                                line_number_table_length,
                                line_number_table,
                            });
                        }

                        predefined_attributes::LOCAL_VARIABLE_TABLE => {
                            let local_variable_table_length = self.reader.read_unsigned_short()?;
                            let mut local_variable_table =
                                Vec::with_capacity(local_variable_table_length as usize);

                            for _ in 0..local_variable_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let length = self.reader.read_unsigned_short()?;
                                let name_index = self.reader.read_unsigned_short()?;
                                let descriptor_index = self.reader.read_unsigned_short()?;
                                let index = self.reader.read_unsigned_short()?;

                                local_variable_table.push(LocalVariable {
                                    start_pc,
                                    length,
                                    name_index,
                                    descriptor_index,
                                    index,
                                });
                            }

                            attributes.push(AttributeInfo::LocalVariableTable {
                                attribute_name_index,
                                attribute_length,
                                local_variable_table_length,
                                local_variable_table,
                            });
                        }

                        predefined_attributes::STACK_MAP_TABLE => {
                            let number_of_entries = self.reader.read_unsigned_short()?;
                            let mut entries = Vec::new();
                            for _ in 0..number_of_entries {
                                let frame_type = self.reader.read_unsigned_byte()?;

                                let frame = match frame_type {
                                    // 0 - 63
                                    0x00..=0x3f => StackMapFrame::SameFrame { frame_type },
                                    // 64 - 127
                                    0x40..=0x7f => {
                                        let mut stack = Vec::with_capacity(1);
                                        stack.push(self.deserialize_verification_type_info()?);
                                        StackMapFrame::SameLocals1StackItemFrame {
                                            frame_type,
                                            stack,
                                        }
                                    }

                                    // 128 - 246 are reserved
                                    0x80..=0xf6 => unreachable!(),

                                    0xf7 => {
                                        let offset_delta = self.reader.read_unsigned_short()?;
                                        let mut stack = Vec::with_capacity(1);
                                        stack.push(self.deserialize_verification_type_info()?);

                                        StackMapFrame::SameLocals1StackItemFrameExtended {
                                            frame_type,
                                            offset_delta,
                                            stack,
                                        }
                                    }

                                    // 248 - 250
                                    0xf8..=0xfa => {
                                        let offset_delta = self.reader.read_unsigned_short()?;
                                        StackMapFrame::ChopFrame {
                                            frame_type,
                                            offset_delta,
                                        }
                                    }

                                    // 251
                                    0xfb => {
                                        let offset_delta = self.reader.read_unsigned_short()?;
                                        StackMapFrame::SameFrameExtended {
                                            frame_type,
                                            offset_delta,
                                        }
                                    }

                                    0xfc..=0xfe => {
                                        let offset_delta = self.reader.read_unsigned_short()?;
                                        let mut locals = Vec::with_capacity(1);
                                        locals.push(self.deserialize_verification_type_info()?);

                                        StackMapFrame::AppendFrame {
                                            frame_type,
                                            offset_delta,
                                            locals,
                                        }
                                    }

                                    // 255
                                    0xff => {
                                        let offset_delta = self.reader.read_unsigned_short()?;
                                        let number_of_locals = self.reader.read_unsigned_short()?;

                                        let mut locals =
                                            Vec::with_capacity(number_of_locals as usize);
                                        for _ in 0..number_of_locals {
                                            locals.push(self.deserialize_verification_type_info()?);
                                        }

                                        let number_of_stack_items =
                                            self.reader.read_unsigned_short()?;
                                        let mut stack =
                                            Vec::with_capacity(number_of_stack_items as usize);
                                        for _ in 0..number_of_stack_items {
                                            stack.push(self.deserialize_verification_type_info()?);
                                        }

                                        StackMapFrame::FullFrame {
                                            frame_type,
                                            offset_delta,
                                            number_of_locals,
                                            locals,
                                            number_of_stack_items,
                                            stack,
                                        }
                                    }
                                };
                                entries.push(frame);
                            }

                            attributes.push(AttributeInfo::StackMapTable {
                                attribute_name_index,
                                attribute_length,
                                number_of_entries,
                                entries,
                            });
                        }
                        predefined_attributes::INNER_CLASSES => {
                            let number_of_classes = self.reader.read_unsigned_short()?;

                            let mut classes = Vec::new();
                            for _ in 0..number_of_classes {
                                let inner_class_info_index = self.reader.read_unsigned_short()?;
                                let outer_class_info_index = self.reader.read_unsigned_short()?;
                                let inner_name_index = self.reader.read_unsigned_short()?;
                                let inner_class_access_flags = self.reader.read_unsigned_short()?;
                                classes.push(Class {
                                    inner_class_info_index,
                                    outer_class_info_index,
                                    inner_name_index,
                                    inner_class_access_flags,
                                });
                            }

                            attributes.push(AttributeInfo::InnerClasses {
                                attribute_name_index,
                                attribute_length,
                                number_of_classes,
                                classes,
                            });
                        }

                        predefined_attributes::ENCLOSING_METHOD => {
                            let class_index = self.reader.read_unsigned_short()?;
                            let method_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::EnclosingMethod {
                                attribute_name_index,
                                attribute_length,
                                class_index,
                                method_index,
                            });
                        }

                        predefined_attributes::SYNTHETIC => {
                            attributes.push(AttributeInfo::Synthetic {
                                attribute_name_index,
                                attribute_length,
                            });
                        }

                        predefined_attributes::SIGNATURE => {
                            let signature_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::Signature {
                                attribute_name_index,
                                attribute_length,
                                signature_index,
                            });
                        }

                        predefined_attributes::SOURCE_DEBUG_EXTENSION => {
                            let mut debug_extension = Vec::new();
                            for _ in 0..attribute_length {
                                debug_extension.push(self.reader.read_unsigned_byte()?);
                            }
                            attributes.push(AttributeInfo::SourceDebugExtension {
                                attribute_name_index,
                                attribute_length,
                                debug_extension,
                            });
                        }

                        predefined_attributes::LOCAL_VARIABLE_TYPE_TABLE => {
                            let local_variable_type_table_length =
                                self.reader.read_unsigned_short()?;

                            let mut local_variable_type_table = Vec::new();
                            for _ in 0..local_variable_type_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let length = self.reader.read_unsigned_short()?;
                                let name_index = self.reader.read_unsigned_short()?;
                                let signature_index = self.reader.read_unsigned_short()?;
                                let index = self.reader.read_unsigned_short()?;

                                local_variable_type_table.push(LocalVariableType {
                                    start_pc,
                                    length,
                                    name_index,
                                    signature_index,
                                    index,
                                });
                            }

                            attributes.push(AttributeInfo::LocalVariableTypeTable {
                                attribute_name_index,
                                attribute_length,
                                local_variable_type_table_length,
                                local_variable_type_table,
                            });
                        }

                        predefined_attributes::DEPRECATED => {
                            attributes.push(AttributeInfo::Deprecated {
                                attribute_name_index,
                                attribute_length,
                            });
                        }

                        predefined_attributes::RUNTIME_VISIBLE_ANNOTATIONS => {
                            let num_annotations = self.reader.read_unsigned_short()?;

                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.deserialize_annotation()?);
                            }

                            attributes.push(AttributeInfo::RuntimeVisibleAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            });
                        }

                        predefined_attributes::RUNTIME_INVISIBLE_ANNOTATIONS => {
                            let num_annotations = self.reader.read_unsigned_short()?;

                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.deserialize_annotation()?);
                            }

                            attributes.push(AttributeInfo::RuntimeInvisibleAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            });
                        }

                        predefined_attributes::RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => {
                            let num_parameters = self.reader.read_unsigned_byte()?;

                            let mut parameter_annotations = Vec::new();
                            for _ in 0..num_parameters {
                                let num_annotations = self.reader.read_unsigned_short()?;
                                let mut annotations = Vec::new();
                                for _ in 0..num_annotations {
                                    annotations.push(self.deserialize_annotation()?);
                                }
                                parameter_annotations.push(ParameterAnnotation {
                                    num_annotations,
                                    annotations,
                                });
                            }

                            attributes.push(AttributeInfo::RuntimeVisibleParameterAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_parameters,
                                parameter_annotations,
                            });
                        }

                        predefined_attributes::RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
                            let num_parameters = self.reader.read_unsigned_byte()?;

                            let mut parameter_annotations = Vec::new();
                            for _ in 0..num_parameters {
                                let num_annotations = self.reader.read_unsigned_short()?;
                                let mut annotations = Vec::new();
                                for _ in 0..num_annotations {
                                    annotations.push(self.deserialize_annotation()?);
                                }
                                parameter_annotations.push(ParameterAnnotation {
                                    num_annotations,
                                    annotations,
                                });
                            }

                            attributes.push(AttributeInfo::RuntimeInvisibleParameterAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_parameters,
                                parameter_annotations,
                            });
                        }

                        predefined_attributes::RUNTIME_VISIBLE_TYPE_ANNOTATIONS => {
                            let num_annotations = self.reader.read_unsigned_short()?;

                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.deserialize_type_annotation()?);
                            }

                            attributes.push(AttributeInfo::RuntimeVisibleTypeAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            })
                        }

                        predefined_attributes::RUNTIME_INVISIBLE_TYPE_ANNOTATIONS => {
                            let num_annotations = self.reader.read_unsigned_short()?;

                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.deserialize_type_annotation()?);
                            }

                            attributes.push(AttributeInfo::RuntimeInvisibleTypeAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            })
                        }

                        predefined_attributes::ANNOTATION_DEFAULT => {
                            let default_value = self.deserialize_element_value()?;
                            attributes.push(AttributeInfo::AnnotationDefault {
                                attribute_name_index,
                                attribute_length,
                                default_value,
                            });
                        }

                        predefined_attributes::BOOTSTRAP_METHODS => {
                            let num_bootstrap_methods = self.reader.read_unsigned_short()?;

                            let mut bootstrap_methods = Vec::new();
                            for _ in 0..num_bootstrap_methods {
                                let bootstrap_method_ref = self.reader.read_unsigned_short()?;

                                let num_bootstrap_arguments = self.reader.read_unsigned_short()?;
                                let mut bootstrap_arguments = Vec::new();
                                for _ in 0..num_bootstrap_arguments {
                                    bootstrap_arguments.push(self.reader.read_unsigned_short()?);
                                }

                                bootstrap_methods.push(BootstrapMethod {
                                    bootstrap_method_ref,
                                    num_bootstrap_arguments,
                                    bootstrap_arguments,
                                });
                            }

                            attributes.push(AttributeInfo::BootstrapMethods {
                                attribute_name_index,
                                attribute_length,
                                num_bootstrap_methods,
                                bootstrap_methods,
                            });
                        }

                        predefined_attributes::METHOD_PARAMETERS => {
                            let parameters_count = self.reader.read_unsigned_byte()?;

                            let mut parameters = Vec::new();
                            for _ in 0..parameters_count {
                                let name_index = self.reader.read_unsigned_short()?;
                                let access_flags = self.reader.read_unsigned_short()?;
                                parameters.push(Parameter {
                                    name_index,
                                    access_flags,
                                });
                            }

                            attributes.push(AttributeInfo::MethodParameters {
                                attribute_name_index,
                                attribute_length,
                                parameters_count,
                                parameters,
                            });
                        }

                        predefined_attributes::MODULE => {
                            let module_name_index = self.reader.read_unsigned_short()?;
                            let module_flags = self.reader.read_unsigned_short()?;
                            let module_version_index = self.reader.read_unsigned_short()?;

                            let requires_count = self.reader.read_unsigned_short()?;
                            let mut requires = Vec::new();
                            for _ in 0..requires_count {
                                let requires_index = self.reader.read_unsigned_short()?;
                                let requires_flags = self.reader.read_unsigned_short()?;
                                let requires_version_index = self.reader.read_unsigned_short()?;

                                requires.push(Require {
                                    requires_index,
                                    requires_flags,
                                    requires_version_index,
                                });
                            }

                            let exports_count = self.reader.read_unsigned_short()?;
                            let mut exports = Vec::new();
                            for _ in 0..exports_count {
                                let exports_index = self.reader.read_unsigned_short()?;
                                let exports_flags = self.reader.read_unsigned_short()?;

                                let exports_to_count = self.reader.read_unsigned_short()?;
                                let mut exports_to_index = Vec::new();
                                for _ in 0..exports_to_count {
                                    exports_to_index.push(self.reader.read_unsigned_short()?);
                                }

                                exports.push(Export {
                                    exports_index,
                                    exports_flags,
                                    exports_to_count,
                                    exports_to_index,
                                });
                            }

                            let opens_count = self.reader.read_unsigned_short()?;
                            let mut opens = Vec::new();
                            for _ in 0..opens_count {
                                let opens_index = self.reader.read_unsigned_short()?;
                                let opens_flags = self.reader.read_unsigned_short()?;

                                let opens_to_count = self.reader.read_unsigned_short()?;
                                let mut opens_to_index = Vec::new();
                                for _ in 0..opens_to_count {
                                    opens_to_index.push(self.reader.read_unsigned_short()?);
                                }

                                opens.push(Open {
                                    opens_index,
                                    opens_flags,
                                    opens_to_count,
                                    opens_to_index,
                                });
                            }

                            let uses_count = self.reader.read_unsigned_short()?;
                            let mut uses_index = Vec::new();
                            for _ in 0..uses_count {
                                uses_index.push(self.reader.read_unsigned_short()?);
                            }

                            let provides_count = self.reader.read_unsigned_short()?;
                            let mut provides = Vec::new();
                            for _ in 0..provides_count {
                                let provides_index = self.reader.read_unsigned_short()?;

                                let provides_with_count = self.reader.read_unsigned_short()?;
                                let mut provides_with_index = Vec::new();
                                for _ in 0..provides_with_count {
                                    provides_with_index.push(self.reader.read_unsigned_short()?);
                                }

                                provides.push(Provide {
                                    provides_index,
                                    provides_with_count,
                                    provides_with_index,
                                });
                            }
                            attributes.push(AttributeInfo::Module {
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
                            })
                        }

                        predefined_attributes::MODULE_PACKAGES => {
                            let package_count = self.reader.read_unsigned_short()?;
                            let mut package_index = Vec::new();
                            for _ in 0..package_count {
                                package_index.push(self.reader.read_unsigned_short()?);
                            }

                            attributes.push(AttributeInfo::ModulePackages {
                                attribute_name_index,
                                attribute_length,
                                package_count,
                                package_index,
                            });
                        }

                        predefined_attributes::MODULE_MAIN_CLASS => {
                            let main_class_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::ModuleMainClass {
                                attribute_name_index,
                                attribute_length,
                                main_class_index,
                            });
                        }

                        predefined_attributes::NEST_HOST => {
                            let host_class_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::NestHost {
                                attribute_name_index,
                                attribute_length,
                                host_class_index,
                            });
                        }

                        predefined_attributes::NEST_MEMBERS => {
                            let number_of_classes = self.reader.read_unsigned_short()?;
                            let mut classes = Vec::new();
                            for _ in 0..number_of_classes {
                                classes.push(self.reader.read_unsigned_short()?);
                            }
                            attributes.push(AttributeInfo::NestMembers {
                                attribute_name_index,
                                attribute_length,
                                number_of_classes,
                                classes,
                            });
                        }

                        predefined_attributes::RECORD => {
                            let components_count = self.reader.read_unsigned_short()?;
                            let mut components = Vec::new();
                            for _ in 0..components_count {
                                let name_index = self.reader.read_unsigned_short()?;
                                let descriptor_index = self.reader.read_unsigned_short()?;

                                let attributes_count = self.reader.read_unsigned_short()?;
                                let attributes =
                                    self.deserialize_attributes(attributes_count, constant_pool)?;

                                components.push(RecordComponentInfo {
                                    name_index,
                                    descriptor_index,
                                    attributes_count,
                                    attributes,
                                });
                            }

                            attributes.push(AttributeInfo::Record {
                                attribute_name_index,
                                attribute_length,
                                components_count,
                                components,
                            });
                        }

                        predefined_attributes::PERMITTED_SUBCLASSES => {
                            let number_of_classes = self.reader.read_unsigned_short()?;
                            let mut classes = Vec::new();
                            for _ in 0..number_of_classes {
                                classes.push(self.reader.read_unsigned_short()?);
                            }

                            attributes.push(AttributeInfo::PermittedSubclasses {
                                attribute_name_index,
                                attribute_length,
                                number_of_classes,
                                classes,
                            });
                        }

                        _ => {
                            // simply read the bytes and discard for any unknown attributes
                            // for now - look into providing a pluggable way to store and
                            // query for custom attributes.
                            for _ in 0..attribute_length {
                                let _ = self.reader.read_unsigned_byte()?;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(attributes)
    }

    /// Deserialize the fields of the class file.
    fn deserialize_fields(
        &mut self,
        fields_count: u16,
        constant_pool: &Vec<Option<CpInfo>>,
    ) -> DeserializeResult<Vec<FieldInfo>> {
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            let access_flags = self.reader.read_unsigned_short()?;
            let name_index = self.reader.read_unsigned_short()?;
            let descriptor_index = self.reader.read_unsigned_short()?;
            let attributes_count = self.reader.read_unsigned_short()?;

            let attributes = self.deserialize_attributes(attributes_count, constant_pool)?;
            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        Ok(fields)
    }

    /// Deserialize the methods of the class file.
    fn deserialize_methods(
        &mut self,
        methods_count: u16,
        constant_pool: &Vec<Option<CpInfo>>,
    ) -> DeserializeResult<Vec<MethodInfo>> {
        let mut methods = Vec::new();

        for _ in 0..methods_count {
            let access_flags = self.reader.read_unsigned_short()?;
            let name_index = self.reader.read_unsigned_short()?;
            let descriptor_index = self.reader.read_unsigned_short()?;
            let attributes_count = self.reader.read_unsigned_short()?;

            let attributes = self.deserialize_attributes(attributes_count, constant_pool)?;
            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        Ok(methods)
    }

    /// Deserialize the contents of the Constant Pool.
    fn deserialize_constant_pool(
        &mut self,
        constant_pool_count: u16,
    ) -> DeserializeResult<Vec<Option<CpInfo>>> {
        let mut constant_pool = vec![None; constant_pool_count as usize];
        let mut cp_idx = 1usize; // the first cell is not used

        while cp_idx < constant_pool_count as usize {
            let tag = self.reader.read_unsigned_byte()?;

            match tag {
                CONSTANT_METHOD_REF => {
                    let class_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_CLASS => {
                    let name_index = self.reader.read_unsigned_short()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantClassInfo { tag, name_index });
                }

                CONSTANT_FIELD_REF => {
                    let class_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantFieldrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_INTERFACE_METHOD_REF => {
                    let class_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_STRING => {
                    let string_index = self.reader.read_unsigned_short()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantStringInfo { tag, string_index });
                }

                CONSTANT_INTEGER => {
                    let bytes = self.reader.read_unsigned_int()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantIntegerInfo { tag, bytes });
                }

                CONSTANT_FLOAT => {
                    let bytes = self.reader.read_unsigned_int()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantFloatInfo { tag, bytes });
                }

                CONSTANT_LONG => {
                    let high_bytes = self.reader.read_unsigned_int()?;
                    let low_bytes = self.reader.read_unsigned_int()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantLongInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    });

                    // Long values take up two consecutive entries in the Constant Pool.
                    cp_idx += 1;
                }

                CONSTANT_DOUBLE => {
                    let high_bytes = self.reader.read_unsigned_int()?;
                    let low_bytes = self.reader.read_unsigned_int()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantDoubleInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    });

                    // Double values take up two consuective entries in the Constant Pool.
                    cp_idx += 1;
                }

                CONSTANT_NAME_AND_TYPE => {
                    let name_index = self.reader.read_unsigned_short()?;
                    let descriptor_index = self.reader.read_unsigned_short()?;
                    constant_pool[cp_idx] = Some(CpInfo::ConstantNameAndTypeInfo {
                        tag,
                        name_index,
                        descriptor_index,
                    });
                }

                CONSTANT_UTF8 => {
                    let length = self.reader.read_unsigned_short()?;
                    let mut bytes = Vec::new();
                    for _ in 0..length {
                        bytes.push(self.reader.read_unsigned_byte()?);
                    }

                    constant_pool[cp_idx] = Some(CpInfo::ConstantUtf8Info { tag, length, bytes });
                }

                CONSTANT_METHOD_HANDLE => {
                    let reference_kind = self.reader.read_unsigned_byte()?;
                    let reference_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(Some(CpInfo::ConstantMethodHandleInfo {
                        tag,
                        reference_kind,
                        reference_index,
                    }));
                }

                CONSTANT_METHOD_TYPE => {
                    let descriptor_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(Some(CpInfo::ConstantMethodTypeInfo {
                        tag,
                        descriptor_index,
                    }));
                }

                CONSTANT_DYNAMIC => {
                    let bootstrap_method_attr_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(Some(CpInfo::ConstantDynamicInfo {
                        tag,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }));
                }

                CONSTANT_INVOKE_DYNAMIC => {
                    let bootstrap_method_attr_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(Some(CpInfo::ConstantInvokeDynamicInfo {
                        tag,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }));
                }

                CONSTANT_MODULE => {
                    let name_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(Some(CpInfo::ConstantModuleInfo { tag, name_index }));
                }

                CONSTANT_PACKAGE => {
                    let name_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(Some(CpInfo::ConstantPackageInfo { tag, name_index }));
                }

                _ => unreachable!(),
            }
            cp_idx += 1;
        }

        Ok(constant_pool)
    }

    /// Deserialize the class file into the object model.
    pub fn deserialize(&mut self) -> DeserializeResult<ClassFile> {
        // Headers
        let magic = self.reader.read_unsigned_int()?;
        let minor_version = self.reader.read_unsigned_short()?;
        let major_version = self.reader.read_unsigned_short()?;

        // Constant Pool
        let constant_pool_count = self.reader.read_unsigned_short()?;
        assert!(constant_pool_count > 0);
        let constant_pool = self.deserialize_constant_pool(constant_pool_count)?;

        let access_flags = self.reader.read_unsigned_short()?;
        let this_class = self.reader.read_unsigned_short()?;

        let mut super_class = self.reader.read_unsigned_short()?;
        // if super_class == 0 then this is ``java.lang.Object``
        if super_class > 0 {
            super_class -= 1;
        }

        let interfaces_count = self.reader.read_unsigned_short()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(self.reader.read_unsigned_short()?);
        }

        // Fields
        let fields_count = self.reader.read_unsigned_short()?;
        let fields = self.deserialize_fields(fields_count, &constant_pool)?;

        // methods
        let methods_count = self.reader.read_unsigned_short()?;
        let methods = self.deserialize_methods(methods_count, &constant_pool)?;

        // class attributes
        let attributes_count = self.reader.read_unsigned_short()?;
        let attributes = self.deserialize_attributes(attributes_count, &constant_pool)?;

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        })
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
    fn test_deserialize_minimal() {
        use std::io::Cursor;

        let bytes = [
            0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x0e, 0x0a, 0x00, 0x0d, 0x00,
            0x07, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
            0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63,
            0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e,
            0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x07, 0x4d, 0x69, 0x6e, 0x69,
            0x6d, 0x61, 0x6c, 0x0c, 0x00, 0x04, 0x00, 0x0b, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64,
            0x65, 0x01, 0x00, 0x0c, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x6a, 0x61,
            0x76, 0x61, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
            0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01,
            0x00, 0x03, 0x28, 0x29, 0x56, 0x07, 0x00, 0x06, 0x07, 0x00, 0x02, 0x00, 0x21, 0x00,
            0x0c, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x04, 0x00,
            0x0b, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
            0x00, 0x05, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x01,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x09,
        ];

        let mut deserializer = Deserializer::new(Reader::new(Cursor::new(bytes)));
        let _classfile = deserializer.deserialize().unwrap();
    }
}
