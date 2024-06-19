let wasm;const heap=new Array(128).fill(undefined);heap.push(undefined,null,true,false);function getObject(a){return heap[a]}let WASM_VECTOR_LEN=0;let cachedUint8Memory0=null;function getUint8Memory0(){if(cachedUint8Memory0===null||cachedUint8Memory0.byteLength===0){cachedUint8Memory0=new Uint8Array(wasm.memory.buffer)};return cachedUint8Memory0}const cachedTextEncoder=typeof TextEncoder!=='undefined'?new TextEncoder('utf-8'):{encode:()=>{throw Error('TextEncoder not available')}};const encodeString=typeof cachedTextEncoder.encodeInto==='function'?function(a,b){return cachedTextEncoder.encodeInto(a,b)}:function(a,b){const c=cachedTextEncoder.encode(a);b.set(c);return {read:a.length,written:c.length}};function passStringToWasm0(a,b,c){if(c===undefined){const h=cachedTextEncoder.encode(a);const i=b(h.length,1)>>>0;getUint8Memory0().subarray(i,i+ h.length).set(h);WASM_VECTOR_LEN=h.length;return i};let d=a.length;let e=b(d,1)>>>0;const f=getUint8Memory0();let g=0;for(;g<d;g++){const h=a.charCodeAt(g);if(h>0x7F)break;f[e+ g]=h};if(g!==d){if(g!==0){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,1)>>>0;const h=getUint8Memory0().subarray(e+ g,e+ d);const i=encodeString(a,h);g+=i.written;e=c(e,d,g,1)>>>0};WASM_VECTOR_LEN=g;return e}function isLikeNone(a){return a===undefined||a===null}let cachedInt32Memory0=null;function getInt32Memory0(){if(cachedInt32Memory0===null||cachedInt32Memory0.byteLength===0){cachedInt32Memory0=new Int32Array(wasm.memory.buffer)};return cachedInt32Memory0}let heap_next=heap.length;function dropObject(a){if(a<132)return;heap[a]=heap_next;heap_next=a}function takeObject(a){const b=getObject(a);dropObject(a);return b}function addHeapObject(a){if(heap_next===heap.length)heap.push(heap.length+ 1);const b=heap_next;heap_next=heap[b];heap[b]=a;return b}const cachedTextDecoder=typeof TextDecoder!=='undefined'?new TextDecoder('utf-8',{ignoreBOM:true,fatal:true}):{decode:()=>{throw Error('TextDecoder not available')}};if(typeof TextDecoder!=='undefined'){cachedTextDecoder.decode()};function getStringFromWasm0(a,b){a=a>>>0;return cachedTextDecoder.decode(getUint8Memory0().subarray(a,a+ b))}function debugString(a){const b=typeof a;if(b=='number'||b=='boolean'||a==null){return `${a}`};if(b=='string'){return `"${a}"`};if(b=='symbol'){const e=a.description;if(e==null){return 'Symbol'}else{return `Symbol(${e})`}};if(b=='function'){const e=a.name;if(typeof e=='string'&&e.length>0){return `Function(${e})`}else{return 'Function'}};if(Array.isArray(a)){const e=a.length;let f='[';if(e>0){f+=debugString(a[0])};for(let g=1;g<e;g++){f+=', '+ debugString(a[g])};f+=']';return f};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>1){d=c[1]}else{return toString.call(a)};if(d=='Object'){try{return 'Object('+ JSON.stringify(a)+ ')'}catch(e){return 'Object'}};if(a instanceof Error){return `${a.name}: ${a.message}\n${a.stack}`};return d}const CLOSURE_DTORS=typeof FinalizationRegistry==='undefined'?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(a=>{wasm.__wbindgen_export_2.get(a.dtor)(a.a,a.b)});function makeMutClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;const h=e.a;e.a=0;try{return d(h,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(h,e.b);CLOSURE_DTORS.unregister(e)}else{e.a=h}}};f.original=e;CLOSURE_DTORS.register(f,e,e);return f}function __wbg_adapter_26(a,b,c){wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1a326a6dc08489e2(a,b,addHeapObject(c))}function makeClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;try{return d(e.a,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(e.a,e.b);e.a=0;CLOSURE_DTORS.unregister(e)}}};f.original=e;CLOSURE_DTORS.register(f,e,e);return f}function __wbg_adapter_29(a,b){wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h680b2da551fe94f0(a,b)}function __wbg_adapter_32(a,b,c){wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h40f082517919f349(a,b,addHeapObject(c))}function __wbg_adapter_35(a,b){wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6ced718866b924f0(a,b)}function __wbg_adapter_38(a,b,c){wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he5627adb80c0d291(a,b,addHeapObject(c))}function getCachedStringFromWasm0(a,b){if(a===0){return getObject(b)}else{return getStringFromWasm0(a,b)}}function handleError(a,b){try{return a.apply(this,b)}catch(c){wasm.__wbindgen_exn_store(addHeapObject(c))}}async function __wbg_load(a,b){if(typeof Response==='function'&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming==='function'){try{return await WebAssembly.instantiateStreaming(a,b)}catch(d){if(a.headers.get('Content-Type')!='application/wasm'){console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",d)}else{throw d}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}}function __wbg_get_imports(){const a={};a.wbg={};a.wbg.__wbindgen_is_undefined=function(b){const c=getObject(b)===undefined;return c};a.wbg.__wbindgen_string_get=function(b,c){const f=getObject(c);const g=typeof f==='string'?f:undefined;var d=isLikeNone(g)?0:passStringToWasm0(g,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbindgen_object_drop_ref=function(b){takeObject(b)};a.wbg.__wbindgen_object_clone_ref=function(b){const c=getObject(b);return addHeapObject(c)};a.wbg.__wbindgen_string_new=function(b,c){const d=getStringFromWasm0(b,c);return addHeapObject(d)};a.wbg.__wbindgen_cb_drop=function(b){const c=takeObject(b).original;if(c.cnt--==1){c.a=0;return true};const d=false;return d};a.wbg.__wbindgen_number_new=function(b){const c=b;return addHeapObject(c)};a.wbg.__wbindgen_boolean_get=function(b){const c=getObject(b);const d=typeof c==='boolean'?(c?1:0):2;return d};a.wbg.__wbindgen_jsval_eq=function(b,c){const d=getObject(b)===getObject(c);return d};a.wbg.__wbg_nodeId_bbf0efafa303e805=function(b,c){const d=getObject(c).$$$nodeId;getInt32Memory0()[b/4+ 1]=isLikeNone(d)?0:d;getInt32Memory0()[b/4+ 0]=!isLikeNone(d)};a.wbg.__wbg_setnodeId_433ef8ed15bd1612=function(b,c){getObject(b).$$$nodeId=c>>>0};a.wbg.__wbg_createTextNode_a7d5f5b956acda97=function(b,c){const d=getObject(b).createTextNode(c);return addHeapObject(d)};a.wbg.__wbg_setclassName_f86a95d6ffe042e6=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).className=e},arguments)};a.wbg.__wbg_queueMicrotask_b580a35152f7cc7c=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_newwitheventinitdict_df944b5b582e7ecf=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new CustomEvent(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_documentElement_da9c841ddb352d95=function(b){const c=getObject(b).documentElement;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_body_edb1908d3ceff3a1=function(b){const c=getObject(b).body;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_createComment_354ccab4fdc521ee=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createComment(e);return addHeapObject(f)};a.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createElement(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_createTextNode_0c38fd80a5b2284d=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createTextNode(e);return addHeapObject(f)};a.wbg.__wbg_getElementById_c369ff43f0db99cf=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).getElementById(e);return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_querySelector_a5f74efc5fa193dd=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelector(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_querySelectorAll_4e0fcdb64cda2cd5=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_tagName_a98846fa0b63dd7f=function(b,c){const d=getObject(c).tagName;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setid_37bacc3f09f555aa=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).id=e};a.wbg.__wbg_innerHTML_5e7bc1b9545c80e2=function(b,c){const d=getObject(c).innerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).innerHTML=e};a.wbg.__wbg_outerHTML_e073aa84e7bc1eaf=function(b,c){const d=getObject(c).outerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_closest_52e34b5423e99b39=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).closest(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_getAttribute_99bddb29274b29b9=function(b,c,d,e){var f=getCachedStringFromWasm0(d,e);const i=getObject(c).getAttribute(f);var g=isLikeNone(i)?0:passStringToWasm0(i,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var h=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=h;getInt32Memory0()[b/4+ 0]=g};a.wbg.__wbg_insertAdjacentHTML_00b8bca474a1bc0a=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).insertAdjacentHTML(g,h)},arguments)};a.wbg.__wbg_querySelectorAll_2dc2b23a07bee926=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).setAttribute(g,h)},arguments)};a.wbg.__wbg_append_fcf463f0b4a8f219=function(){return handleError(function(b,c){getObject(b).append(getObject(c))},arguments)};a.wbg.__wbg_target_2fc177e386c8b7b0=function(b){const c=getObject(b).target;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_preventDefault_b1a4aafc79409429=function(b){getObject(b).preventDefault()};a.wbg.__wbg_addEventListener_53b787075bd5e003=function(){return handleError(function(b,c,d,e){var f=getCachedStringFromWasm0(c,d);getObject(b).addEventListener(f,getObject(e))},arguments)};a.wbg.__wbg_dispatchEvent_63c0c01600a98fd2=function(){return handleError(function(b,c){const d=getObject(b).dispatchEvent(getObject(c));return d},arguments)};a.wbg.__wbg_new_ab6fd82b10560829=function(){return handleError(function(){const b=new Headers();return addHeapObject(b)},arguments)};a.wbg.__wbg_pushState_b8e8d346f8bb33fd=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).pushState(getObject(c),h,i)},arguments)};a.wbg.__wbg_replaceState_ec9431bea5108a50=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).replaceState(getObject(c),h,i)},arguments)};a.wbg.__wbg_rel_97d32812b3a790c0=function(b,c){const d=getObject(c).rel;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_origin_553ab46a8b9d9701=function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_pathname_f1a386f593a3c228=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_hash_47130aed03d7cade=function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_href_2edbae9e92cdfeff=function(b,c){const d=getObject(c).href;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_altKey_2e6c34c37088d8b1=function(b){const c=getObject(b).altKey;return c};a.wbg.__wbg_ctrlKey_bb5b6fef87339703=function(b){const c=getObject(b).ctrlKey;return c};a.wbg.__wbg_shiftKey_5911baf439ab232b=function(b){const c=getObject(b).shiftKey;return c};a.wbg.__wbg_metaKey_6bf4ae4e83a11278=function(b){const c=getObject(b).metaKey;return c};a.wbg.__wbg_sethref_b94692d1a9f05b53=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).href=e},arguments)};a.wbg.__wbg_origin_ee93e29ace71f568=function(){return handleError(function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_pathname_5449afe3829f96a1=function(){return handleError(function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_hash_553098e838e06c1d=function(){return handleError(function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_sethash_03173a6fc146de27=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).hash=e},arguments)};a.wbg.__wbg_language_64a5be2885d1c412=function(b,c){const f=getObject(c).language;var d=isLikeNone(f)?0:passStringToWasm0(f,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbg_languages_185a098934c0e22a=function(b){const c=getObject(b).languages;return addHeapObject(c)};a.wbg.__wbg_parentNode_6be3abff20e1a5fb=function(b){const c=getObject(b).parentNode;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_nextSibling_709614fdb0fb7a66=function(b){const c=getObject(b).nextSibling;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_settextContent_d271bab459cbb1ba=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).textContent=e};a.wbg.__wbg_appendChild_580ccb11a660db68=function(){return handleError(function(b,c){const d=getObject(b).appendChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_insertBefore_d2a001abf538c1f8=function(){return handleError(function(b,c,d){const e=getObject(b).insertBefore(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return handleError(function(b,c){const d=getObject(b).removeChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_replaceChild_b8c54d870e3cc9bf=function(){return handleError(function(b,c,d){const e=getObject(b).replaceChild(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_length_d0a802565d17eec4=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_get_8cd5eba00ab6304f=function(b,c){const d=getObject(b)[c>>>0];return isLikeNone(d)?0:addHeapObject(d)};a.wbg.__wbg_url_7807f6a1fddc3e23=function(b,c){const d=getObject(c).url;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_newwithstr_36b0b3f97efe096f=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Request(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_newwithstrandinit_3fd6fba4083ff2d0=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new Request(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_instanceof_Response_849eb93e75734b6e=function(b){let c;try{c=getObject(b) instanceof Response}catch(e){c=false};const d=c;return d};a.wbg.__wbg_status_61a01141acd3cf74=function(b){const c=getObject(b).status;return c};a.wbg.__wbg_text_450a059667fd91fd=function(){return handleError(function(b){const c=getObject(b).text();return addHeapObject(c)},arguments)};a.wbg.__wbg_pathname_c5fe403ef9525ec6=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_search_c68f506c44be6d1e=function(b,c){const d=getObject(c).search;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setsearch_fd62f4de409a2bb3=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).search=e};a.wbg.__wbg_new_67853c351755d2cf=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new URL(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_new_4c501d7c115d20a6=function(){return handleError(function(){const b=new URLSearchParams();return addHeapObject(b)},arguments)};a.wbg.__wbg_instanceof_Window_f401953a2cf86220=function(b){let c;try{c=getObject(b) instanceof Window}catch(e){c=false};const d=c;return d};a.wbg.__wbg_document_5100775d18896c16=function(b){const c=getObject(b).document;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_location_2951b5ee34f19221=function(b){const c=getObject(b).location;return addHeapObject(c)};a.wbg.__wbg_history_bc4057de66a2015f=function(){return handleError(function(b){const c=getObject(b).history;return addHeapObject(c)},arguments)};a.wbg.__wbg_navigator_6c8fa55c5cc8796e=function(b){const c=getObject(b).navigator;return addHeapObject(c)};a.wbg.__wbg_scrollTo_4d970c5e1c4b340b=function(b,c,d){getObject(b).scrollTo(c,d)};a.wbg.__wbg_get_d36d61640bbf4503=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b)[e];return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_fetch_c4b6afebdb1f918e=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_instanceof_WorkerGlobalScope_46b577f151fad960=function(b){let c;try{c=getObject(b) instanceof WorkerGlobalScope}catch(e){c=false};const d=c;return d};a.wbg.__wbg_fetch_921fad6ef9e883dd=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_log_5bb5f88f245d7762=function(b){console.log(getObject(b))};a.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=function(b){const c=getObject(b).queueMicrotask;return addHeapObject(c)};a.wbg.__wbindgen_is_function=function(b){const c=typeof getObject(b)==='function';return c};a.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return handleError(function(){const b=self.self;return addHeapObject(b)},arguments)};a.wbg.__wbg_window_c6fb939a7f436783=function(){return handleError(function(){const b=window.window;return addHeapObject(b)},arguments)};a.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return handleError(function(){const b=globalThis.globalThis;return addHeapObject(b)},arguments)};a.wbg.__wbg_global_207b558942527489=function(){return handleError(function(){const b=global.global;return addHeapObject(b)},arguments)};a.wbg.__wbg_length_cd7af8117672b8b8=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_new_72fb9a18b5ae2624=function(){const b=new Object();return addHeapObject(b)};a.wbg.__wbg_decodeURIComponent_59d9a001daa1c250=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=decodeURIComponent(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_get_bd8e338fbd5f5cc8=function(b,c){const d=getObject(b)[c>>>0];return addHeapObject(d)};a.wbg.__wbg_instanceof_Error_e20bb56fd5591a93=function(b){let c;try{c=getObject(b) instanceof Error}catch(e){c=false};const d=c;return d};a.wbg.__wbg_message_5bf28016c2b49cfb=function(b){const c=getObject(b).message;return addHeapObject(c)};a.wbg.__wbg_name_e7429f0dda6079e2=function(b){const c=getObject(b).name;return addHeapObject(c)};a.wbg.__wbg_toString_ffe4c9ea3b3532e9=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_newnoargs_e258087cd0daa0ea=function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Function(d);return addHeapObject(e)};a.wbg.__wbg_call_27c0f87801dedf93=function(){return handleError(function(b,c){const d=getObject(b).call(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_getTimezoneOffset_38257122e236c190=function(b){const c=getObject(b).getTimezoneOffset();return c};a.wbg.__wbg_new_cf3ec55744a78578=function(b){const c=new Date(getObject(b));return addHeapObject(c)};a.wbg.__wbg_is_010fdc0f4ab96916=function(b,c){const d=Object.is(getObject(b),getObject(c));return d};a.wbg.__wbg_toString_c816a20ab859d0c1=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_get_e3c254076557e348=function(){return handleError(function(b,c){const d=Reflect.get(getObject(b),getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_set_1f9b04f170055d33=function(){return handleError(function(b,c,d){const e=Reflect.set(getObject(b),getObject(c),getObject(d));return e},arguments)};a.wbg.__wbg_resolve_b0083a7967828ec8=function(b){const c=Promise.resolve(getObject(b));return addHeapObject(c)};a.wbg.__wbg_then_0c86a60e8fcfe9f6=function(b,c){const d=getObject(b).then(getObject(c));return addHeapObject(d)};a.wbg.__wbg_then_a73caa9a87991566=function(b,c,d){const e=getObject(b).then(getObject(c),getObject(d));return addHeapObject(e)};a.wbg.__wbindgen_debug_string=function(b,c){const d=debugString(getObject(c));const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbindgen_throw=function(b,c){throw new Error(getStringFromWasm0(b,c))};a.wbg.__wbindgen_closure_wrapper869=function(b,c,d){const e=makeMutClosure(b,c,277,__wbg_adapter_26);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1151=function(b,c,d){const e=makeClosure(b,c,316,__wbg_adapter_29);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1159=function(b,c,d){const e=makeMutClosure(b,c,316,__wbg_adapter_32);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper2002=function(b,c,d){const e=makeMutClosure(b,c,455,__wbg_adapter_35);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper3149=function(b,c,d){const e=makeMutClosure(b,c,480,__wbg_adapter_38);return addHeapObject(e)};return a}function __wbg_init_memory(a,b){}function __wbg_finalize_init(a,b){wasm=a.exports;__wbg_init.__wbindgen_wasm_module=b;cachedInt32Memory0=null;cachedUint8Memory0=null;wasm.__wbindgen_start();return wasm}function initSync(a){if(wasm!==undefined)return wasm;const b=__wbg_get_imports();__wbg_init_memory(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const c=new WebAssembly.Instance(a,b);return __wbg_finalize_init(c,a)}async function __wbg_init(a){if(wasm!==undefined)return wasm;if(typeof a==='undefined'){a=new URL('perseus_engine_bg.wasm',import.meta.url)};const b=__wbg_get_imports();if(typeof a==='string'||typeof Request==='function'&&a instanceof Request||typeof URL==='function'&&a instanceof URL){a=fetch(a)};__wbg_init_memory(b);const {instance:c,module:d}=await __wbg_load(await a,b);return __wbg_finalize_init(c,d)}export default __wbg_init;;