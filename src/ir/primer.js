/*
 * Primer
 * ===
 * 
 * Boilerplate code to wrap around compiler builtins to make the creation of
 * functions, modules, etc. easier
 *
 */

//==={

/**
 * @typedef {object} TreeNode
 * @property {string} type
 * @property {unknown} value
 * @property {Node[]} children
 */

/** @type {TreeNode} */
var tree;

/** @typedef {(value: unknown) => string} ToValue */
/** @type {ToValue} */
var toValue;

/** @typedef {{__typeid_FAKE_FOR_SAKE_OF_TYPES: unknown}} TypeId */

/** @typedef {(signed: boolean, size: number) => TypeId} CompilerType */
/** @type {CompilerType} */
var __compiler_type;

/** @typedef {{__methodid_FAKE_FOR_SAKE_OF_TYPES: unknown}} MethodId */

/** @typedef {(name: string, return_type: TypeId, parameters: TypeId[]) => MethodId} CompilerGenerateMethod */
/** @type {CompilerGenerateMethod} */
var __compiler_generate_method;

/** @typedef {(method_id: MethodId) => void} CompilerSetMain */
/** @type {CompilerSetMain} */
var __compiler_set_main;

/** @typedef {{__blockid_FAKE_FOR_SAKE_OF_TYPES: unknown}} BlockId */

/** @typedef {(methodId: MethodId, name: string) => BlockId} CompilerGenerateBlock */
/** @type {CompilerGenerateBlock} */
var __compiler_generate_block;

/** @typedef {(methodId: MethodId, blockId: BlockId, instruction: string, values: any[])} CompilerEmit */
/** @type {CompilerEmit} */
var __compiler_emit;

/** @typedef {number} Register */

//===}

const { context, bool, i8, i16, i32, i64, u8, u16, u32, u64 } = (() => {
    class Block {
        /**
         * @param {MethodId} methodId
         * @param {string} blockId
         */
        constructor(methodId, blockId) {
            this.methodId = methodId;
            this.blockId = blockId;
            this.emit = __compiler_emit.bind(undefined, this.methodId, this.blockId);
        }

        /**
         * @param {Register} rResult
         * @param {Register} rA
         * @param {Register} rB
         */
        add(rResult, rA, rB) {
            this.emit("add", [rResult, rA, rB]);
            return this;
        }

        /**
         * @param {Register} rResult
         * @param {number} paramNum
         */
        ld_param(rResult, paramNum) {
            this.emit("ld_param", [rResult, paramNum]);
            return this;
        }

        /**
         * @param {Registter} rResult
         * @param {TypeId} type
         * @param {*} value
         */
        ld_const(rResult, type, value) {
            this.emit("ld_const", [rResult, type, value]);
            return this;
        }

        /**
         * @param {Register} rResult 
         * @param {MethodId} method 
         * @param {Register[]} rParams 
         */
        call(rResult, method, rParams) {
            this.emit("call", [rResult, method, rParams]);
            return this;
        }

        /**
         * @param {Register | undefined} rResult 
         */
        ret(rResult) {
            this.emit("ret", [rResult]);
            return this;
        }
    }

    class Method {
        /**
         * @param {MethodId} id
         */
        constructor(id) {
            this.id = id;
        }

        /**
         * @param {string} name
         */
        block(name) {
            return new Block(this.id, __compiler_generate_block(this.id, name));
        }
    }

    const context = new class Context {
        /**
         * @param {string} name
         * @param {TypeId} returnType
         * @param {TypeId[]} parameters
         */
        method(name, returnType, parameters) {
            return new Method(__compiler_generate_method(name, returnType, parameters));
        }

        /**
         * @param {Method} method
         */
        setMain(method) {
            __compiler_set_main(method.id);
        }
    }();

    return {
        context,
        bool: __compiler_type(false, 1),
        i8: __compiler_type(true, 8),
        u8: __compiler_type(false, 8),
        i16: __compiler_type(true, 16),
        i32: __compiler_type(true, 32),
        i64: __compiler_type(true, 64),
        u16: __compiler_type(false, 16),
        u32: __compiler_type(false, 32),
        u64: __compiler_type(false, 64),
    };
})(this);
