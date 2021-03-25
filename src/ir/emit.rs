use std::collections::HashMap;

use super::js_boundary::JsMetaHandle;
use crate::ir::js_boundary::{Constant, Instruction};
use inkwell::{
    context::Context,
    module::{Linkage, Module},
    types::{FunctionType, IntType},
    values::BasicValue,
    values::{FunctionValue, IntValue},
};

pub struct LLVMType<'ctx> {
    int_type: IntType<'ctx>,
    signed: bool,
}

pub struct LLVMMethod<'ctx> {
    method_type: FunctionType<'ctx>,
    method_impl: FunctionValue<'ctx>,
}

pub fn hydrate<'ctx>(meta: &JsMetaHandle, context: &'ctx Context, module: &mut Module<'ctx>) {
    let meta = meta.lock().unwrap();

    // populate types
    let mut llvm_types = Vec::new();
    for r#type in meta.types.iter() {
        llvm_types.push(LLVMType {
            signed: r#type.signed,
            int_type: context.custom_width_int_type(r#type.bits),
        });
    }

    // populate methods *declarations*
    let mut llvm_methods = Vec::new();
    for method in meta.methods.iter() {
        let return_type = &llvm_types[method.return_type as usize];
        let parameter_types = method
            .parameters
            .iter()
            .map(|p| &llvm_types[*p as usize])
            .map(|t| t.int_type)
            .map(|i| i.into())
            .collect::<Vec<_>>();

        let fn_type = return_type
            .int_type
            .fn_type(parameter_types.as_slice(), false);

        let function = module.add_function(
            format!("tinkerlang_{}", method.name).as_str(),
            fn_type,
            None,
        );

        llvm_methods.push(LLVMMethod {
            method_type: fn_type,
            method_impl: function,
        })
    }

    // emit entrypoint
    let main = &llvm_methods[meta
        .main_id
        .map(|v| v as usize)
        .expect("expected main method to be set")];

    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, Some(Linkage::External));
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    let retval = builder
        .build_call(main.method_impl, &[], "call")
        .try_as_basic_value()
        .unwrap_left();
    retval.set_name("retval");

    builder.build_return(Some(&retval));

    // emit method declarations
    for (id, llvm_method) in llvm_methods.iter().enumerate() {
        let source = &meta.methods[id];

        for block in source.blocks.iter() {
            let llvm_block =
                context.append_basic_block(llvm_method.method_impl, block.name.as_str());
            builder.position_at_end(llvm_block);

            let mut registers = HashMap::new();

            for inst in block.instructions.iter() {
                match inst {
                    Instruction::Add { result, a, b } => {
                        let a = registers.get(a).unwrap();
                        let b = registers.get(b).unwrap();

                        let result_reg = builder.build_int_add(*a, *b, "");
                        registers.insert(*result, result_reg);
                    }
                    Instruction::LoadParameter {
                        result,
                        parameter_number,
                    } => {
                        let param = llvm_method
                            .method_impl
                            .get_nth_param(*parameter_number as u32)
                            .unwrap();
                        let param = param.into_int_value();

                        let source_param_id = source.parameters[*parameter_number as usize];
                        let param_type = &llvm_types[source_param_id as usize];

                        let result_reg =
                            builder.build_int_add(param_type.int_type.const_zero(), param, "");
                        registers.insert(*result, result_reg);
                    }
                    Instruction::LoadConstant {
                        result,
                        type_id,
                        constant,
                    } => match constant {
                        Constant::Number(number) => {
                            let llvm_type = &llvm_types[*type_id as usize];
                            let value = llvm_type.int_type.const_int(*number as u64, *number < 0);
                            let const_reg =
                                builder.build_int_add(llvm_type.int_type.const_zero(), value, "");
                            registers.insert(*result, const_reg);
                        }
                        _ => todo!(),
                    },
                    Instruction::Call {
                        result,
                        method_id,
                        parameters,
                    } => {
                        let function = &llvm_methods[*method_id as usize];
                        let parameters = parameters
                            .iter()
                            .map(|r| registers.get(r).unwrap().as_basic_value_enum())
                            .collect::<Vec<_>>();

                        let result_reg =
                            builder.build_call(function.method_impl, parameters.as_slice(), "");
                        registers.insert(
                            *result,
                            result_reg
                                .try_as_basic_value()
                                .unwrap_left()
                                .into_int_value(),
                        );
                    }
                    Instruction::Return { result } => {
                        let ret = result
                            .and_then(|register| registers.get(&register))
                            .map(|v| -> Box<dyn BasicValue> { Box::new(*v) });

                        builder.build_return(ret.as_deref());
                    }
                    _ => panic!("unhandled instruction"),
                }
            }
        }
    }

    println!(" ==> HYDRATION ==>");
    println!("JsMeta: {:#?}", meta);
    println!(" <== HYDRATION <==");
}
