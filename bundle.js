/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("{__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_embedded_engine_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pkg/embedded_engine.js */ \"./pkg/embedded_engine.js\");\n\n\n// Prevent arrow keys from scrolling the page\nwindow.addEventListener(\"keydown\", (event) => {\n    switch (event.code) {\n        case \"ArrowUp\":\n        case \"ArrowDown\":\n        case \"Space\":\n            event.preventDefault(); // stop scrolling\n            break;\n    }\n});\n\nasync function run() {\n    await (0,_pkg_embedded_engine_js__WEBPACK_IMPORTED_MODULE_0__[\"default\"])();\n    await (0,_pkg_embedded_engine_js__WEBPACK_IMPORTED_MODULE_0__.wasm_main)();\n}\n\nrun().catch(console.error);\n\n\n//# sourceURL=webpack:///./index.js?\n}");

/***/ }),

/***/ "./pkg/embedded_engine.js":
/*!********************************!*\
  !*** ./pkg/embedded_engine.js ***!
  \********************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

eval("{__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__),\n/* harmony export */   initSync: () => (/* binding */ initSync),\n/* harmony export */   wasm_main: () => (/* binding */ wasm_main)\n/* harmony export */ });\nlet wasm;\n\nconst cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );\n\nif (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };\n\nlet cachedUint8ArrayMemory0 = null;\n\nfunction getUint8ArrayMemory0() {\n    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {\n        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8ArrayMemory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));\n}\n\nfunction addToExternrefTable0(obj) {\n    const idx = wasm.__externref_table_alloc();\n    wasm.__wbindgen_export_2.set(idx, obj);\n    return idx;\n}\n\nfunction handleError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        const idx = addToExternrefTable0(e);\n        wasm.__wbindgen_exn_store(idx);\n    }\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8ArrayMemory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n        ptr = realloc(ptr, len, offset, 1) >>> 0;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedDataViewMemory0 = null;\n\nfunction getDataViewMemory0() {\n    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {\n        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);\n    }\n    return cachedDataViewMemory0;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nconst CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(state => {\n    wasm.__wbindgen_export_5.get(state.dtor)(state.a, state.b)\n});\n\nfunction makeMutClosure(arg0, arg1, dtor, f) {\n    const state = { a: arg0, b: arg1, cnt: 1, dtor };\n    const real = (...args) => {\n        // First up with a closure we increment the internal reference\n        // count. This ensures that the Rust closure environment won't\n        // be deallocated while we're invoking it.\n        state.cnt++;\n        const a = state.a;\n        state.a = 0;\n        try {\n            return f(a, state.b, ...args);\n        } finally {\n            if (--state.cnt === 0) {\n                wasm.__wbindgen_export_5.get(state.dtor)(a, state.b);\n                CLOSURE_DTORS.unregister(state);\n            } else {\n                state.a = a;\n            }\n        }\n    };\n    real.original = state;\n    CLOSURE_DTORS.register(real, state, state);\n    return real;\n}\n\nfunction debugString(val) {\n    // primitive types\n    const type = typeof val;\n    if (type == 'number' || type == 'boolean' || val == null) {\n        return  `${val}`;\n    }\n    if (type == 'string') {\n        return `\"${val}\"`;\n    }\n    if (type == 'symbol') {\n        const description = val.description;\n        if (description == null) {\n            return 'Symbol';\n        } else {\n            return `Symbol(${description})`;\n        }\n    }\n    if (type == 'function') {\n        const name = val.name;\n        if (typeof name == 'string' && name.length > 0) {\n            return `Function(${name})`;\n        } else {\n            return 'Function';\n        }\n    }\n    // objects\n    if (Array.isArray(val)) {\n        const length = val.length;\n        let debug = '[';\n        if (length > 0) {\n            debug += debugString(val[0]);\n        }\n        for(let i = 1; i < length; i++) {\n            debug += ', ' + debugString(val[i]);\n        }\n        debug += ']';\n        return debug;\n    }\n    // Test for built-in\n    const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));\n    let className;\n    if (builtInMatches && builtInMatches.length > 1) {\n        className = builtInMatches[1];\n    } else {\n        // Failed to match the standard '[object ClassName]'\n        return toString.call(val);\n    }\n    if (className == 'Object') {\n        // we're a user defined class or Object\n        // JSON.stringify avoids problems with cycles, and is generally much\n        // easier than looping through ownProperties of `val`.\n        try {\n            return 'Object(' + JSON.stringify(val) + ')';\n        } catch (_) {\n            return 'Object';\n        }\n    }\n    // errors\n    if (val instanceof Error) {\n        return `${val.name}: ${val.message}\\n${val.stack}`;\n    }\n    // TODO we could test for more things here, like `Set`s and `Map`s.\n    return className;\n}\n/**\n * @returns {Promise<void>}\n */\nfunction wasm_main() {\n    const ret = wasm.wasm_main();\n    return ret;\n}\n\nfunction __wbg_adapter_22(arg0, arg1) {\n    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h72d53d16c05dfc45(arg0, arg1);\n}\n\nfunction __wbg_adapter_25(arg0, arg1, arg2) {\n    wasm.closure46_externref_shim(arg0, arg1, arg2);\n}\n\nfunction __wbg_adapter_28(arg0, arg1, arg2) {\n    wasm.closure58_externref_shim(arg0, arg1, arg2);\n}\n\nfunction __wbg_adapter_96(arg0, arg1, arg2, arg3) {\n    wasm.closure67_externref_shim(arg0, arg1, arg2, arg3);\n}\n\nasync function __wbg_load(module, imports) {\n    if (typeof Response === 'function' && module instanceof Response) {\n        if (typeof WebAssembly.instantiateStreaming === 'function') {\n            try {\n                return await WebAssembly.instantiateStreaming(module, imports);\n\n            } catch (e) {\n                if (module.headers.get('Content-Type') != 'application/wasm') {\n                    console.warn(\"`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\\n\", e);\n\n                } else {\n                    throw e;\n                }\n            }\n        }\n\n        const bytes = await module.arrayBuffer();\n        return await WebAssembly.instantiate(bytes, imports);\n\n    } else {\n        const instance = await WebAssembly.instantiate(module, imports);\n\n        if (instance instanceof WebAssembly.Instance) {\n            return { instance, module };\n\n        } else {\n            return instance;\n        }\n    }\n}\n\nfunction __wbg_get_imports() {\n    const imports = {};\n    imports.wbg = {};\n    imports.wbg.__wbg_addEventListener_84ae3eac6e15480a = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {\n        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);\n    }, arguments) };\n    imports.wbg.__wbg_call_672a4d21634d4a24 = function() { return handleError(function (arg0, arg1) {\n        const ret = arg0.call(arg1);\n        return ret;\n    }, arguments) };\n    imports.wbg.__wbg_call_7cccdd69e0791ae2 = function() { return handleError(function (arg0, arg1, arg2) {\n        const ret = arg0.call(arg1, arg2);\n        return ret;\n    }, arguments) };\n    imports.wbg.__wbg_clearTimeout_5a54f8841c30079a = function(arg0) {\n        const ret = clearTimeout(arg0);\n        return ret;\n    };\n    imports.wbg.__wbg_code_459c120478e1ab6e = function(arg0, arg1) {\n        const ret = arg1.code;\n        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len1 = WASM_VECTOR_LEN;\n        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);\n        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);\n    };\n    imports.wbg.__wbg_debug_3cb59063b29f58c1 = function(arg0) {\n        console.debug(arg0);\n    };\n    imports.wbg.__wbg_document_d249400bd7bd996d = function(arg0) {\n        const ret = arg0.document;\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_error_524f506f44df1645 = function(arg0) {\n        console.error(arg0);\n    };\n    imports.wbg.__wbg_fillRect_c38d5d56492a2368 = function(arg0, arg1, arg2, arg3, arg4) {\n        arg0.fillRect(arg1, arg2, arg3, arg4);\n    };\n    imports.wbg.__wbg_getContext_e9cf379449413580 = function() { return handleError(function (arg0, arg1, arg2) {\n        const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    }, arguments) };\n    imports.wbg.__wbg_getElementById_f827f0d6648718a8 = function(arg0, arg1, arg2) {\n        const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_info_3daf2e093e091b66 = function(arg0) {\n        console.info(arg0);\n    };\n    imports.wbg.__wbg_innerHeight_05f4225d754a7929 = function() { return handleError(function (arg0) {\n        const ret = arg0.innerHeight;\n        return ret;\n    }, arguments) };\n    imports.wbg.__wbg_innerWidth_7e0498dbd876d498 = function() { return handleError(function (arg0) {\n        const ret = arg0.innerWidth;\n        return ret;\n    }, arguments) };\n    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_df82a4d3437bf1cc = function(arg0) {\n        let result;\n        try {\n            result = arg0 instanceof CanvasRenderingContext2D;\n        } catch (_) {\n            result = false;\n        }\n        const ret = result;\n        return ret;\n    };\n    imports.wbg.__wbg_instanceof_HtmlCanvasElement_2ea67072a7624ac5 = function(arg0) {\n        let result;\n        try {\n            result = arg0 instanceof HTMLCanvasElement;\n        } catch (_) {\n            result = false;\n        }\n        const ret = result;\n        return ret;\n    };\n    imports.wbg.__wbg_instanceof_KeyboardEvent_1f79ad7d32051924 = function(arg0) {\n        let result;\n        try {\n            result = arg0 instanceof KeyboardEvent;\n        } catch (_) {\n            result = false;\n        }\n        const ret = result;\n        return ret;\n    };\n    imports.wbg.__wbg_instanceof_Window_def73ea0955fc569 = function(arg0) {\n        let result;\n        try {\n            result = arg0 instanceof Window;\n        } catch (_) {\n            result = false;\n        }\n        const ret = result;\n        return ret;\n    };\n    imports.wbg.__wbg_log_c222819a41e063d3 = function(arg0) {\n        console.log(arg0);\n    };\n    imports.wbg.__wbg_new_23a2665fac83c611 = function(arg0, arg1) {\n        try {\n            var state0 = {a: arg0, b: arg1};\n            var cb0 = (arg0, arg1) => {\n                const a = state0.a;\n                state0.a = 0;\n                try {\n                    return __wbg_adapter_96(a, state0.b, arg0, arg1);\n                } finally {\n                    state0.a = a;\n                }\n            };\n            const ret = new Promise(cb0);\n            return ret;\n        } finally {\n            state0.a = state0.b = 0;\n        }\n    };\n    imports.wbg.__wbg_new_405e22f390576ce2 = function() {\n        const ret = new Object();\n        return ret;\n    };\n    imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {\n        const ret = new Function(getStringFromWasm0(arg0, arg1));\n        return ret;\n    };\n    imports.wbg.__wbg_now_d18023d54d4e5500 = function(arg0) {\n        const ret = arg0.now();\n        return ret;\n    };\n    imports.wbg.__wbg_performance_c185c0cdc2766575 = function(arg0) {\n        const ret = arg0.performance;\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5 = function(arg0) {\n        queueMicrotask(arg0);\n    };\n    imports.wbg.__wbg_queueMicrotask_d3219def82552485 = function(arg0) {\n        const ret = arg0.queueMicrotask;\n        return ret;\n    };\n    imports.wbg.__wbg_resolve_4851785c9c5f573d = function(arg0) {\n        const ret = Promise.resolve(arg0);\n        return ret;\n    };\n    imports.wbg.__wbg_setTimeout_db2dbaeefb6f39c7 = function() { return handleError(function (arg0, arg1) {\n        const ret = setTimeout(arg0, arg1);\n        return ret;\n    }, arguments) };\n    imports.wbg.__wbg_setcapture_46bd7043887eba02 = function(arg0, arg1) {\n        arg0.capture = arg1 !== 0;\n    };\n    imports.wbg.__wbg_setfillStyle_2205fca942c641ba = function(arg0, arg1, arg2) {\n        arg0.fillStyle = getStringFromWasm0(arg1, arg2);\n    };\n    imports.wbg.__wbg_setheight_da683a33fa99843c = function(arg0, arg1) {\n        arg0.height = arg1 >>> 0;\n    };\n    imports.wbg.__wbg_setonce_0cb80aea26303a35 = function(arg0, arg1) {\n        arg0.once = arg1 !== 0;\n    };\n    imports.wbg.__wbg_setpassive_57a5a4c4b00a7c62 = function(arg0, arg1) {\n        arg0.passive = arg1 !== 0;\n    };\n    imports.wbg.__wbg_setwidth_c5fed9f5e7f0b406 = function(arg0, arg1) {\n        arg0.width = arg1 >>> 0;\n    };\n    imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {\n        const ret = typeof global === 'undefined' ? null : global;\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {\n        const ret = typeof globalThis === 'undefined' ? null : globalThis;\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {\n        const ret = typeof self === 'undefined' ? null : self;\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {\n        const ret = typeof window === 'undefined' ? null : window;\n        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);\n    };\n    imports.wbg.__wbg_then_44b73946d2fb3e7d = function(arg0, arg1) {\n        const ret = arg0.then(arg1);\n        return ret;\n    };\n    imports.wbg.__wbg_warn_4ca3906c248c47c4 = function(arg0) {\n        console.warn(arg0);\n    };\n    imports.wbg.__wbindgen_cb_drop = function(arg0) {\n        const obj = arg0.original;\n        if (obj.cnt-- == 1) {\n            obj.a = 0;\n            return true;\n        }\n        const ret = false;\n        return ret;\n    };\n    imports.wbg.__wbindgen_closure_wrapper101 = function(arg0, arg1, arg2) {\n        const ret = makeMutClosure(arg0, arg1, 40, __wbg_adapter_22);\n        return ret;\n    };\n    imports.wbg.__wbindgen_closure_wrapper116 = function(arg0, arg1, arg2) {\n        const ret = makeMutClosure(arg0, arg1, 47, __wbg_adapter_25);\n        return ret;\n    };\n    imports.wbg.__wbindgen_closure_wrapper140 = function(arg0, arg1, arg2) {\n        const ret = makeMutClosure(arg0, arg1, 59, __wbg_adapter_28);\n        return ret;\n    };\n    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {\n        const ret = debugString(arg1);\n        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len1 = WASM_VECTOR_LEN;\n        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);\n        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);\n    };\n    imports.wbg.__wbindgen_init_externref_table = function() {\n        const table = wasm.__wbindgen_export_2;\n        const offset = table.grow(4);\n        table.set(0, undefined);\n        table.set(offset + 0, undefined);\n        table.set(offset + 1, null);\n        table.set(offset + 2, true);\n        table.set(offset + 3, false);\n        ;\n    };\n    imports.wbg.__wbindgen_is_function = function(arg0) {\n        const ret = typeof(arg0) === 'function';\n        return ret;\n    };\n    imports.wbg.__wbindgen_is_undefined = function(arg0) {\n        const ret = arg0 === undefined;\n        return ret;\n    };\n    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {\n        const obj = arg1;\n        const ret = typeof(obj) === 'number' ? obj : undefined;\n        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);\n        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);\n    };\n    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {\n        const ret = getStringFromWasm0(arg0, arg1);\n        return ret;\n    };\n    imports.wbg.__wbindgen_throw = function(arg0, arg1) {\n        throw new Error(getStringFromWasm0(arg0, arg1));\n    };\n\n    return imports;\n}\n\nfunction __wbg_init_memory(imports, memory) {\n\n}\n\nfunction __wbg_finalize_init(instance, module) {\n    wasm = instance.exports;\n    __wbg_init.__wbindgen_wasm_module = module;\n    cachedDataViewMemory0 = null;\n    cachedUint8ArrayMemory0 = null;\n\n\n    wasm.__wbindgen_start();\n    return wasm;\n}\n\nfunction initSync(module) {\n    if (wasm !== undefined) return wasm;\n\n\n    if (typeof module !== 'undefined') {\n        if (Object.getPrototypeOf(module) === Object.prototype) {\n            ({module} = module)\n        } else {\n            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')\n        }\n    }\n\n    const imports = __wbg_get_imports();\n\n    __wbg_init_memory(imports);\n\n    if (!(module instanceof WebAssembly.Module)) {\n        module = new WebAssembly.Module(module);\n    }\n\n    const instance = new WebAssembly.Instance(module, imports);\n\n    return __wbg_finalize_init(instance, module);\n}\n\nasync function __wbg_init(module_or_path) {\n    if (wasm !== undefined) return wasm;\n\n\n    if (typeof module_or_path !== 'undefined') {\n        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {\n            ({module_or_path} = module_or_path)\n        } else {\n            console.warn('using deprecated parameters for the initialization function; pass a single object instead')\n        }\n    }\n\n    if (typeof module_or_path === 'undefined') {\n        module_or_path = new URL(/* asset import */ __webpack_require__(/*! embedded_engine_bg.wasm */ \"./pkg/embedded_engine_bg.wasm\"), __webpack_require__.b);\n    }\n    const imports = __wbg_get_imports();\n\n    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {\n        module_or_path = fetch(module_or_path);\n    }\n\n    __wbg_init_memory(imports);\n\n    const { instance, module } = await __wbg_load(await module_or_path, imports);\n\n    return __wbg_finalize_init(instance, module);\n}\n\n\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (__wbg_init);\n\n\n//# sourceURL=webpack:///./pkg/embedded_engine.js?\n}");

/***/ }),

/***/ "./pkg/embedded_engine_bg.wasm":
/*!*************************************!*\
  !*** ./pkg/embedded_engine_bg.wasm ***!
  \*************************************/
/***/ ((module, __unused_webpack_exports, __webpack_require__) => {

eval("{module.exports = __webpack_require__.p + \"9238080fe7de047280ea.wasm\";\n\n//# sourceURL=webpack:///./pkg/embedded_engine_bg.wasm?\n}");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		__webpack_require__.p = "/embedded-engine/";
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	(() => {
/******/ 		__webpack_require__.b = document.baseURI || self.location.href;
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		// no chunk on demand loading
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// no jsonp function
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./index.js");
/******/ 	
/******/ })()
;