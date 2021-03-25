use quick_js::JsValue;
use std::sync::{Arc, Mutex};

pub type JsMetaHandle = Arc<Mutex<JsMeta>>;

#[derive(Debug, Clone)]
pub struct JsMeta {
    pub(crate) main_id: Option<MethodId>,
    pub(crate) types: Vec<TypeDefinition>,
    pub(crate) methods: Vec<MethodDefinition>,
}

pub type TypeId = i32;
pub type MethodId = i32;
pub type BlockId = i32;

impl JsMeta {
    pub fn add_type(&mut self, signed: bool, bits: i32) -> TypeId {
        if bits < 0 {
            panic!("expected >= 0 bits");
        }

        let type_id = self.types.len();
        self.types.push(TypeDefinition {
            signed,
            bits: bits as u32,
        });

        type_id as TypeId
    }

    pub fn add_method(
        &mut self,
        name: String,
        return_type: TypeId,
        parameters: Vec<TypeId>,
    ) -> MethodId {
        // ensure that the types specified exist
        self.get_type(return_type);
        parameters
            .iter()
            .map(|id| self.get_type(*id))
            .for_each(drop);

        let method_id = self.methods.len();
        self.methods.push(MethodDefinition {
            name,
            return_type,
            parameters,
            blocks: vec![],
        });

        method_id as MethodId
    }

    pub fn get_type<'a>(&'a self, id: TypeId) -> &'a TypeDefinition {
        self.types.get(id as usize).expect("expected type")
    }

    pub fn get_method_mut<'a>(&'a mut self, id: MethodId) -> &'a mut MethodDefinition {
        self.methods.get_mut(id as usize).expect("expected type")
    }
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub(crate) signed: bool,
    pub(crate) bits: u32,
}

#[derive(Debug, Clone)]
pub struct MethodDefinition {
    pub(crate) name: String,
    pub(crate) return_type: TypeId,
    pub(crate) parameters: Vec<TypeId>,
    pub(crate) blocks: Vec<BlockDefinition>,
}

impl MethodDefinition {
    pub fn add_block(&mut self, name: String) -> BlockId {
        let block_id = self.blocks.len();
        self.blocks.push(BlockDefinition {
            name,
            instructions: vec![],
        });

        block_id as BlockId
    }

    pub fn get_block_mut<'a>(&'a mut self, id: BlockId) -> &'a mut BlockDefinition {
        self.blocks.get_mut(id as usize).expect("expected block")
    }
}

#[derive(Debug, Clone)]
pub struct BlockDefinition {
    pub(crate) name: String,
    pub(crate) instructions: Vec<Instruction>,
}

type Register = i32;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add {
        result: Register,
        a: Register,
        b: Register,
    },
    LoadParameter {
        result: Register,
        parameter_number: i32,
    },
    LoadConstant {
        result: Register,
        type_id: TypeId,
        constant: Constant,
    },
    Call {
        result: Register,
        method_id: MethodId,
        parameters: Vec<Register>,
    },
    Return {
        result: Option<Register>,
    },
}

impl Instruction {
    pub fn deserialize<F: Fn(TypeId) -> bool>(
        parameters: usize,
        is_valid: F,
        name: String,
        args: Vec<JsValue>,
    ) -> Instruction {
        println!("DES: {:?} {:?}", name, args);

        match name.as_str() {
            "add" => Instruction::des_add(args),
            "ld_param" => Instruction::des_ld_param(parameters, args),
            "ld_const" => Instruction::des_ld_const(is_valid, args),
            "call" => Instruction::des_call(args),
            "ret" => Instruction::des_ret(args),
            _ => panic!("unrecognized instruction {}", name),
        }
    }

    fn des_add(mut args: Vec<JsValue>) -> Instruction {
        let b = Instruction::get_register(args.pop().unwrap());
        let a = Instruction::get_register(args.pop().unwrap());
        let result = Instruction::get_register(args.pop().unwrap());

        Instruction::Add { result, a, b }
    }

    fn des_ld_param(parameters: usize, mut args: Vec<JsValue>) -> Instruction {
        let parameter_number = Instruction::get_number(args.pop().unwrap());
        let result = Instruction::get_register(args.pop().unwrap());

        if parameter_number < 0 || parameter_number as usize > parameters {
            panic!(
                "parameter number out of bounds - mathematical range: [0, {}], value: {}",
                parameters, parameter_number
            );
        }

        Instruction::LoadParameter {
            result,
            parameter_number,
        }
    }

