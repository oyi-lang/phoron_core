use phoron_core::{
    model::{
        attributes::{AttributeInfo::*, StackMapFrame::*, VerificationTypeInfo::*, *},
        constant_pool::types::CpInfo::*,
        *,
    },
    rw::writer::Writer,
    serializer::Serializer,
};

use std::error::Error;

pub type SerializerResult = Result<(), Box<dyn Error + Send + Sync + 'static>>;

// Bytecode for the following class file:
//Classfile /Users/z0ltan/dev/playground/HelloWorld.class
//  Last modified 27-Jan-2023; size 422 bytes
//  SHA-256 checksum 8b07d9dd65152998eda6951af14be9052f0dd66d8c60bbf1be42530fefe2e056
//  Compiled from "HelloWorld.java"
//public class HelloWorld
//  minor version: 0
//  major version: 65
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #21                         // HelloWorld
//  super_class: #2                         // java/lang/Object
//  interfaces: 0, fields: 0, methods: 2, attributes: 1
//Constant pool:
//   #1 = Methodref          #2.#3          // java/lang/Object."<init>":()V
//   #2 = Class              #4             // java/lang/Object
//   #3 = NameAndType        #5:#6          // "<init>":()V
//   #4 = Utf8               java/lang/Object
//   #5 = Utf8               <init>
//   #6 = Utf8               ()V
//   #7 = Fieldref           #8.#9          // java/lang/System.out:Ljava/io/PrintStream;
//   #8 = Class              #10            // java/lang/System
//   #9 = NameAndType        #11:#12        // out:Ljava/io/PrintStream;
//  #10 = Utf8               java/lang/System
//  #11 = Utf8               out
//  #12 = Utf8               Ljava/io/PrintStream;
//  #13 = String             #14            // Hello, world
//  #14 = Utf8               Hello, world
//  #15 = Methodref          #16.#17        // java/io/PrintStream.println:(Ljava/lang/String;)V
//  #16 = Class              #18            // java/io/PrintStream
//  #17 = NameAndType        #19:#20        // println:(Ljava/lang/String;)V
//  #18 = Utf8               java/io/PrintStream
//  #19 = Utf8               println
//  #20 = Utf8               (Ljava/lang/String;)V
//  #21 = Class              #22            // HelloWorld
//  #22 = Utf8               HelloWorld
//  #23 = Utf8               Code
//  #24 = Utf8               LineNumberTable
//  #25 = Utf8               main
//  #26 = Utf8               ([Ljava/lang/String;)V
//  #27 = Utf8               SourceFile
//  #28 = Utf8               HelloWorld.java
//{
//  public HelloWorld();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
//         4: return
//      LineNumberTable:
//        line 1: 0
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=2, locals=1, args_size=1
//         0: getstatic     #7                  // Field java/lang/System.out:Ljava/io/PrintStream;
//         3: ldc           #13                 // String Hello, world
//         5: invokevirtual #15                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
//         8: return
//      LineNumberTable:
//        line 2: 0
//}
//SourceFile: "HelloWorld.java"
#[test]
fn test_serialize_hello_world() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x1d, 0x0a, 0x00, 0x02, 0x00, 0x03,
        0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06,
        0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x09, 0x00, 0x08,
        0x00, 0x09, 0x07, 0x00, 0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x10, 0x6a, 0x61,
        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x01,
        0x00, 0x03, 0x6f, 0x75, 0x74, 0x01, 0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69,
        0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x08,
        0x00, 0x0e, 0x01, 0x00, 0x0c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72,
        0x6c, 0x64, 0x0a, 0x00, 0x10, 0x00, 0x11, 0x07, 0x00, 0x12, 0x0c, 0x00, 0x13, 0x00, 0x14,
        0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e,
        0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x07, 0x70, 0x72, 0x69, 0x6e, 0x74,
        0x6c, 0x6e, 0x01, 0x00, 0x15, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e,
        0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x07, 0x00, 0x16, 0x01,
        0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x01, 0x00, 0x04,
        0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62,
        0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01,
        0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
        0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75,
        0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x0f, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x6a, 0x61, 0x76, 0x61, 0x00, 0x21, 0x00, 0x15, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x06, 0x00, 0x01,
        0x00, 0x17, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a,
        0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x09, 0x00, 0x19, 0x00, 0x1a, 0x00, 0x01, 0x00, 0x17,
        0x00, 0x00, 0x00, 0x21, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0xb2, 0x00, 0x07,
        0x12, 0x0d, 0xb6, 0x00, 0x0f, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x1c,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 0,
        major_version: 65,
        constant_pool_count: 29,
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
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 9,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 10,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 11,
                descriptor_index: 12,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 14,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 12,
                bytes: vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 16,
                name_and_type_index: 17,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 18,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 19,
                descriptor_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 41, 86,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 22,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![72, 101, 108, 108, 111, 87, 111, 114, 108, 100],
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
                length: 15,
                bytes: vec![
                    72, 101, 108, 108, 111, 87, 111, 114, 108, 100, 46, 106, 97, 118, 97,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 21,
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
                    attribute_name_index: 23,
                    attribute_length: 29,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 1, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 24,
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
                name_index: 25,
                descriptor_index: 26,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 23,
                    attribute_length: 33,
                    max_stack: 2,
                    max_locals: 1,
                    code_length: 9,
                    code: vec![178, 0, 7, 18, 13, 182, 0, 15, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 24,
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
            attribute_name_index: 27,
            attribute_length: 2,
            sourcefile_index: 28,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dv/playground/Fields.class
//  Last modified 27-Jan-2023; size 1023 bytes
//  SHA-256 checksum 65434f38c6bb13a5bf08b4226f80394cfaba5cc5dcbb7cacd3145cb3336f49f2
//  Compiled from "Fields.java"
//public class Fields
//  minor version: 0
//  major version: 65
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #8                          // Fields
//  super_class: #2                         // java/lang/Object
//  interfaces: 0, fields: 5, methods: 2, attributes: 1
//Constant pool:
//   #1 = Methodref          #2.#3          // java/lang/Object."<init>":()V
//   #2 = Class              #4             // java/lang/Object
//   #3 = NameAndType        #5:#6          // "<init>":()V
//   #4 = Utf8               java/lang/Object
//   #5 = Utf8               <init>
//   #6 = Utf8               ()V
//   #7 = Fieldref           #8.#9          // Fields.one:I
//   #8 = Class              #10            // Fields
//   #9 = NameAndType        #11:#12        // one:I
//  #10 = Utf8               Fields
//  #11 = Utf8               one
//  #12 = Utf8               I
//  #13 = Fieldref           #8.#14         // Fields.two:Ljava/lang/String;
//  #14 = NameAndType        #15:#16        // two:Ljava/lang/String;
//  #15 = Utf8               two
//  #16 = Utf8               Ljava/lang/String;
//  #17 = Fieldref           #8.#18         // Fields.three:D
//  #18 = NameAndType        #19:#20        // three:D
//  #19 = Utf8               three
//  #20 = Utf8               D
//  #21 = Fieldref           #8.#22         // Fields.four:Z
//  #22 = NameAndType        #23:#24        // four:Z
//  #23 = Utf8               four
//  #24 = Utf8               Z
//  #25 = Fieldref           #8.#26         // Fields.five:Ljava/lang/Integer;
//  #26 = NameAndType        #27:#28        // five:Ljava/lang/Integer;
//  #27 = Utf8               five
//  #28 = Utf8               Ljava/lang/Integer;
//  #29 = String             #15            // two
//  #30 = Double             3.0d
//  #32 = Methodref          #33.#34        // java/lang/Integer.valueOf:(I)Ljava/lang/Integer;
//  #33 = Class              #35            // java/lang/Integer
//  #34 = NameAndType        #36:#37        // valueOf:(I)Ljava/lang/Integer;
//  #35 = Utf8               java/lang/Integer
//  #36 = Utf8               valueOf
//  #37 = Utf8               (I)Ljava/lang/Integer;
//  #38 = Methodref          #8.#39         // Fields."<init>":(ILjava/lang/String;DZLjava/lang/Integer;)V
//  #39 = NameAndType        #5:#40         // "<init>":(ILjava/lang/String;DZLjava/lang/Integer;)V
//  #40 = Utf8               (ILjava/lang/String;DZLjava/lang/Integer;)V
//  #41 = Fieldref           #42.#43        // java/lang/System.out:Ljava/io/PrintStream;
//  #42 = Class              #44            // java/lang/System
//  #43 = NameAndType        #45:#46        // out:Ljava/io/PrintStream;
//  #44 = Utf8               java/lang/System
//  #45 = Utf8               out
//  #46 = Utf8               Ljava/io/PrintStream;
//  #47 = String             #48            // %d, %s, %f, %b, %d\n
//  #48 = Utf8               %d, %s, %f, %b, %d\n
//  #49 = Methodref          #50.#51        // java/lang/Double.valueOf:(D)Ljava/lang/Double;
//  #50 = Class              #52            // java/lang/Double
//  #51 = NameAndType        #36:#53        // valueOf:(D)Ljava/lang/Double;
//  #52 = Utf8               java/lang/Double
//  #53 = Utf8               (D)Ljava/lang/Double;
//  #54 = Methodref          #55.#56        // java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;
//  #55 = Class              #57            // java/lang/Boolean
//  #56 = NameAndType        #36:#58        // valueOf:(Z)Ljava/lang/Boolean;
//  #57 = Utf8               java/lang/Boolean
//  #58 = Utf8               (Z)Ljava/lang/Boolean;
//  #59 = Methodref          #60.#61        // java/io/PrintStream.printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//  #60 = Class              #62            // java/io/PrintStream
//  #61 = NameAndType        #63:#64        // printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//  #62 = Utf8               java/io/PrintStream
//  #63 = Utf8               printf
//  #64 = Utf8               (Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//  #65 = Utf8               Code
//  #66 = Utf8               LineNumberTable
//  #67 = Utf8               main
//  #68 = Utf8               ([Ljava/lang/String;)V
//  #69 = Utf8               SourceFile
//  #70 = Utf8               Fields.java
//{
//  public double three;
//    descriptor: D
//    flags: (0x0001) ACC_PUBLIC
//
//  protected boolean four;
//    descriptor: Z
//    flags: (0x0004) ACC_PROTECTED
//
//  java.lang.Integer five;
//    descriptor: Ljava/lang/Integer;
//    flags: (0x0000)
//
//  Fields(int, java.lang.String, double, boolean, java.lang.Integer);
//    descriptor: (ILjava/lang/String;DZLjava/lang/Integer;)V
//    flags: (0x0000)
//    Code:
//      stack=3, locals=7, args_size=6
//         0: aload_0
//         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
//         4: aload_0
//         5: iload_1
//         6: putfield      #7                  // Field one:I
//         9: aload_0
//        10: aload_2
//        11: putfield      #13                 // Field two:Ljava/lang/String;
//        14: aload_0
//        15: dload_3
//        16: putfield      #17                 // Field three:D
//        19: aload_0
//        20: iload         5
//        22: putfield      #21                 // Field four:Z
//        25: aload_0
//        26: aload         6
//        28: putfield      #25                 // Field five:Ljava/lang/Integer;
//        31: return
//      LineNumberTable:
//        line 8: 0
//        line 9: 4
//        line 10: 9
//        line 11: 14
//        line 12: 19
//        line 13: 25
//        line 14: 31
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=8, locals=2, args_size=1
//         0: new           #8                  // class Fields
//         3: dup
//         4: iconst_1
//         5: ldc           #29                 // String two
//         7: ldc2_w        #30                 // double 3.0d
//        10: iconst_1
//        11: iconst_5
//        12: invokestatic  #32                 // Method java/lang/Integer.valueOf:(I)Ljava/lang/Integer;
//        15: invokespecial #38                 // Method "<init>":(ILjava/lang/String;DZLjava/lang/Integer;)V
//        18: astore_1
//        19: getstatic     #41                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        22: ldc           #47                 // String %d, %s, %f, %b, %d\n
//        24: iconst_5
//        25: anewarray     #2                  // class java/lang/Object
//        28: dup
//        29: iconst_0
//        30: aload_1
//        31: getfield      #7                  // Field one:I
//        34: invokestatic  #32                 // Method java/lang/Integer.valueOf:(I)Ljava/lang/Integer;
//        37: aastore
//        38: dup
//        39: iconst_1
//        40: aload_1
//        41: getfield      #13                 // Field two:Ljava/lang/String;
//        44: aastore
//        45: dup
//        46: iconst_2
//        47: aload_1
//        48: getfield      #17                 // Field three:D
//        51: invokestatic  #49                 // Method java/lang/Double.valueOf:(D)Ljava/lang/Double;
//        54: aastore
//        55: dup
//        56: iconst_3
//        57: aload_1
//        58: getfield      #21                 // Field four:Z
//        61: invokestatic  #54                 // Method java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;
//        64: aastore
//        65: dup
//        66: iconst_4
//        67: aload_1
//        68: getfield      #25                 // Field five:Ljava/lang/Integer;
//        71: aastore
//        72: invokevirtual #59                 // Method java/io/PrintStream.printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//        75: pop
//        76: return
//      LineNumberTable:
//        line 17: 0
//        line 18: 19
//        line 19: 51
//        line 18: 72
//        line 20: 76
//}
//SourceFile: "Fields.java"
#[test]
fn test_serialize_fields() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x47, 0x0a, 0x00, 0x02, 0x00, 0x03,
        0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06,
        0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x09, 0x00, 0x08,
        0x00, 0x09, 0x07, 0x00, 0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x06, 0x46, 0x69,
        0x65, 0x6c, 0x64, 0x73, 0x01, 0x00, 0x03, 0x6f, 0x6e, 0x65, 0x01, 0x00, 0x01, 0x49, 0x09,
        0x00, 0x08, 0x00, 0x0e, 0x0c, 0x00, 0x0f, 0x00, 0x10, 0x01, 0x00, 0x03, 0x74, 0x77, 0x6f,
        0x01, 0x00, 0x12, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53,
        0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x09, 0x00, 0x08, 0x00, 0x12, 0x0c, 0x00, 0x13, 0x00,
        0x14, 0x01, 0x00, 0x05, 0x74, 0x68, 0x72, 0x65, 0x65, 0x01, 0x00, 0x01, 0x44, 0x09, 0x00,
        0x08, 0x00, 0x16, 0x0c, 0x00, 0x17, 0x00, 0x18, 0x01, 0x00, 0x04, 0x66, 0x6f, 0x75, 0x72,
        0x01, 0x00, 0x01, 0x5a, 0x09, 0x00, 0x08, 0x00, 0x1a, 0x0c, 0x00, 0x1b, 0x00, 0x1c, 0x01,
        0x00, 0x04, 0x66, 0x69, 0x76, 0x65, 0x01, 0x00, 0x13, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f,
        0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x3b, 0x08, 0x00,
        0x0f, 0x06, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x21, 0x00, 0x22,
        0x07, 0x00, 0x23, 0x0c, 0x00, 0x24, 0x00, 0x25, 0x01, 0x00, 0x11, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x01, 0x00,
        0x07, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x4f, 0x66, 0x01, 0x00, 0x16, 0x28, 0x49, 0x29, 0x4c,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67,
        0x65, 0x72, 0x3b, 0x0a, 0x00, 0x08, 0x00, 0x27, 0x0c, 0x00, 0x05, 0x00, 0x28, 0x01, 0x00,
        0x2b, 0x28, 0x49, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53,
        0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x44, 0x5a, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x3b, 0x29, 0x56, 0x09,
        0x00, 0x2a, 0x00, 0x2b, 0x07, 0x00, 0x2c, 0x0c, 0x00, 0x2d, 0x00, 0x2e, 0x01, 0x00, 0x10,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65,
        0x6d, 0x01, 0x00, 0x03, 0x6f, 0x75, 0x74, 0x01, 0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
        0x3b, 0x08, 0x00, 0x30, 0x01, 0x00, 0x13, 0x25, 0x64, 0x2c, 0x20, 0x25, 0x73, 0x2c, 0x20,
        0x25, 0x66, 0x2c, 0x20, 0x25, 0x62, 0x2c, 0x20, 0x25, 0x64, 0x0a, 0x0a, 0x00, 0x32, 0x00,
        0x33, 0x07, 0x00, 0x34, 0x0c, 0x00, 0x24, 0x00, 0x35, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76,
        0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x01, 0x00,
        0x15, 0x28, 0x44, 0x29, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
        0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x3b, 0x0a, 0x00, 0x37, 0x00, 0x38, 0x07, 0x00, 0x39,
        0x0c, 0x00, 0x24, 0x00, 0x3a, 0x01, 0x00, 0x11, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61,
        0x6e, 0x67, 0x2f, 0x42, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5a,
        0x29, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x42, 0x6f, 0x6f,
        0x6c, 0x65, 0x61, 0x6e, 0x3b, 0x0a, 0x00, 0x3c, 0x00, 0x3d, 0x07, 0x00, 0x3e, 0x0c, 0x00,
        0x3f, 0x00, 0x40, 0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50,
        0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x06, 0x70, 0x72,
        0x69, 0x6e, 0x74, 0x66, 0x01, 0x00, 0x3c, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x5b, 0x4c, 0x6a, 0x61,
        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x3b,
        0x29, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74,
        0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01,
        0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62,
        0x6c, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e,
        0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69,
        0x6c, 0x65, 0x01, 0x00, 0x0b, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x2e, 0x6a, 0x61, 0x76,
        0x61, 0x00, 0x21, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x0b,
        0x00, 0x0c, 0x00, 0x00, 0x00, 0x02, 0x00, 0x0f, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x13, 0x00, 0x14, 0x00, 0x00, 0x00, 0x04, 0x00, 0x17, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x1b, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x28, 0x00,
        0x01, 0x00, 0x41, 0x00, 0x00, 0x00, 0x50, 0x00, 0x03, 0x00, 0x07, 0x00, 0x00, 0x00, 0x20,
        0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x1b, 0xb5, 0x00, 0x07, 0x2a, 0x2c, 0xb5, 0x00, 0x0d, 0x2a,
        0x29, 0xb5, 0x00, 0x11, 0x2a, 0x15, 0x05, 0xb5, 0x00, 0x15, 0x2a, 0x19, 0x06, 0xb5, 0x00,
        0x19, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x42, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x08, 0x00, 0x04, 0x00, 0x09, 0x00, 0x09, 0x00, 0x0a, 0x00, 0x0e, 0x00, 0x0b,
        0x00, 0x13, 0x00, 0x0c, 0x00, 0x19, 0x00, 0x0d, 0x00, 0x1f, 0x00, 0x0e, 0x00, 0x09, 0x00,
        0x43, 0x00, 0x44, 0x00, 0x01, 0x00, 0x41, 0x00, 0x00, 0x00, 0x75, 0x00, 0x08, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x4d, 0xbb, 0x00, 0x08, 0x59, 0x04, 0x12, 0x1d, 0x14, 0x00, 0x1e, 0x04,
        0x08, 0xb8, 0x00, 0x20, 0xb7, 0x00, 0x26, 0x4c, 0xb2, 0x00, 0x29, 0x12, 0x2f, 0x08, 0xbd,
        0x00, 0x02, 0x59, 0x03, 0x2b, 0xb4, 0x00, 0x07, 0xb8, 0x00, 0x20, 0x53, 0x59, 0x04, 0x2b,
        0xb4, 0x00, 0x0d, 0x53, 0x59, 0x05, 0x2b, 0xb4, 0x00, 0x11, 0xb8, 0x00, 0x31, 0x53, 0x59,
        0x06, 0x2b, 0xb4, 0x00, 0x15, 0xb8, 0x00, 0x36, 0x53, 0x59, 0x07, 0x2b, 0xb4, 0x00, 0x19,
        0x53, 0xb6, 0x00, 0x3b, 0x57, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x42, 0x00, 0x00, 0x00,
        0x16, 0x00, 0x05, 0x00, 0x00, 0x00, 0x11, 0x00, 0x13, 0x00, 0x12, 0x00, 0x33, 0x00, 0x13,
        0x00, 0x48, 0x00, 0x12, 0x00, 0x4c, 0x00, 0x14, 0x00, 0x01, 0x00, 0x45, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x46,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 0,
        major_version: 65,
        constant_pool_count: 71,
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
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 9,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 10,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 11,
                descriptor_index: 12,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![70, 105, 101, 108, 100, 115],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 110, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![73],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 14,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 15,
                descriptor_index: 16,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![116, 119, 111],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 18,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103,
                    59,
                ],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 18,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 19,
                descriptor_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![116, 104, 114, 101, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![68],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 22,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 23,
                descriptor_index: 24,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![102, 111, 117, 114],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![90],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 26,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 27,
                descriptor_index: 28,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![102, 105, 118, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110, 116, 101, 103, 101,
                    114, 59,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 15,
            }),
            Some(ConstantDoubleInfo {
                tag: 6,
                high_bytes: 1074266112,
                low_bytes: 0,
            }),
            None,
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 33,
                name_and_type_index: 34,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 35,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 36,
                descriptor_index: 37,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 17,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110, 116, 101, 103, 101, 114,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![118, 97, 108, 117, 101, 79, 102],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 22,
                bytes: vec![
                    40, 73, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110, 116, 101,
                    103, 101, 114, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 8,
                name_and_type_index: 39,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 40,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 43,
                bytes: vec![
                    40, 73, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                    110, 103, 59, 68, 90, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110,
                    116, 101, 103, 101, 114, 59, 41, 86,
                ],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 42,
                name_and_type_index: 43,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 44,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 45,
                descriptor_index: 46,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 48,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    37, 100, 44, 32, 37, 115, 44, 32, 37, 102, 44, 32, 37, 98, 44, 32, 37, 100, 10,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 50,
                name_and_type_index: 51,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 52,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 36,
                descriptor_index: 53,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 68, 111, 117, 98, 108, 101,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 68, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 68, 111, 117, 98,
                    108, 101, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 55,
                name_and_type_index: 56,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 57,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 36,
                descriptor_index: 58,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 17,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 66, 111, 111, 108, 101, 97, 110,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 22,
                bytes: vec![
                    40, 90, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 66, 111, 111, 108,
                    101, 97, 110, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 60,
                name_and_type_index: 61,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 62,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 63,
                descriptor_index: 64,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![112, 114, 105, 110, 116, 102],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 60,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101,
                    99, 116, 59, 41, 76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110,
                    116, 83, 116, 114, 101, 97, 109, 59,
                ],
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
                length: 11,
                bytes: vec![70, 105, 101, 108, 100, 115, 46, 106, 97, 118, 97],
            }),
        ],
        access_flags: 33,
        this_class: 8,
        super_class: 2,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 5,
        fields: vec![
            FieldInfo {
                access_flags: 2,
                name_index: 11,
                descriptor_index: 12,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 2,
                name_index: 15,
                descriptor_index: 16,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 1,
                name_index: 19,
                descriptor_index: 20,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 4,
                name_index: 23,
                descriptor_index: 24,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 0,
                name_index: 27,
                descriptor_index: 28,
                attributes_count: 0,
                attributes: vec![],
            },
        ],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 0,
                name_index: 5,
                descriptor_index: 40,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 65,
                    attribute_length: 80,
                    max_stack: 3,
                    max_locals: 7,
                    code_length: 32,
                    code: vec![
                        42, 183, 0, 1, 42, 27, 181, 0, 7, 42, 44, 181, 0, 13, 42, 41, 181, 0, 17,
                        42, 21, 5, 181, 0, 21, 42, 25, 6, 181, 0, 25, 177,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 66,
                        attribute_length: 30,
                        line_number_table_length: 7,
                        line_number_table: vec![
                            LineNumber {
                                start_pc: 0,
                                line_number: 8,
                            },
                            LineNumber {
                                start_pc: 4,
                                line_number: 9,
                            },
                            LineNumber {
                                start_pc: 9,
                                line_number: 10,
                            },
                            LineNumber {
                                start_pc: 14,
                                line_number: 11,
                            },
                            LineNumber {
                                start_pc: 19,
                                line_number: 12,
                            },
                            LineNumber {
                                start_pc: 25,
                                line_number: 13,
                            },
                            LineNumber {
                                start_pc: 31,
                                line_number: 14,
                            },
                        ],
                    }],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 67,
                descriptor_index: 68,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 65,
                    attribute_length: 117,
                    max_stack: 8,
                    max_locals: 2,
                    code_length: 77,
                    code: vec![
                        187, 0, 8, 89, 4, 18, 29, 20, 0, 30, 4, 8, 184, 0, 32, 183, 0, 38, 76, 178,
                        0, 41, 18, 47, 8, 189, 0, 2, 89, 3, 43, 180, 0, 7, 184, 0, 32, 83, 89, 4,
                        43, 180, 0, 13, 83, 89, 5, 43, 180, 0, 17, 184, 0, 49, 83, 89, 6, 43, 180,
                        0, 21, 184, 0, 54, 83, 89, 7, 43, 180, 0, 25, 83, 182, 0, 59, 87, 177,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 66,
                        attribute_length: 22,
                        line_number_table_length: 5,
                        line_number_table: vec![
                            LineNumber {
                                start_pc: 0,
                                line_number: 17,
                            },
                            LineNumber {
                                start_pc: 19,
                                line_number: 18,
                            },
                            LineNumber {
                                start_pc: 51,
                                line_number: 19,
                            },
                            LineNumber {
                                start_pc: 72,
                                line_number: 18,
                            },
                            LineNumber {
                                start_pc: 76,
                                line_number: 20,
                            },
                        ],
                    }],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 69,
            attribute_length: 2,
            sourcefile_index: 70,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dev/playground/ArithEvaluator.class
