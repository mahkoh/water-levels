(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "../pkg/water_levels_wasm.js":
/*!***********************************!*\
  !*** ../pkg/water_levels_wasm.js ***!
  \***********************************/
/*! exports provided: calculate, commit, Res, __wbg_new_693216e109162396, __wbg_stack_0ddaca5d1abfb52f, __wbg_error_09919627ac0992f5, __wbindgen_object_drop_ref, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./water_levels_wasm_bg.wasm */ \"../pkg/water_levels_wasm_bg.wasm\");\n/* harmony import */ var _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./water_levels_wasm_bg.js */ \"../pkg/water_levels_wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"calculate\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"calculate\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"commit\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"commit\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Res\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Res\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_693216e109162396\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_693216e109162396\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_0ddaca5d1abfb52f\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_0ddaca5d1abfb52f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_09919627ac0992f5\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_09919627ac0992f5\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _water_levels_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/water_levels_wasm.js?");

/***/ }),

/***/ "../pkg/water_levels_wasm_bg.js":
/*!**************************************!*\
  !*** ../pkg/water_levels_wasm_bg.js ***!
  \**************************************/
/*! exports provided: calculate, commit, Res, __wbg_new_693216e109162396, __wbg_stack_0ddaca5d1abfb52f, __wbg_error_09919627ac0992f5, __wbindgen_object_drop_ref, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"calculate\", function() { return calculate; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"commit\", function() { return commit; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Res\", function() { return Res; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_693216e109162396\", function() { return __wbg_new_693216e109162396; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_0ddaca5d1abfb52f\", function() { return __wbg_stack_0ddaca5d1abfb52f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_09919627ac0992f5\", function() { return __wbg_error_09919627ac0992f5; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./water_levels_wasm_bg.wasm */ \"../pkg/water_levels_wasm_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nlet cachegetFloat64Memory0 = null;\nfunction getFloat64Memory0() {\n    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetFloat64Memory0 = new Float64Array(_water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetFloat64Memory0;\n}\n\nfunction getArrayF64FromWasm0(ptr, len) {\n    return getFloat64Memory0().subarray(ptr / 8, ptr / 8 + len);\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArrayF64ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 8);\n    getFloat64Memory0().set(arg, ptr / 8);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n* @param {Float64Array} segments\n* @param {number} level\n* @returns {Res}\n*/\nfunction calculate(segments, level) {\n    var ptr0 = passArrayF64ToWasm0(segments, _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    var ret = _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"calculate\"](ptr0, len0, level);\n    return Res.__wrap(ret);\n}\n\n/**\n* @returns {string}\n*/\nfunction commit() {\n    try {\n        const retptr = _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"commit\"](retptr);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n/**\n*/\nclass Res {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Res.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_res_free\"](ptr);\n    }\n    /**\n    */\n    get res() {\n        try {\n            const retptr = _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_res_res\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            let v0;\n            if (r0 !== 0) {\n                v0 = getArrayF64FromWasm0(r0, r1).slice();\n                _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 8);\n            }\n            return v0;\n        } finally {\n            _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        }\n    }\n    /**\n    * @param {Float64Array | undefined} arg0\n    */\n    set res(arg0) {\n        var ptr0 = isLikeNone(arg0) ? 0 : passArrayF64ToWasm0(arg0, _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_res_res\"](this.ptr, ptr0, len0);\n    }\n    /**\n    */\n    get err() {\n        try {\n            const retptr = _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n            _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_res_err\"](retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            let v0;\n            if (r0 !== 0) {\n                v0 = getStringFromWasm0(r0, r1).slice();\n                _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 1);\n            }\n            return v0;\n        } finally {\n            _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        }\n    }\n    /**\n    * @param {string | undefined} arg0\n    */\n    set err(arg0) {\n        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_res_err\"](this.ptr, ptr0, len0);\n    }\n}\n\nfunction __wbg_new_693216e109162396() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_09919627ac0992f5(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _water_levels_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/water_levels_wasm_bg.js?");

/***/ }),

/***/ "../pkg/water_levels_wasm_bg.wasm":
/*!****************************************!*\
  !*** ../pkg/water_levels_wasm_bg.wasm ***!
  \****************************************/