    fn des_ld_const<F: Fn(TypeId) -> bool>(is_valid: F, mut args: Vec<JsValue>) -> Instruction {
        let constant = Instruction::get_value(args.pop().unwrap());
        let type_id = Instruction::get_type_id(args.pop().unwrap());
        let result = Instruction::get_register(args.pop().unwrap());

        if !is_valid(type_id) {
            panic!("invalid type {}", type_id);
        }

        Instruction::LoadConstant {
            result,
            type_id,
            constant,
        }
    }

    fn des_call(mut args: Vec<JsValue>) -> Instruction {
        let parameters = Instruction::get_arr(args.pop().unwrap())
            .into_iter()
            .map(Instruction::get_register)
            .collect::<Vec<_>>();
        let method_id = Instruction::get_method_id(args.pop().unwrap());
        let result = Instruction::get_register(args.pop().unwrap());

        eprintln!("// TODO: sanitize deserialize call");

        Instruction::Call {
            result,
            method_id,
            parameters,
        }
    }

    fn des_ret(mut args: Vec<JsValue>) -> Instruction {
        Instruction::Return {
            result: args.pop().map(Instruction::get_register),
        }
    }

    fn get_register(arg: JsValue) -> Register {
        match arg {
            JsValue::Int(i) => i,
            _ => panic!("unable to get register from arg {:?}", arg),
        }
    }

    fn get_number(arg: JsValue) -> i32 {
        match arg {
            JsValue::Int(i) => i,
            _ => panic!("unable to get number from arg {:?}", arg),
        }
    }

    fn get_type_id(arg: JsValue) -> TypeId {
        match arg {
            JsValue::Int(i) => i,
            _ => panic!("unable to get register from arg {:?}", arg),
        }
    }

    fn get_value(arg: JsValue) -> Constant {
        match arg {
            JsValue::Int(i) => Constant::Number(i),
            _ => panic!("unable to get constant from arg {:?}", arg),
        }
    }

    fn get_arr(arg: JsValue) -> Vec<JsValue> {
        match arg {
            JsValue::Array(values) => values,
            _ => panic!("unable to get array from arg {:?}", arg),
        }
    }

    fn get_method_id(arg: JsValue) -> MethodId {
        match arg {
            JsValue::Int(i) => i,
            _ => panic!("unable to get method from arg {:?}", arg),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Number(i32),
}

pub fn hook(context: &mut quick_js::Context) -> JsMetaHandle {
    let primer = include_str!("./primer.js");
    let source_meta = Arc::new(Mutex::new(JsMeta {
        main_id: None,
        types: vec![],
        methods: vec![],
    }));

    let meta = source_meta.clone();
    context
        .add_callback("__compiler_type", move |signed: bool, bits: i32| {
            let mut meta = meta.lock().unwrap();

            let type_id = meta.add_type(signed, bits);

            JsValue::Int(type_id)
        })
        .expect("expected to inject __compiler_type");

    let meta = source_meta.clone();
    context
        .add_callback("__compiler_set_main", move |method_id: MethodId| {
            let mut meta = meta.lock().unwrap();

            // ensure the method exists
            meta.get_method_mut(method_id);

            meta.main_id = Some(method_id);

            JsValue::Undefined
        })
        .expect("expected to inject __compiler_set_main");

    let meta = source_meta.clone();
    context
        .add_callback(
            "__compiler_generate_method",
            move |name: String, return_type: TypeId, parameters: Vec<TypeId>| {
                let mut meta = meta.lock().unwrap();

                let method_id = meta.add_method(name, return_type, parameters);

                JsValue::Int(method_id)
            },
        )
        .expect("expected to inject __compiler_generate_method");

    let meta = source_meta.clone();
    context
        .add_callback(
            "__compiler_generate_block",
            move |method_id: MethodId, name: String| {
                let mut meta = meta.lock().unwrap();

                let method = meta.get_method_mut(method_id);
                let block_id = method.add_block(name);

                JsValue::Int(block_id)
            },
        )
        .expect("expected to inject __compiler_generate_method");

    let meta = source_meta.clone();
    context
        .add_callback(
            "__compiler_emit",
            move |method_id: MethodId,
                  block_id: BlockId,
                  instruction: String,
                  values: Vec<JsValue>| {
                let mut meta = meta.lock().unwrap();

                let method = meta.get_method_mut(method_id);
                let instruction = Instruction::deserialize(
                    method.parameters.len(),
                    |_| {
                        eprintln!("TODO: verify method id");
                        true
                    },
                    instruction,
                    values,
                );

                let block = method.get_block_mut(block_id);
                block.instructions.push(instruction);

                JsValue::Undefined
            },
        )
        .expect("expected to inject __compiler_generate_method");

    context
        .eval(primer)
        .expect("expected primer to inject okay");

    source_meta
}