//  Last modified 31-Jan-2023; size 1148 bytes
//  SHA-256 checksum 709b0cbc3ec9c48129e97fd18ba5f4ed5c24ada073243ddc425b449d43bb2b9a
//  Compiled from "ArithEvaluator.java"
//public class ArithEvaluator
//  minor version: 0
//  major version: 65
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #22                         // ArithEvaluator
//  super_class: #2                         // java/lang/Object
//  interfaces: 0, fields: 1, methods: 2, attributes: 1
//Constant pool:
//   #1 = Methodref          #2.#3          // java/lang/Object."<init>":()V
//   #2 = Class              #4             // java/lang/Object
//   #3 = NameAndType        #5:#6          // "<init>":()V
//   #4 = Utf8               java/lang/Object
//   #5 = Utf8               <init>
//   #6 = Utf8               ()V
//   #7 = Class              #8             // java/util/Scanner
//   #8 = Utf8               java/util/Scanner
//   #9 = Fieldref           #10.#11        // java/lang/System.in:Ljava/io/InputStream;
//  #10 = Class              #12            // java/lang/System
//  #11 = NameAndType        #13:#14        // in:Ljava/io/InputStream;
//  #12 = Utf8               java/lang/System
//  #13 = Utf8               in
//  #14 = Utf8               Ljava/io/InputStream;
//  #15 = Methodref          #7.#16         // java/util/Scanner."<init>":(Ljava/io/InputStream;)V
//  #16 = NameAndType        #5:#17         // "<init>":(Ljava/io/InputStream;)V
//  #17 = Utf8               (Ljava/io/InputStream;)V
//  #18 = Fieldref           #10.#19        // java/lang/System.out:Ljava/io/PrintStream;
//  #19 = NameAndType        #20:#21        // out:Ljava/io/PrintStream;
//  #20 = Utf8               out
//  #21 = Utf8               Ljava/io/PrintStream;
//  #22 = Class              #23            // ArithEvaluator
//  #23 = Utf8               ArithEvaluator
//  #24 = String             #25            // >>
//  #25 = Utf8               >>
//  #26 = Methodref          #27.#28        // java/io/PrintStream.print:(Ljava/lang/String;)V
//  #27 = Class              #29            // java/io/PrintStream
//  #28 = NameAndType        #30:#31        // print:(Ljava/lang/String;)V
//  #29 = Utf8               java/io/PrintStream
//  #30 = Utf8               print
//  #31 = Utf8               (Ljava/lang/String;)V
//  #32 = Methodref          #27.#33        // java/io/PrintStream.flush:()V
//  #33 = NameAndType        #34:#6         // flush:()V
//  #34 = Utf8               flush
//  #35 = Methodref          #7.#36         // java/util/Scanner.nextLine:()Ljava/lang/String;
//  #36 = NameAndType        #37:#38        // nextLine:()Ljava/lang/String;
//  #37 = Utf8               nextLine
//  #38 = Utf8               ()Ljava/lang/String;
//  #39 = Methodref          #40.#41        // java/lang/String.trim:()Ljava/lang/String;
//  #40 = Class              #42            // java/lang/String
//  #41 = NameAndType        #43:#38        // trim:()Ljava/lang/String;
//  #42 = Utf8               java/lang/String
//  #43 = Utf8               trim
//  #44 = Class              #45            // Parser
//  #45 = Utf8               Parser
//  #46 = Class              #47            // Lexer
//  #47 = Utf8               Lexer
//  #48 = Methodref          #46.#49        // Lexer."<init>":(Ljava/lang/String;)V
//  #49 = NameAndType        #5:#31         // "<init>":(Ljava/lang/String;)V
//  #50 = Methodref          #44.#51        // Parser."<init>":(LLexer;)V
//  #51 = NameAndType        #5:#52         // "<init>":(LLexer;)V
//  #52 = Utf8               (LLexer;)V
//  #53 = Class              #54            // Evaluator
//  #54 = Utf8               Evaluator
//  #55 = Methodref          #53.#3         // Evaluator."<init>":()V
//  #56 = Methodref          #44.#57        // Parser.parse:()LAst;
//  #57 = NameAndType        #58:#59        // parse:()LAst;
//  #58 = Utf8               parse
//  #59 = Utf8               ()LAst;
//  #60 = Methodref          #53.#61        // Evaluator.eval:(LAst;)D
//  #61 = NameAndType        #62:#63        // eval:(LAst;)D
//  #62 = Utf8               eval
//  #63 = Utf8               (LAst;)D
//  #64 = Methodref          #27.#65        // java/io/PrintStream.println:(D)V
//  #65 = NameAndType        #66:#67        // println:(D)V
//  #66 = Utf8               println
//  #67 = Utf8               (D)V
//  #68 = Class              #69            // java/lang/Throwable
//  #69 = Utf8               java/lang/Throwable
//  #70 = Methodref          #7.#71         // java/util/Scanner.close:()V
//  #71 = NameAndType        #72:#6         // close:()V
//  #72 = Utf8               close
//  #73 = Methodref          #68.#74        // java/lang/Throwable.addSuppressed:(Ljava/lang/Throwable;)V
//  #74 = NameAndType        #75:#76        // addSuppressed:(Ljava/lang/Throwable;)V
//  #75 = Utf8               addSuppressed
//  #76 = Utf8               (Ljava/lang/Throwable;)V
//  #77 = Utf8               PROMPT
//  #78 = Utf8               Ljava/lang/String;
//  #79 = Utf8               ConstantValue
//  #80 = Utf8               Code
//  #81 = Utf8               LineNumberTable
//  #82 = Utf8               main
//  #83 = Utf8               ([Ljava/lang/String;)V
//  #84 = Utf8               StackMapTable
//  #85 = Class              #86            // "[Ljava/lang/String;"
//  #86 = Utf8               [Ljava/lang/String;
//  #87 = Utf8               SourceFile
//  #88 = Utf8               ArithEvaluator.java
//{
//  public ArithEvaluator();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
//         4: return
//      LineNumberTable:
//        line 3: 0
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=5, locals=5, args_size=1
//         0: new           #7                  // class java/util/Scanner
//         3: dup
//         4: getstatic     #9                  // Field java/lang/System.in:Ljava/io/InputStream;
//         7: invokespecial #15                 // Method java/util/Scanner."<init>":(Ljava/io/InputStream;)V
//        10: astore_1
//        11: getstatic     #18                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        14: ldc           #24                 // String >>
//        16: invokevirtual #26                 // Method java/io/PrintStream.print:(Ljava/lang/String;)V
//        19: getstatic     #18                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        22: invokevirtual #32                 // Method java/io/PrintStream.flush:()V
//        25: aload_1
//        26: invokevirtual #35                 // Method java/util/Scanner.nextLine:()Ljava/lang/String;
//        29: invokevirtual #39                 // Method java/lang/String.trim:()Ljava/lang/String;
//        32: astore_2
//        33: new           #44                 // class Parser
//        36: dup
//        37: new           #46                 // class Lexer
//        40: dup
//        41: aload_2
//        42: invokespecial #48                 // Method Lexer."<init>":(Ljava/lang/String;)V
//        45: invokespecial #50                 // Method Parser."<init>":(LLexer;)V
//        48: astore_3
//        49: new           #53                 // class Evaluator
//        52: dup
//        53: invokespecial #55                 // Method Evaluator."<init>":()V
//        56: astore        4
//        58: getstatic     #18                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        61: aload         4
//        63: aload_3
//        64: invokevirtual #56                 // Method Parser.parse:()LAst;
//        67: invokevirtual #60                 // Method Evaluator.eval:(LAst;)D
//        70: invokevirtual #64                 // Method java/io/PrintStream.println:(D)V
//        73: goto          11
//        76: astore_2
//        77: aload_1
//        78: invokevirtual #70                 // Method java/util/Scanner.close:()V
//        81: goto          90
//        84: astore_3
//        85: aload_2
//        86: aload_3
//        87: invokevirtual #73                 // Method java/lang/Throwable.addSuppressed:(Ljava/lang/Throwable;)V
//        90: aload_2
//        91: athrow
//      Exception table:
//         from    to  target type
//            11    76    76   Class java/lang/Throwable
//            77    81    84   Class java/lang/Throwable
//      LineNumberTable:
//        line 7: 0
//        line 9: 11
//        line 10: 19
//        line 12: 25
//        line 13: 33
//        line 14: 49
//        line 15: 58
//        line 16: 73
//        line 7: 76
//      StackMapTable: number_of_entries = 4
//        frame_type = 252 /* append */
//          offset_delta = 11
//          locals = [ class java/util/Scanner ]
//        frame_type = 247 /* same_locals_1_stack_item_frame_extended */
//          offset_delta = 64
//          stack = [ class java/lang/Throwable ]
//        frame_type = 255 /* full_frame */
//          offset_delta = 7
//          locals = [ class "[Ljava/lang/String;", class java/util/Scanner, class java/lang/Throwable ]
//          stack = [ class java/lang/Throwable ]
//        frame_type = 5 /* same */
//}
//SourceFile: "ArithEvaluator.java"
#[test]
fn test_serialize_arith_evaluator() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x59, 0x0a, 0x00, 0x02, 0x00, 0x03,
        0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06,
        0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x07, 0x00, 0x08,
        0x01, 0x00, 0x11, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x75, 0x74, 0x69, 0x6c, 0x2f, 0x53, 0x63,
        0x61, 0x6e, 0x6e, 0x65, 0x72, 0x09, 0x00, 0x0a, 0x00, 0x0b, 0x07, 0x00, 0x0c, 0x0c, 0x00,
        0x0d, 0x00, 0x0e, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67,
        0x2f, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x01, 0x00, 0x02, 0x69, 0x6e, 0x01, 0x00, 0x15,
        0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x53,
        0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x0a, 0x00, 0x07, 0x00, 0x10, 0x0c, 0x00, 0x05, 0x00,
        0x11, 0x01, 0x00, 0x18, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x49,
        0x6e, 0x70, 0x75, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x29, 0x56, 0x09, 0x00,
        0x0a, 0x00, 0x13, 0x0c, 0x00, 0x14, 0x00, 0x15, 0x01, 0x00, 0x03, 0x6f, 0x75, 0x74, 0x01,
        0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e,
        0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x07, 0x00, 0x17, 0x01, 0x00, 0x0e, 0x41,
        0x72, 0x69, 0x74, 0x68, 0x45, 0x76, 0x61, 0x6c, 0x75, 0x61, 0x74, 0x6f, 0x72, 0x08, 0x00,
        0x19, 0x01, 0x00, 0x03, 0x3e, 0x3e, 0x20, 0x0a, 0x00, 0x1b, 0x00, 0x1c, 0x07, 0x00, 0x1d,
        0x0c, 0x00, 0x1e, 0x00, 0x1f, 0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f,
        0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x05,
        0x70, 0x72, 0x69, 0x6e, 0x74, 0x01, 0x00, 0x15, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f,
        0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x0a,
        0x00, 0x1b, 0x00, 0x21, 0x0c, 0x00, 0x22, 0x00, 0x06, 0x01, 0x00, 0x05, 0x66, 0x6c, 0x75,
        0x73, 0x68, 0x0a, 0x00, 0x07, 0x00, 0x24, 0x0c, 0x00, 0x25, 0x00, 0x26, 0x01, 0x00, 0x08,
        0x6e, 0x65, 0x78, 0x74, 0x4c, 0x69, 0x6e, 0x65, 0x01, 0x00, 0x14, 0x28, 0x29, 0x4c, 0x6a,
        0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67,
        0x3b, 0x0a, 0x00, 0x28, 0x00, 0x29, 0x07, 0x00, 0x2a, 0x0c, 0x00, 0x2b, 0x00, 0x26, 0x01,
        0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72,
        0x69, 0x6e, 0x67, 0x01, 0x00, 0x04, 0x74, 0x72, 0x69, 0x6d, 0x07, 0x00, 0x2d, 0x01, 0x00,
        0x06, 0x50, 0x61, 0x72, 0x73, 0x65, 0x72, 0x07, 0x00, 0x2f, 0x01, 0x00, 0x05, 0x4c, 0x65,
        0x78, 0x65, 0x72, 0x0a, 0x00, 0x2e, 0x00, 0x31, 0x0c, 0x00, 0x05, 0x00, 0x1f, 0x0a, 0x00,
        0x2c, 0x00, 0x33, 0x0c, 0x00, 0x05, 0x00, 0x34, 0x01, 0x00, 0x0a, 0x28, 0x4c, 0x4c, 0x65,
        0x78, 0x65, 0x72, 0x3b, 0x29, 0x56, 0x07, 0x00, 0x36, 0x01, 0x00, 0x09, 0x45, 0x76, 0x61,
        0x6c, 0x75, 0x61, 0x74, 0x6f, 0x72, 0x0a, 0x00, 0x35, 0x00, 0x03, 0x0a, 0x00, 0x2c, 0x00,
        0x39, 0x0c, 0x00, 0x3a, 0x00, 0x3b, 0x01, 0x00, 0x05, 0x70, 0x61, 0x72, 0x73, 0x65, 0x01,
        0x00, 0x07, 0x28, 0x29, 0x4c, 0x41, 0x73, 0x74, 0x3b, 0x0a, 0x00, 0x35, 0x00, 0x3d, 0x0c,
        0x00, 0x3e, 0x00, 0x3f, 0x01, 0x00, 0x04, 0x65, 0x76, 0x61, 0x6c, 0x01, 0x00, 0x08, 0x28,
        0x4c, 0x41, 0x73, 0x74, 0x3b, 0x29, 0x44, 0x0a, 0x00, 0x1b, 0x00, 0x41, 0x0c, 0x00, 0x42,
        0x00, 0x43, 0x01, 0x00, 0x07, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x6c, 0x6e, 0x01, 0x00, 0x04,
        0x28, 0x44, 0x29, 0x56, 0x07, 0x00, 0x45, 0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f,
        0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x54, 0x68, 0x72, 0x6f, 0x77, 0x61, 0x62, 0x6c, 0x65, 0x0a,
        0x00, 0x07, 0x00, 0x47, 0x0c, 0x00, 0x48, 0x00, 0x06, 0x01, 0x00, 0x05, 0x63, 0x6c, 0x6f,
        0x73, 0x65, 0x0a, 0x00, 0x44, 0x00, 0x4a, 0x0c, 0x00, 0x4b, 0x00, 0x4c, 0x01, 0x00, 0x0d,
        0x61, 0x64, 0x64, 0x53, 0x75, 0x70, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x01, 0x00,
        0x18, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x54, 0x68,
        0x72, 0x6f, 0x77, 0x61, 0x62, 0x6c, 0x65, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x06, 0x50, 0x52,
        0x4f, 0x4d, 0x50, 0x54, 0x01, 0x00, 0x12, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61,
        0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x01, 0x00, 0x0d, 0x43, 0x6f,
        0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x01, 0x00, 0x04, 0x43,
        0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65,
        0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00,
        0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53,
        0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x0d, 0x53, 0x74, 0x61, 0x63,
        0x6b, 0x4d, 0x61, 0x70, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x07, 0x00, 0x56, 0x01, 0x00, 0x13,
        0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72,
        0x69, 0x6e, 0x67, 0x3b, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69,
        0x6c, 0x65, 0x01, 0x00, 0x13, 0x41, 0x72, 0x69, 0x74, 0x68, 0x45, 0x76, 0x61, 0x6c, 0x75,
        0x61, 0x74, 0x6f, 0x72, 0x2e, 0x6a, 0x61, 0x76, 0x61, 0x00, 0x21, 0x00, 0x16, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x1a, 0x00, 0x4d, 0x00, 0x4e, 0x00, 0x01, 0x00, 0x4f, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x18, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x06, 0x00, 0x01,
        0x00, 0x50, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a,
        0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x51, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x09, 0x00, 0x52, 0x00, 0x53, 0x00, 0x01, 0x00, 0x50,
        0x00, 0x00, 0x00, 0xcc, 0x00, 0x05, 0x00, 0x05, 0x00, 0x00, 0x00, 0x5c, 0xbb, 0x00, 0x07,
        0x59, 0xb2, 0x00, 0x09, 0xb7, 0x00, 0x0f, 0x4c, 0xb2, 0x00, 0x12, 0x12, 0x18, 0xb6, 0x00,
        0x1a, 0xb2, 0x00, 0x12, 0xb6, 0x00, 0x20, 0x2b, 0xb6, 0x00, 0x23, 0xb6, 0x00, 0x27, 0x4d,
        0xbb, 0x00, 0x2c, 0x59, 0xbb, 0x00, 0x2e, 0x59, 0x2c, 0xb7, 0x00, 0x30, 0xb7, 0x00, 0x32,
        0x4e, 0xbb, 0x00, 0x35, 0x59, 0xb7, 0x00, 0x37, 0x3a, 0x04, 0xb2, 0x00, 0x12, 0x19, 0x04,
        0x2d, 0xb6, 0x00, 0x38, 0xb6, 0x00, 0x3c, 0xb6, 0x00, 0x40, 0xa7, 0xff, 0xc2, 0x4d, 0x2b,
        0xb6, 0x00, 0x46, 0xa7, 0x00, 0x09, 0x4e, 0x2c, 0x2d, 0xb6, 0x00, 0x49, 0x2c, 0xbf, 0x00,
        0x02, 0x00, 0x0b, 0x00, 0x4c, 0x00, 0x4c, 0x00, 0x44, 0x00, 0x4d, 0x00, 0x51, 0x00, 0x54,
        0x00, 0x44, 0x00, 0x02, 0x00, 0x51, 0x00, 0x00, 0x00, 0x26, 0x00, 0x09, 0x00, 0x00, 0x00,
        0x07, 0x00, 0x0b, 0x00, 0x09, 0x00, 0x13, 0x00, 0x0a, 0x00, 0x19, 0x00, 0x0c, 0x00, 0x21,
        0x00, 0x0d, 0x00, 0x31, 0x00, 0x0e, 0x00, 0x3a, 0x00, 0x0f, 0x00, 0x49, 0x00, 0x10, 0x00,
        0x4c, 0x00, 0x07, 0x00, 0x54, 0x00, 0x00, 0x00, 0x22, 0x00, 0x04, 0xfc, 0x00, 0x0b, 0x07,
        0x00, 0x07, 0xf7, 0x00, 0x40, 0x07, 0x00, 0x44, 0xff, 0x00, 0x07, 0x00, 0x03, 0x07, 0x00,
        0x55, 0x07, 0x00, 0x07, 0x07, 0x00, 0x44, 0x00, 0x01, 0x07, 0x00, 0x44, 0x05, 0x00, 0x01,
        0x00, 0x57, 0x00, 0x00, 0x00, 0x02, 0x00, 0x58,
    ];

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 0,
        major_version: 65,
        constant_pool_count: 89,
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
                length: 17,
                bytes: vec![
                    106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 83, 99, 97, 110, 110, 101, 114,
                ],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 10,
                name_and_type_index: 11,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 12,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 13,
                descriptor_index: 14,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 2,
                bytes: vec![105, 110],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 73, 110, 112, 117, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 7,
                name_and_type_index: 16,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 17,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 24,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 105, 111, 47, 73, 110, 112, 117, 116, 83, 116,
                    114, 101, 97, 109, 59, 41, 86,
                ],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 10,
                name_and_type_index: 19,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 20,
                descriptor_index: 21,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 23,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 14,
                bytes: vec![
                    65, 114, 105, 116, 104, 69, 118, 97, 108, 117, 97, 116, 111, 114,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 25,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![62, 62, 32],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 27,
                name_and_type_index: 28,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 29,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 30,
                descriptor_index: 31,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![112, 114, 105, 110, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 41, 86,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 27,
                name_and_type_index: 33,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 34,
                descriptor_index: 6,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![102, 108, 117, 115, 104],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 7,
                name_and_type_index: 36,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 37,
                descriptor_index: 38,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 8,
                bytes: vec![110, 101, 120, 116, 76, 105, 110, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 20,
                bytes: vec![
                    40, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                    110, 103, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 40,
                name_and_type_index: 41,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 42,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 43,
                descriptor_index: 38,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![116, 114, 105, 109],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 45,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![80, 97, 114, 115, 101, 114],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 47,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![76, 101, 120, 101, 114],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 46,
                name_and_type_index: 49,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 31,
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 44,
                name_and_type_index: 51,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 52,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![40, 76, 76, 101, 120, 101, 114, 59, 41, 86],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 54,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 9,
                bytes: vec![69, 118, 97, 108, 117, 97, 116, 111, 114],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 53,
                name_and_type_index: 3,
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 44,
                name_and_type_index: 57,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 58,
                descriptor_index: 59,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![112, 97, 114, 115, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![40, 41, 76, 65, 115, 116, 59],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 53,
                name_and_type_index: 61,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 62,
                descriptor_index: 63,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![101, 118, 97, 108],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 8,
                bytes: vec![40, 76, 65, 115, 116, 59, 41, 68],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 27,
                name_and_type_index: 65,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 66,
                descriptor_index: 67,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![40, 68, 41, 86],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 69,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 84, 104, 114, 111, 119, 97, 98,
                    108, 101,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 7,
                name_and_type_index: 71,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 72,
                descriptor_index: 6,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![99, 108, 111, 115, 101],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 68,
                name_and_type_index: 74,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 75,
                descriptor_index: 76,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 13,
                bytes: vec![
                    97, 100, 100, 83, 117, 112, 112, 114, 101, 115, 115, 101, 100,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 24,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 84, 104, 114, 111, 119,
                    97, 98, 108, 101, 59, 41, 86,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![80, 82, 79, 77, 80, 84],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 18,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103,
                    59,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 13,
                bytes: vec![67, 111, 110, 115, 116, 97, 110, 116, 86, 97, 108, 117, 101],
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
                length: 13,
                bytes: vec![83, 116, 97, 99, 107, 77, 97, 112, 84, 97, 98, 108, 101],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 86,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    65, 114, 105, 116, 104, 69, 118, 97, 108, 117, 97, 116, 111, 114, 46, 106, 97,
                    118, 97,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 22,
        super_class: 2,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 1,
        fields: vec![FieldInfo {
            access_flags: 26,
            name_index: 77,
            descriptor_index: 78,
            attributes_count: 1,
            attributes: vec![ConstantValue {
                attribute_name_index: 79,
                attribute_length: 2,
                constantvalue_index: 24,
            }],
        }],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 5,
                descriptor_index: 6,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 80,
                    attribute_length: 29,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 1, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 81,
                        attribute_length: 6,
                        line_number_table_length: 1,
                        line_number_table: vec![LineNumber {
                            start_pc: 0,
                            line_number: 3,
                        }],
                    }],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 82,
                descriptor_index: 83,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 80,
                    attribute_length: 204,
                    max_stack: 5,
                    max_locals: 5,
                    code_length: 92,
                    code: vec![
                        187, 0, 7, 89, 178, 0, 9, 183, 0, 15, 76, 178, 0, 18, 18, 24, 182, 0, 26,
                        178, 0, 18, 182, 0, 32, 43, 182, 0, 35, 182, 0, 39, 77, 187, 0, 44, 89,
                        187, 0, 46, 89, 44, 183, 0, 48, 183, 0, 50, 78, 187, 0, 53, 89, 183, 0, 55,
                        58, 4, 178, 0, 18, 25, 4, 45, 182, 0, 56, 182, 0, 60, 182, 0, 64, 167, 255,
                        194, 77, 43, 182, 0, 70, 167, 0, 9, 78, 44, 45, 182, 0, 73, 44, 191,
                    ],
                    exception_table_length: 2,
                    exception_table: vec![
                        ExceptionHandler {
                            start_pc: 11,
                            end_pc: 76,
                            handler_pc: 76,
                            catch_type: 67,
                        },
                        ExceptionHandler {
                            start_pc: 77,
                            end_pc: 81,
                            handler_pc: 84,
                            catch_type: 67,
                        },
                    ],
                    code_attributes_count: 2,
                    code_attributes: vec![
                        LineNumberTable {
                            attribute_name_index: 81,
                            attribute_length: 38,
                            line_number_table_length: 9,
                            line_number_table: vec![
                                LineNumber {
                                    start_pc: 0,
                                    line_number: 7,
                                },
                                LineNumber {
                                    start_pc: 11,
                                    line_number: 9,
                                },
                                LineNumber {
                                    start_pc: 19,
                                    line_number: 10,
                                },
                                LineNumber {
                                    start_pc: 25,
                                    line_number: 12,
                                },
                                LineNumber {
                                    start_pc: 33,
                                    line_number: 13,
                                },
                                LineNumber {
                                    start_pc: 49,
                                    line_number: 14,
                                },
                                LineNumber {
                                    start_pc: 58,
                                    line_number: 15,
                                },
                                LineNumber {
                                    start_pc: 73,
                                    line_number: 16,
                                },
                                LineNumber {
                                    start_pc: 76,
                                    line_number: 7,
                                },
                            ],
                        },
                        StackMapTable {
                            attribute_name_index: 84,
                            attribute_length: 34,
                            number_of_entries: 4,
                            entries: vec![
                                AppendFrame {
                                    frame_type: 252,
                                    offset_delta: 11,
                                    locals: vec![ObjectVariableInfo {
                                        tag: 7,
                                        cpool_index: 7,
                                    }],
                                },
                                SameLocals1StackItemFrameExtended {
                                    frame_type: 247,
                                    offset_delta: 64,
                                    stack: vec![ObjectVariableInfo {
                                        tag: 7,
                                        cpool_index: 68,
                                    }],
                                },
                                FullFrame {
                                    frame_type: 255,
                                    offset_delta: 7,
                                    number_of_locals: 3,
                                    locals: vec![
                                        ObjectVariableInfo {
                                            tag: 7,
                                            cpool_index: 85,
                                        },
                                        ObjectVariableInfo {
                                            tag: 7,
                                            cpool_index: 7,
                                        },
                                        ObjectVariableInfo {
                                            tag: 7,
                                            cpool_index: 68,
                                        },
                                    ],
                                    number_of_stack_items: 1,
                                    stack: vec![ObjectVariableInfo {
                                        tag: 7,
                                        cpool_index: 68,
                                    }],
                                },
                                SameFrame { frame_type: 5 },
                            ],
                        },
                    ],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 87,
            attribute_length: 2,
            sourcefile_index: 88,
        }],
    };

    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dev/playground/HelloWorld.class
//  Last modified 01-Mar-2023; size 380 bytes
//  SHA-256 checksum ef195638e3713a3dde3628d6e9f23bc0a3b29b03a2ba8c4d1b676958a9c657b5
//  Compiled from "HelloWorld.java"
//public class HelloWorld
//  minor version: 3
//  major version: 45
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #12                         // HelloWorld
//  super_class: #5                         // java/lang/Object
//  interfaces: 0, fields: 0, methods: 2, attributes: 1
//Constant pool:
//   #1 = NameAndType        #24:#27        // out:Ljava/io/PrintStream;
//   #2 = Utf8               ([Ljava/lang/String;)V
//   #3 = Utf8               java/lang/Object
//   #4 = Utf8               <init>
//   #5 = Class              #3             // java/lang/Object
//   #6 = NameAndType        #4:#8          // "<init>":()V
//   #7 = Class              #18            // java/io/PrintStream
//   #8 = Utf8               ()V
//   #9 = Class              #22            // java/lang/System
//  #10 = Utf8               Code
//  #11 = Utf8               main
//  #12 = Class              #17            // HelloWorld
//  #13 = Fieldref           #9.#1          // java/lang/System.out:Ljava/io/PrintStream;
//  #14 = Utf8               SourceFile
//  #15 = Utf8               Hello, world
//  #16 = NameAndType        #19:#25        // println:(Ljava/lang/String;)V
//  #17 = Utf8               HelloWorld
//  #18 = Utf8               java/io/PrintStream
//  #19 = Utf8               println
//  #20 = Methodref          #5.#6          // java/lang/Object."<init>":()V
//  #21 = String             #15            // Hello, world
//  #22 = Utf8               java/lang/System
//  #23 = Methodref          #7.#16         // java/io/PrintStream.println:(Ljava/lang/String;)V
//  #24 = Utf8               out
//  #25 = Utf8               (Ljava/lang/String;)V
//  #26 = Utf8               HelloWorld.java
//  #27 = Utf8               Ljava/io/PrintStream;
//{
//  public HelloWorld();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #20                 // Method java/lang/Object."<init>":()V
//         4: return
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=2, locals=1, args_size=1
//         0: getstatic     #13                 // Field java/lang/System.out:Ljava/io/PrintStream;
//         3: ldc           #21                 // String Hello, world
//         5: invokevirtual #23                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
//         8: return
//}
//SourceFile: "HelloWorld.java"
#[test]
fn test_serialize_hello_world_no_line_number() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x1c, 0x0c, 0x00, 0x18, 0x00, 0x1b,
        0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67,
        0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x10, 0x6a, 0x61,
        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01,
        0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x07, 0x00, 0x03, 0x0c, 0x00, 0x04, 0x00,
        0x08, 0x07, 0x00, 0x12, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x07, 0x00, 0x16, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x07, 0x00, 0x11,
        0x09, 0x00, 0x09, 0x00, 0x01, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46,
        0x69, 0x6c, 0x65, 0x01, 0x00, 0x0c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f,
        0x72, 0x6c, 0x64, 0x0c, 0x00, 0x13, 0x00, 0x19, 0x01, 0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c,
        0x6f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69,
        0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00,
        0x07, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x6c, 0x6e, 0x0a, 0x00, 0x05, 0x00, 0x06, 0x08, 0x00,
        0x0f, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53,
        0x79, 0x73, 0x74, 0x65, 0x6d, 0x0a, 0x00, 0x07, 0x00, 0x10, 0x01, 0x00, 0x03, 0x6f, 0x75,
        0x74, 0x01, 0x00, 0x15, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67,
        0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x0f, 0x48, 0x65,
        0x6c, 0x6c, 0x6f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x6a, 0x61, 0x76, 0x61, 0x01, 0x00,
        0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74,
        0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x00, 0x21, 0x00, 0x0c, 0x00, 0x05, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x04, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0a, 0x00,
        0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x14,
        0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x01, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x15, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0xb2, 0x00, 0x0d,
        0x12, 0x15, 0xb6, 0x00, 0x17, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x1a,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 3,
        major_version: 45,
        constant_pool_count: 28,
        constant_pool: vec![
            None,
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 24,
                descriptor_index: 27,
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
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 3,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 4,
                descriptor_index: 8,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 18,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 22,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 17,
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 9,
                name_and_type_index: 1,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 12,
                bytes: vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100],
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 19,
                descriptor_index: 25,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![72, 101, 108, 108, 111, 87, 111, 114, 108, 100],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 5,
                name_and_type_index: 6,
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 15,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 7,
                name_and_type_index: 16,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 41, 86,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    72, 101, 108, 108, 111, 87, 111, 114, 108, 100, 46, 106, 97, 118, 97,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 12,
        super_class: 5,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 4,
                descriptor_index: 8,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 10,
                    attribute_length: 17,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 20, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 11,
                descriptor_index: 2,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 10,
                    attribute_length: 21,
                    max_stack: 2,
                    max_locals: 1,
                    code_length: 9,
                    code: vec![178, 0, 13, 18, 21, 182, 0, 23, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 14,
            attribute_length: 2,
            sourcefile_index: 26,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dev/playground/FieldsDemo.class
//  Last modified 02-Mar-2023; size 345 bytes
//  SHA-256 checksum 46d3f52393888f1761e0ce540dde51e3f3daba5e66cddaf4627ab8e0ef36cddc
//  Compiled from "FieldsDemo.pho"
//public class FieldsDemo
//  minor version: 3
//  major version: 45
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #12                         // FieldsDemo
//  super_class: #5                         // java/lang/Object
//  interfaces: 0, fields: 4, methods: 2, attributes: 1
//Constant pool:
//   #1 = Utf8               ([Ljava/lang/String;)V
//   #2 = Utf8               java/lang/Object
//   #3 = Utf8               <init>
//   #4 = NameAndType        #3:#9          // "<init>":()V
//   #5 = Class              #2             // java/lang/Object
//   #6 = Utf8               Foo
//   #7 = String             #6             // Foo
//   #8 = Utf8               FieldsDemo.pho
//   #9 = Utf8               ()V
//  #10 = Utf8               Code
//  #11 = Utf8               main
//  #12 = Class              #19            // FieldsDemo
//  #13 = Utf8               ConstantValue
//  #14 = Utf8               z
//  #15 = Utf8               y
//  #16 = Utf8               SourceFile
//  #17 = Utf8               I
//  #18 = Utf8               x
//  #19 = Utf8               FieldsDemo
//  #20 = Utf8               D
//  #21 = Methodref          #5.#4          // java/lang/Object."<init>":()V
//  #22 = Float              3.14159f
//  #23 = Utf8               PI
//  #24 = Utf8               Ljava/lang/String;
//{
//  public static final double PI;
//    descriptor: D
//    flags: (0x0019) ACC_PUBLIC, ACC_STATIC, ACC_FINAL
//    ConstantValue: float 3.14159f
//
//  public FieldsDemo();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #21                 // Method java/lang/Object."<init>":()V
//         4: return
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=1, locals=1, args_size=1
//}
//SourceFile: "FieldsDemo.pho"
#[test]
fn test_serialize_fields_demo() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x1a, 0x01, 0x00, 0x16, 0x28, 0x5b,
        0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69,
        0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61,
        0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x0c, 0x00, 0x03, 0x00, 0x09, 0x07, 0x00, 0x02, 0x01, 0x00, 0x03, 0x46,
        0x6f, 0x6f, 0x08, 0x00, 0x06, 0x01, 0x00, 0x0e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x44,
        0x65, 0x6d, 0x6f, 0x2e, 0x70, 0x68, 0x6f, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x07, 0x00, 0x14,
        0x01, 0x00, 0x0d, 0x43, 0x6f, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x75,
        0x65, 0x01, 0x00, 0x01, 0x7a, 0x01, 0x00, 0x01, 0x79, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75,
        0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x01, 0x49, 0x01, 0x00, 0x01, 0x78,
        0x01, 0x00, 0x01, 0x46, 0x01, 0x00, 0x0a, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x44, 0x65,
        0x6d, 0x6f, 0x01, 0x00, 0x01, 0x44, 0x0a, 0x00, 0x05, 0x00, 0x04, 0x04, 0x40, 0x49, 0x0f,
        0xd0, 0x01, 0x00, 0x02, 0x50, 0x49, 0x01, 0x00, 0x12, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f,
        0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x00, 0x21, 0x00,
        0x0c, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x00, 0x11, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x0f, 0x00, 0x15, 0x00, 0x00, 0x00, 0x02, 0x00, 0x0e, 0x00, 0x19, 0x00,
        0x01, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x07, 0x00, 0x19, 0x00, 0x18, 0x00, 0x13,
        0x00, 0x01, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x17, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x03, 0x00, 0x09, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x16, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
        0x00, 0x0b, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x02, 0x00, 0x08,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 3,
        major_version: 45,
        constant_pool_count: 26,
        constant_pool: vec![
            None,
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
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 3,
                descriptor_index: 9,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 2,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![70, 111, 111],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 6,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 14,
                bytes: vec![
                    70, 105, 101, 108, 100, 115, 68, 101, 109, 111, 46, 112, 104, 111,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 13,
                bytes: vec![67, 111, 110, 115, 116, 97, 110, 116, 86, 97, 108, 117, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![122],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![121],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![73],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![120],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![70],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![70, 105, 101, 108, 100, 115, 68, 101, 109, 111],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![68],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 5,
                name_and_type_index: 4,
            }),
            Some(ConstantFloatInfo {
                tag: 4,
                bytes: 1078530000,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 2,
                bytes: vec![80, 73],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 18,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103,
                    59,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 12,
        super_class: 5,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 4,
        fields: vec![
            FieldInfo {
                access_flags: 2,
                name_index: 18,
                descriptor_index: 17,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 2,
                name_index: 15,
                descriptor_index: 21,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 2,
                name_index: 14,
                descriptor_index: 25,
                attributes_count: 1,
                attributes: vec![ConstantValue {
                    attribute_name_index: 13,
                    attribute_length: 2,
                    constantvalue_index: 7,
                }],
            },
            FieldInfo {
                access_flags: 25,
                name_index: 24,
                descriptor_index: 19,
                attributes_count: 1,
                attributes: vec![ConstantValue {
                    attribute_name_index: 13,
                    attribute_length: 2,
                    constantvalue_index: 23,
                }],
            },
        ],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 3,
                descriptor_index: 9,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 10,
                    attribute_length: 17,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 22, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 11,
                descriptor_index: 1,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 10,
                    attribute_length: 13,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 1,
                    code: vec![177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 16,
            attribute_length: 2,
            sourcefile_index: 8,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dev/playground/Catcher.class
//  Last modified 04-Mar-2023; size 424 bytes
//  SHA-256 checksum 530a9244b84b26a34b58decb6d48f63bd75a28e8f78e75320903a8855b15962a
//  Compiled from "Catcher.pho"
//public class Catcher
//  minor version: 3
//  major version: 45
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #27                         // Catcher
//  super_class: #5                         // java/lang/Object
//  interfaces: 0, fields: 0, methods: 2, attributes: 1
//Constant pool:
//   #1 = NameAndType        #28:#30        // out:Ljava/io/PrintStream;
//   #2 = Utf8               ([Ljava/lang/String;)V
//   #3 = Utf8               java/lang/Object
//   #4 = Utf8               <init>
//   #5 = Class              #3             // java/lang/Object
//   #6 = NameAndType        #4:#8          // "<init>":()V
//   #7 = Class              #21            // java/io/PrintStream
//   #8 = Utf8               ()V
//   #9 = Class              #24            // java/lang/System
//  #10 = Utf8               Code
//  #11 = Methodref          #14.#6         // java/lang/Exception."<init>":()V
//  #12 = Utf8               Catcher
//  #13 = Utf8               main
//  #14 = Class              #26            // java/lang/Exception
//  #15 = Fieldref           #9.#1          // java/lang/System.out:Ljava/io/PrintStream;
//  #16 = String             #17            // Exception Caught
//  #17 = Utf8               Exception Caught
//  #18 = Utf8               SourceFile
//  #19 = Utf8               Catcher.pho
//  #20 = NameAndType        #22:#29        // println:(Ljava/lang/String;)V
//  #21 = Utf8               java/io/PrintStream
//  #22 = Utf8               println
//  #23 = Methodref          #5.#6          // java/lang/Object."<init>":()V
//  #24 = Utf8               java/lang/System
//  #25 = Methodref          #7.#20         // java/io/PrintStream.println:(Ljava/lang/String;)V
//  #26 = Utf8               java/lang/Exception
//  #27 = Class              #12            // Catcher
//  #28 = Utf8               out
//  #29 = Utf8               (Ljava/lang/String;)V
//  #30 = Utf8               Ljava/io/PrintStream;
//{
//  public Catcher();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #23                 // Method java/lang/Object."<init>":()V
//         4: return
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=3, locals=3, args_size=1
//         0: new           #14                 // class java/lang/Exception
//         3: dup
//         4: invokespecial #11                 // Method java/lang/Exception."<init>":()V
//         7: athrow
//         8: pop
//         9: getstatic     #15                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        12: ldc           #16                 // String Exception Caught
//        14: invokevirtual #25                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
//        17: return
//      Exception table:
//         from    to  target type
//             0     8     8   Class java/lang/Exception
//}
//SourceFile: "Catcher.pho"
#[test]
fn test_serialize_catcher() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x1f, 0x0c, 0x00, 0x1c, 0x00, 0x1e,
        0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67,
        0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x10, 0x6a, 0x61,
        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01,
        0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x07, 0x00, 0x03, 0x0c, 0x00, 0x04, 0x00,
        0x08, 0x07, 0x00, 0x15, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x07, 0x00, 0x18, 0x01, 0x00,
        0x04, 0x43, 0x6f, 0x64, 0x65, 0x0a, 0x00, 0x0e, 0x00, 0x06, 0x01, 0x00, 0x07, 0x43, 0x61,
        0x74, 0x63, 0x68, 0x65, 0x72, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x07, 0x00, 0x1a,
        0x09, 0x00, 0x09, 0x00, 0x01, 0x08, 0x00, 0x11, 0x01, 0x00, 0x10, 0x45, 0x78, 0x63, 0x65,
        0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x43, 0x61, 0x75, 0x67, 0x68, 0x74, 0x01, 0x00, 0x0a,
        0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x0b, 0x43, 0x61,
        0x74, 0x63, 0x68, 0x65, 0x72, 0x2e, 0x70, 0x68, 0x6f, 0x0c, 0x00, 0x16, 0x00, 0x1d, 0x01,
        0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74,
        0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x07, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x6c,
        0x6e, 0x0a, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x0a, 0x00, 0x07, 0x00, 0x14,
        0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x45, 0x78,
        0x63, 0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x07, 0x00, 0x0c, 0x01, 0x00, 0x03, 0x6f, 0x75,
        0x74, 0x01, 0x00, 0x15, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67,
        0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x15, 0x4c, 0x6a,
        0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72,
        0x65, 0x61, 0x6d, 0x3b, 0x00, 0x21, 0x00, 0x1b, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x01, 0x00, 0x04, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x11,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x17, 0xb1, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x09, 0x00, 0x0d, 0x00, 0x02, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00,
        0x26, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x12, 0xbb, 0x00, 0x0e, 0x59, 0xb7, 0x00,
        0x0b, 0xbf, 0x57, 0xb2, 0x00, 0x0f, 0x12, 0x10, 0xb6, 0x00, 0x19, 0xb1, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x01, 0x00, 0x12, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x13,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 3,
        major_version: 45,
        constant_pool_count: 31,
        constant_pool: vec![
            None,
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 28,
                descriptor_index: 30,
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
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 3,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 4,
                descriptor_index: 8,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 21,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 24,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 14,
                name_and_type_index: 6,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![67, 97, 116, 99, 104, 101, 114],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 26,
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 9,
                name_and_type_index: 1,
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 17,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    69, 120, 99, 101, 112, 116, 105, 111, 110, 32, 67, 97, 117, 103, 104, 116,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 11,
                bytes: vec![67, 97, 116, 99, 104, 101, 114, 46, 112, 104, 111],
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 22,
                descriptor_index: 29,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 5,
                name_and_type_index: 6,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 7,
                name_and_type_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 69, 120, 99, 101, 112, 116, 105,
                    111, 110,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 12,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 41, 86,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 27,
        super_class: 5,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 4,
                descriptor_index: 8,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 10,
                    attribute_length: 17,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 23, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 13,
                descriptor_index: 2,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 10,
                    attribute_length: 38,
                    max_stack: 3,
                    max_locals: 3,
                    code_length: 18,
                    code: vec![
                        187, 0, 14, 89, 183, 0, 11, 191, 87, 178, 0, 15, 18, 16, 182, 0, 25, 177,
                    ],
                    exception_table_length: 1,
                    exception_table: vec![ExceptionHandler {
                        start_pc: 0,
                        end_pc: 8,
                        handler_pc: 8,
                        catch_type: 13,
                    }],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 18,
            attribute_length: 2,
            sourcefile_index: 19,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    serializer.serialize(&classfile)?;
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dev/playground/LookupswitchDemo.class
//  Last modified 07-Mar-2023; size 533 bytes
//  SHA-256 checksum f31ae9c21e28ccbd6126dfd09b46ee6ba0f21426bf83390913a4911b49d241d8
//  Compiled from "LookupSwitchDemo.pho"
//public class LookupswitchDemo
//  minor version: 3
//  major version: 45
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #5                          // LookupswitchDemo
//  super_class: #10                        // java/lang/Object
//  interfaces: 0, fields: 0, methods: 3, attributes: 1
//Constant pool:
//   #1 = Integer            200
//   #2 = Utf8               LookupswitchDemo
//   #3 = Utf8               ()V
//   #4 = Utf8               main
//   #5 = Class              #2             // LookupswitchDemo
//   #6 = NameAndType        #23:#22        // println:(I)V
//   #7 = Integer            10
//   #8 = Integer            100
//   #9 = Utf8               java/lang/Object
//  #10 = Class              #9             // java/lang/Object
//  #11 = Integer            1
//  #12 = Integer            0
//  #13 = Integer            12345
//  #14 = Utf8               java/lang/System
//  #15 = Class              #14            // java/lang/System
//  #16 = NameAndType        #19:#32        // demo:(I)I
//  #17 = Methodref          #5.#16         // LookupswitchDemo.demo:(I)I
//  #18 = Utf8               ([Ljava/lang/String;)V
//  #19 = Utf8               demo
//  #20 = NameAndType        #25:#3         // "<init>":()V
//  #21 = Class              #35            // java/io/PrintStream
//  #22 = Utf8               (I)V
//  #23 = Utf8               println
//  #24 = Utf8               SourceFile
//  #25 = Utf8               <init>
//  #26 = Methodref          #21.#6         // java/io/PrintStream.println:(I)V
//  #27 = NameAndType        #28:#36        // out:Ljava/io/PrintStream;
//  #28 = Utf8               out
//  #29 = Utf8               LookupSwitchDemo.pho
//  #30 = Methodref          #10.#20        // java/lang/Object."<init>":()V
//  #31 = Methodref          #5.#20         // LookupswitchDemo."<init>":()V
//  #32 = Utf8               (I)I
//  #33 = Fieldref           #15.#27        // java/lang/System.out:Ljava/io/PrintStream;
//  #34 = Utf8               Code
//  #35 = Utf8               java/io/PrintStream
//  #36 = Utf8               Ljava/io/PrintStream;
//{
//  public LookupswitchDemo();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #30                 // Method java/lang/Object."<init>":()V
//         4: return
//
//  private int demo(int);
//    descriptor: (I)I
//    flags: (0x0002) ACC_PRIVATE
//    Code:
//      stack=3, locals=3, args_size=2
//         0: iload_1
//         1: lookupswitch  { // 2
//                       1: 28
//                      10: 31
//                 default: 34
//            }
//        28: ldc           #7                  // int 10
//        30: ireturn
//        31: ldc           #8                  // int 100
//        33: ireturn
//        34: ldc           #12                 // int 0
//        36: ireturn
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=3, locals=3, args_size=1
//         0: new           #5                  // class LookupswitchDemo
//         3: dup
//         4: invokespecial #31                 // Method "<init>":()V
//         7: astore_1
//         8: aload_1
//         9: ldc           #11                 // int 1
//        11: invokevirtual #17                 // Method demo:(I)I
//        14: jsr           54
//        17: aload_1
//        18: ldc           #7                  // int 10
//        20: invokevirtual #17                 // Method demo:(I)I
//        23: jsr           54
//        26: aload_1
//        27: ldc           #8                  // int 100
//        29: invokevirtual #17                 // Method demo:(I)I
//        32: jsr           54
//        35: aload_1
//        36: ldc           #1                  // int 200
//        38: invokevirtual #17                 // Method demo:(I)I
//        41: jsr           54
//        44: aload_1
//        45: ldc           #13                 // int 12345
//        47: invokevirtual #17                 // Method demo:(I)I
//        50: jsr           54
//        53: return
//        54: astore_2
//        55: getstatic     #33                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        58: swap
//        59: invokevirtual #26                 // Method java/io/PrintStream.println:(I)V
//        62: ret           2
//}
//SourceFile: "LookupSwitchDemo.pho"
#[test]
fn test_serialize_lookupswitchdemo() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x25, 0x03, 0x00, 0x00, 0x00, 0xc8,
        0x01, 0x00, 0x10, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68,
        0x44, 0x65, 0x6d, 0x6f, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x01, 0x00, 0x04, 0x6d, 0x61,
        0x69, 0x6e, 0x07, 0x00, 0x02, 0x0c, 0x00, 0x17, 0x00, 0x16, 0x03, 0x00, 0x00, 0x00, 0x0a,
        0x03, 0x00, 0x00, 0x00, 0x64, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61,
        0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x07, 0x00, 0x09, 0x03, 0x00, 0x00,
        0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x30, 0x39, 0x01, 0x00, 0x10,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65,
        0x6d, 0x07, 0x00, 0x0e, 0x0c, 0x00, 0x13, 0x00, 0x20, 0x0a, 0x00, 0x05, 0x00, 0x10, 0x01,
        0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
        0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x04, 0x64, 0x65, 0x6d,
        0x6f, 0x0c, 0x00, 0x19, 0x00, 0x03, 0x07, 0x00, 0x23, 0x01, 0x00, 0x04, 0x28, 0x49, 0x29,
        0x56, 0x01, 0x00, 0x07, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x6c, 0x6e, 0x01, 0x00, 0x0a, 0x53,
        0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e,
        0x69, 0x74, 0x3e, 0x0a, 0x00, 0x15, 0x00, 0x06, 0x0c, 0x00, 0x1c, 0x00, 0x24, 0x01, 0x00,
        0x03, 0x6f, 0x75, 0x74, 0x01, 0x00, 0x14, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x53, 0x77,
        0x69, 0x74, 0x63, 0x68, 0x44, 0x65, 0x6d, 0x6f, 0x2e, 0x70, 0x68, 0x6f, 0x0a, 0x00, 0x0a,
        0x00, 0x14, 0x0a, 0x00, 0x05, 0x00, 0x14, 0x01, 0x00, 0x04, 0x28, 0x49, 0x29, 0x49, 0x09,
        0x00, 0x0f, 0x00, 0x1b, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x13, 0x6a,
        0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72,
        0x65, 0x61, 0x6d, 0x01, 0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f,
        0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x00, 0x21, 0x00,
        0x05, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x19, 0x00, 0x03,
        0x00, 0x01, 0x00, 0x22, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x05, 0x2a, 0xb7, 0x00, 0x1e, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x13, 0x00,
        0x20, 0x00, 0x01, 0x00, 0x22, 0x00, 0x00, 0x00, 0x31, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00,
        0x00, 0x25, 0x1b, 0xab, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x1e,
        0x12, 0x07, 0xac, 0x12, 0x08, 0xac, 0x12, 0x0c, 0xac, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
        0x00, 0x04, 0x00, 0x12, 0x00, 0x01, 0x00, 0x22, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x03, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x40, 0xbb, 0x00, 0x05, 0x59, 0xb7, 0x00, 0x1f, 0x4c, 0x2b, 0x12,
        0x0b, 0xb6, 0x00, 0x11, 0xa8, 0x00, 0x28, 0x2b, 0x12, 0x07, 0xb6, 0x00, 0x11, 0xa8, 0x00,
        0x1f, 0x2b, 0x12, 0x08, 0xb6, 0x00, 0x11, 0xa8, 0x00, 0x16, 0x2b, 0x12, 0x01, 0xb6, 0x00,
        0x11, 0xa8, 0x00, 0x0d, 0x2b, 0x12, 0x0d, 0xb6, 0x00, 0x11, 0xa8, 0x00, 0x04, 0xb1, 0x4d,
        0xb2, 0x00, 0x21, 0x5f, 0xb6, 0x00, 0x1a, 0xa9, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x18, 0x00, 0x00, 0x00, 0x02, 0x00, 0x1d,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 3,
        major_version: 45,
        constant_pool_count: 37,
        constant_pool: vec![
            None,
            Some(ConstantIntegerInfo { tag: 3, bytes: 200 }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    76, 111, 111, 107, 117, 112, 115, 119, 105, 116, 99, 104, 68, 101, 109, 111,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 2,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 23,
                descriptor_index: 22,
            }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 10 }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 100 }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 9,
            }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 1 }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 0 }),
            Some(ConstantIntegerInfo {
                tag: 3,
                bytes: 12345,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 14,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 19,
                descriptor_index: 32,
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 5,
                name_and_type_index: 16,
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
                length: 4,
                bytes: vec![100, 101, 109, 111],
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 25,
                descriptor_index: 3,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 35,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![40, 73, 41, 86],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![60, 105, 110, 105, 116, 62],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 21,
                name_and_type_index: 6,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 28,
                descriptor_index: 36,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 20,
                bytes: vec![
                    76, 111, 111, 107, 117, 112, 83, 119, 105, 116, 99, 104, 68, 101, 109, 111, 46,
                    112, 104, 111,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 10,
                name_and_type_index: 20,
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 5,
                name_and_type_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![40, 73, 41, 73],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 15,
                name_and_type_index: 27,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 5,
        super_class: 10,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 3,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 25,
                descriptor_index: 3,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 34,
                    attribute_length: 17,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 30, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 2,
                name_index: 19,
                descriptor_index: 32,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 34,
                    attribute_length: 49,
                    max_stack: 3,
                    max_locals: 3,
                    code_length: 37,
                    code: vec![
                        27, 171, 0, 0, 0, 0, 0, 33, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 27, 0, 0, 0,
                        10, 0, 0, 0, 30, 18, 7, 172, 18, 8, 172, 18, 12, 172,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 4,
                descriptor_index: 18,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 34,
                    attribute_length: 76,
                    max_stack: 3,
                    max_locals: 3,
                    code_length: 64,
                    code: vec![
                        187, 0, 5, 89, 183, 0, 31, 76, 43, 18, 11, 182, 0, 17, 168, 0, 40, 43, 18,
                        7, 182, 0, 17, 168, 0, 31, 43, 18, 8, 182, 0, 17, 168, 0, 22, 43, 18, 1,
                        182, 0, 17, 168, 0, 13, 43, 18, 13, 182, 0, 17, 168, 0, 4, 177, 77, 178, 0,
                        33, 95, 182, 0, 26, 169, 2,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 24,
            attribute_length: 2,
            sourcefile_index: 29,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    serializer.serialize(&classfile)?;
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}

//Classfile /Users/z0ltan/dev/oyi-lang/phoron_asm/doc/grammar/TableswitchDemo.class
//  Last modified 07-Mar-2023; size 558 bytes
//  SHA-256 checksum fe89e7d28f29984e66a5a2b34c945f70077e82a7cd4c83eff63e70d14486210c
//  Compiled from "TableswitchDemo.pho"
//public class TableswitchDemo
//  minor version: 3
//  major version: 45
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #8                          // TableswitchDemo
//  super_class: #11                        // java/lang/Object
//  interfaces: 0, fields: 0, methods: 3, attributes: 1
//Constant pool:
//   #1 = Integer            200
//   #2 = Integer            19
//   #3 = Utf8               ()V
//   #4 = Utf8               main
//   #5 = Integer            199
//   #6 = NameAndType        #28:#26        // println:(I)V
//   #7 = Integer            100
//   #8 = Class              #16            // TableswitchDemo
//   #9 = Utf8               java/lang/Object
//  #10 = Methodref          #8.#19         // TableswitchDemo.demo:(I)I
//  #11 = Class              #9             // java/lang/Object
//  #12 = Integer            2
//  #13 = Integer            1
//  #14 = Integer            0
//  #15 = Integer            12345
//  #16 = Utf8               TableswitchDemo
//  #17 = Utf8               java/lang/System
//  #18 = Class              #17            // java/lang/System
//  #19 = NameAndType        #23:#34        // demo:(I)I
//  #20 = Utf8               TableswitchDemo.pho
//  #21 = Methodref          #8.#24         // TableswitchDemo."<init>":()V
//  #22 = Utf8               ([Ljava/lang/String;)V
//  #23 = Utf8               demo
//  #24 = NameAndType        #29:#3         // "<init>":()V
//  #25 = Class              #37            // java/io/PrintStream
//  #26 = Utf8               (I)V
//  #27 = Utf8               SourceFile
//  #28 = Utf8               println
//  #29 = Utf8               <init>
//  #30 = Methodref          #25.#6         // java/io/PrintStream.println:(I)V
//  #31 = NameAndType        #32:#39        // out:Ljava/io/PrintStream;
//  #32 = Utf8               out
//  #33 = Methodref          #11.#24        // java/lang/Object."<init>":()V
//  #34 = Utf8               (I)I
//  #35 = Fieldref           #18.#31        // java/lang/System.out:Ljava/io/PrintStream;
//  #36 = Utf8               Code
//  #37 = Utf8               java/io/PrintStream
//  #38 = Integer            1009
//  #39 = Utf8               Ljava/io/PrintStream;
//{
//  public TableswitchDemo();
//    descriptor: ()V
//    flags: (0x0001) ACC_PUBLIC
//    Code:
//      stack=1, locals=1, args_size=1
//         0: aload_0
//         1: invokespecial #33                 // Method java/lang/Object."<init>":()V
//         4: return
//
//  private int demo(int);
//    descriptor: (I)I
//    flags: (0x0002) ACC_PRIVATE
//    Code:
//      stack=3, locals=3, args_size=2
//         0: iload_1
//         1: tableswitch   { // 0 to 2
//                       0: 28
//                       1: 31
//                       2: 34
//                 default: 37
//            }
//        28: ldc           #2                  // int 19
//        30: ireturn
//        31: ldc           #5                  // int 199
//        33: ireturn
//        34: ldc           #38                 // int 1009
//        36: ireturn
//        37: ldc           #14                 // int 0
//        39: ireturn
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=3, locals=3, args_size=1
//         0: new           #8                  // class TableswitchDemo
//         3: dup
//         4: invokespecial #21                 // Method "<init>":()V
//         7: astore_1
//         8: aload_1
//         9: ldc           #14                 // int 0
//        11: invokevirtual #10                 // Method demo:(I)I
//        14: jsr           63
//        17: aload_1
//        18: ldc           #13                 // int 1
//        20: invokevirtual #10                 // Method demo:(I)I
//        23: jsr           63
//        26: aload_1
//        27: ldc           #12                 // int 2
//        29: invokevirtual #10                 // Method demo:(I)I
//        32: jsr           63
//        35: aload_1
//        36: ldc           #7                  // int 100
//        38: invokevirtual #10                 // Method demo:(I)I
//        41: jsr           63
//        44: aload_1
//        45: ldc           #1                  // int 200
//        47: invokevirtual #10                 // Method demo:(I)I
//        50: jsr           63
//        53: aload_1
//        54: ldc           #15                 // int 12345
//        56: invokevirtual #10                 // Method demo:(I)I
//        59: jsr           63
//        62: return
//        63: astore_2
//        64: getstatic     #35                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        67: swap
//        68: invokevirtual #30                 // Method java/io/PrintStream.println:(I)V
//        71: ret           2
//}
//SourceFile: "TableswitchDemo.pho"
#[test]
fn test_serialize_tableswitchdemo() -> SerializerResult {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x28, 0x03, 0x00, 0x00, 0x00, 0xc8,
        0x03, 0x00, 0x00, 0x00, 0x13, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x01, 0x00, 0x04, 0x6d,
        0x61, 0x69, 0x6e, 0x03, 0x00, 0x00, 0x00, 0xc7, 0x0c, 0x00, 0x1c, 0x00, 0x1a, 0x03, 0x00,
        0x00, 0x00, 0x64, 0x07, 0x00, 0x10, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x00, 0x08, 0x00, 0x13,
        0x07, 0x00, 0x09, 0x03, 0x00, 0x00, 0x00, 0x02, 0x03, 0x00, 0x00, 0x00, 0x01, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x30, 0x39, 0x01, 0x00, 0x0f, 0x54, 0x61, 0x62, 0x6c,
        0x65, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68, 0x44, 0x65, 0x6d, 0x6f, 0x01, 0x00, 0x10, 0x6a,
        0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d,
        0x07, 0x00, 0x11, 0x0c, 0x00, 0x17, 0x00, 0x22, 0x01, 0x00, 0x13, 0x54, 0x61, 0x62, 0x6c,
        0x65, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68, 0x44, 0x65, 0x6d, 0x6f, 0x2e, 0x70, 0x68, 0x6f,
        0x0a, 0x00, 0x08, 0x00, 0x18, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56,
        0x01, 0x00, 0x04, 0x64, 0x65, 0x6d, 0x6f, 0x0c, 0x00, 0x1d, 0x00, 0x03, 0x07, 0x00, 0x25,
        0x01, 0x00, 0x04, 0x28, 0x49, 0x29, 0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63,
        0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x07, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x6c, 0x6e,
        0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x0a, 0x00, 0x19, 0x00, 0x06, 0x0c,
        0x00, 0x20, 0x00, 0x27, 0x01, 0x00, 0x03, 0x6f, 0x75, 0x74, 0x0a, 0x00, 0x0b, 0x00, 0x18,
        0x01, 0x00, 0x04, 0x28, 0x49, 0x29, 0x49, 0x09, 0x00, 0x12, 0x00, 0x1f, 0x01, 0x00, 0x04,
        0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f,
        0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x03, 0x00, 0x00, 0x03,
        0xf1, 0x01, 0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72,
        0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x00, 0x21, 0x00, 0x08, 0x00,
        0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x1d, 0x00, 0x03, 0x00, 0x01,
        0x00, 0x24, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a,
        0xb7, 0x00, 0x21, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x17, 0x00, 0x22, 0x00,
        0x01, 0x00, 0x24, 0x00, 0x00, 0x00, 0x34, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x28,
        0x1b, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x21, 0x12, 0x02,
        0xac, 0x12, 0x05, 0xac, 0x12, 0x26, 0xac, 0x12, 0x0e, 0xac, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x09, 0x00, 0x04, 0x00, 0x16, 0x00, 0x01, 0x00, 0x24, 0x00, 0x00, 0x00, 0x55, 0x00, 0x03,
        0x00, 0x03, 0x00, 0x00, 0x00, 0x49, 0xbb, 0x00, 0x08, 0x59, 0xb7, 0x00, 0x15, 0x4c, 0x2b,
        0x12, 0x0e, 0xb6, 0x00, 0x0a, 0xa8, 0x00, 0x31, 0x2b, 0x12, 0x0d, 0xb6, 0x00, 0x0a, 0xa8,
        0x00, 0x28, 0x2b, 0x12, 0x0c, 0xb6, 0x00, 0x0a, 0xa8, 0x00, 0x1f, 0x2b, 0x12, 0x07, 0xb6,
        0x00, 0x0a, 0xa8, 0x00, 0x16, 0x2b, 0x12, 0x01, 0xb6, 0x00, 0x0a, 0xa8, 0x00, 0x0d, 0x2b,
        0x12, 0x0f, 0xb6, 0x00, 0x0a, 0xa8, 0x00, 0x04, 0xb1, 0x4d, 0xb2, 0x00, 0x23, 0x5f, 0xb6,
        0x00, 0x1e, 0xa9, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1b, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x14,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 3,
        major_version: 45,
        constant_pool_count: 40,
        constant_pool: vec![
            None,
            Some(ConstantIntegerInfo { tag: 3, bytes: 200 }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 19 }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
            }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 199 }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 28,
                descriptor_index: 26,
            }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 100 }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 16,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 8,
                name_and_type_index: 19,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 9,
            }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 2 }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 1 }),
            Some(ConstantIntegerInfo { tag: 3, bytes: 0 }),
            Some(ConstantIntegerInfo {
                tag: 3,
                bytes: 12345,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    84, 97, 98, 108, 101, 115, 119, 105, 116, 99, 104, 68, 101, 109, 111,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 17,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 23,
                descriptor_index: 34,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    84, 97, 98, 108, 101, 115, 119, 105, 116, 99, 104, 68, 101, 109, 111, 46, 112,
                    104, 111,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 8,
                name_and_type_index: 24,
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
                length: 4,
                bytes: vec![100, 101, 109, 111],
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 29,
                descriptor_index: 3,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 37,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![40, 73, 41, 86],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![60, 105, 110, 105, 116, 62],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 25,
                name_and_type_index: 6,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 32,
                descriptor_index: 39,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 11,
                name_and_type_index: 24,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![40, 73, 41, 73],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 18,
                name_and_type_index: 31,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantIntegerInfo {
                tag: 3,
                bytes: 1009,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 8,
        super_class: 11,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 3,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 29,
                descriptor_index: 3,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 36,
                    attribute_length: 17,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 33, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 2,
                name_index: 23,
                descriptor_index: 34,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 36,
                    attribute_length: 52,
                    max_stack: 3,
                    max_locals: 3,
                    code_length: 40,
                    code: vec![
                        27, 170, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 27, 0, 0, 0,
                        30, 0, 0, 0, 33, 18, 2, 172, 18, 5, 172, 18, 38, 172, 18, 14, 172,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 4,
                descriptor_index: 22,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 36,
                    attribute_length: 85,
                    max_stack: 3,
                    max_locals: 3,
                    code_length: 73,
                    code: vec![
                        187, 0, 8, 89, 183, 0, 21, 76, 43, 18, 14, 182, 0, 10, 168, 0, 49, 43, 18,
                        13, 182, 0, 10, 168, 0, 40, 43, 18, 12, 182, 0, 10, 168, 0, 31, 43, 18, 7,
                        182, 0, 10, 168, 0, 22, 43, 18, 1, 182, 0, 10, 168, 0, 13, 43, 18, 15, 182,
                        0, 10, 168, 0, 4, 177, 77, 178, 0, 35, 95, 182, 0, 30, 169, 2,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 0,
                    code_attributes: vec![],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 27,
            attribute_length: 2,
            sourcefile_index: 20,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    serializer.serialize(&classfile)?;
    assert_eq!(expected_bytes, &bytes[..]);

    Ok(())
}