/*! exports provided: memory, __wbg_res_free, __wbg_get_res_res, __wbg_set_res_res, __wbg_get_res_err, __wbg_set_res_err, calculate, commit, __wbindgen_add_to_stack_pointer, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./water_levels_wasm_bg.js */ \"../pkg/water_levels_wasm_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/water_levels_wasm_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var water_levels_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! water-levels-wasm */ \"../pkg/water_levels_wasm.js\");\n/* harmony import */ var d3__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! d3 */ \"./node_modules/d3/src/index.js\");\n\n\n\ndocument.querySelector(\"#commit\").innerText = water_levels_wasm__WEBPACK_IMPORTED_MODULE_0__[\"commit\"]();\n\nlet level_el = document.querySelector(\"#level\");\nlevel_el.addEventListener(\"input\", calculate);\n\nlet hours_el = document.querySelector(\"#hours\");\nhours_el.addEventListener(\"input\", () => {\n    level_el.value = hours_el.value;\n    calculate();\n})\n\nlet add_el = document.querySelector(\".add\");\nlet graph_el = document.querySelector(\"#graph\");\n\nfunction calculate() {\n    const WIDTH = 50;\n    let elevations = [];\n    document.querySelectorAll(\".elevation\").forEach((el) => elevations.push(el.value));\n    let level = level_el.value;\n    hours_el.value = level;\n    let res = water_levels_wasm__WEBPACK_IMPORTED_MODULE_0__[\"calculate\"](elevations, level);\n    graph_el.innerHTML = \"\";\n    let results = document.querySelectorAll(\".result\");\n    results.forEach(el => el.innerHTML = \"\");\n    if (res.err) {\n        let pre = document.createElement(\"pre\");\n        pre.innerText = \"Error: \" + res.err;\n        graph_el.appendChild(pre);\n    } else {\n        res = res.res;\n        let height = Math.max(...res);\n        let width = res.length * WIDTH;\n        let svg = d3__WEBPACK_IMPORTED_MODULE_1__[\"select\"](\"#graph\").append(\"svg\").attr(\"width\", width).attr(\"height\", height);\n        for (let i = 0; i < elevations.length; i++) {\n            svg.append(\"rect\")\n                .attr(\"x\", i * WIDTH)\n                .attr(\"y\", height - elevations[i])\n                .attr(\"width\", WIDTH)\n                .attr(\"height\", elevations[i])\n                .attr(\"fill\", \"#15792b\");\n            if (res[i] > elevations[i]) {\n                svg.append(\"rect\")\n                    .attr(\"x\", i * WIDTH)\n                    .attr(\"y\", height - res[i])\n                    .attr(\"width\", WIDTH)\n                    .attr(\"height\", res[i] - elevations[i])\n                    .attr(\"fill\", \"#116fc2\");\n            }\n            results[i].innerText = res[i];\n        }\n    }\n}\n\nfunction remove(el) {\n    let li = el.target.parentElement;\n    li.parentElement.removeChild(li);\n    calculate();\n}\n\nfor (let button of document.querySelectorAll(\".remove\")) {\n    button.addEventListener(\"click\", remove);\n}\ndocument.querySelectorAll(\".elevation\").forEach((el) => el.addEventListener(\"change\", calculate));\n\nfunction add_elevation(val) {\n    let li = add_el.parentElement;\n    let new_ = document.createElement(\"li\");\n    new_.innerHTML = `\n        <input class=\"elevation\" type=\"number\" value=\"${val}\">\n        <button class=\"remove\">Remove</button>\n        Result: <span class=\"result\"></span>\n    `;\n    new_.querySelector(\"button\").addEventListener(\"click\", remove);\n    new_.querySelector(\"input\").addEventListener(\"change\", calculate);\n    li.parentElement.insertBefore(new_, li);\n}\n\nadd_el.addEventListener(\"click\", (el) => {\n    add_elevation(0);\n    calculate();\n})\n\nadd_elevation(10);\nadd_elevation(30);\nadd_elevation(50);\nadd_elevation(70);\nadd_elevation(90);\nadd_elevation(0);\nadd_elevation(70);\nadd_elevation(50);\nadd_elevation(30);\nadd_elevation(10);\nadd_elevation(0);\nadd_elevation(44);\nadd_elevation(0);\nadd_elevation(0);\n\ncalculate();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);