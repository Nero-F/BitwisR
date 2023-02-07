// #[path = "../src/binary_operation.rs"]
// mod boi;

// #[test]
// fn test_op_and() {
//     let input = "12&13";
//     let mut op_interpreter = boi::OperationInterpreter::new();
//
//     op_interpreter.lexer(input);
//     op_interpreter.parser().unwrap();
//     op_interpreter.interpreter();
//     assert_eq!(op_interpreter.result.res,
//                ["#", "   12  00001100  0xc",
//                      "&  13  00001101  0xd",
//                      "────────────────────",
//                      "=  12  00001100  0xc"]);
// }
//
// #[test]
// fn test_op_or() {
//     let input = "12|13";
//     let mut op_interpreter = boi::OperationInterpreter::new();
//
//     op_interpreter.lexer(input);
//     op_interpreter.parser().unwrap();
//     op_interpreter.interpreter();
//     assert_eq!(op_interpreter.result.res,
//                ["#", "   12  00001100  0xc",
//                      "|  13  00001101  0xd",
//                      "────────────────────",
//                      "=  13  00001101  0xd"]);
// }
//
// #[test]
// fn test_op_xor() {
//     let input = "12^13";
//     let mut op_interpreter = boi::OperationInterpreter::new();
//
//     op_interpreter.lexer(input);
//     op_interpreter.parser().unwrap();
//     op_interpreter.interpreter();
//     assert_eq!(op_interpreter.result.res,
//                ["#", "   12  00001100  0xc",
//                      "^  13  00001101  0xd",
//                      "────────────────────",
//                      "=   1  00000001  0x1"]);
// }
//
// #[test]
// fn test_num_chained() {
//     let input = "12 13 14";
//     let mut op_interpreter = boi::OperationInterpreter::new();
//
//     op_interpreter.lexer(input);
//     op_interpreter.parser().unwrap();
//     op_interpreter.interpreter();
//     assert_eq!(op_interpreter.result.res,
//                ["#", "✪  14  00001110  0xe",
//                "#", "✪  13  00001101  0xd",
//                "#", "✪  12  00001100  0xc"]);
// }
//
// //#[test]
// //fn test_hexnum_chained() {
//     //let input = "0x12 0x13 0x14";
//     //let mut op_interpreter = boi::OperationInterpreter::new();
//
//     //op_interpreter.lexer(input);
//     //op_interpreter.parser().unwrap();
//     //op_interpreter.interpreter();
//     //assert_eq!(op_interpreter.result.res,
//                //["#", "✪  0x14  00000000 00000000 00000000 00010100  20",
//                //"✪  0x13  00000000 00000000 00000000 00010011  19",
//                //"✪  0x12  00000000 00000000 00000000 00010010  18"]);
// //}
//
// #[test]
// fn test_op_not() {
//     let input = "~12";
//     let mut op_interpreter = boi::OperationInterpreter::new();
//
//     op_interpreter.lexer(input);
//     op_interpreter.parser().unwrap();
//     op_interpreter.interpreter();
//     assert_eq!(op_interpreter.result.res,
//                ["#", 
//                 "~  12  00001100  0xc",
//                 "────────────────────",
//                 "= -13  11110011 -0xd" ]);
// }
