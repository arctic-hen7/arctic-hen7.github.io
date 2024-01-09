let wasm;const heap=new Array(128).fill(undefined);heap.push(undefined,null,true,false);function getObject(a){return heap[a]}let WASM_VECTOR_LEN=0;let cachedUint8Memory0=null;function getUint8Memory0(){if(cachedUint8Memory0===null||cachedUint8Memory0.byteLength===0){cachedUint8Memory0=new Uint8Array(wasm.memory.buffer)};return cachedUint8Memory0}const cachedTextEncoder=typeof TextEncoder!=='undefined'?new TextEncoder('utf-8'):{encode:()=>{throw Error('TextEncoder not available')}};const encodeString=typeof cachedTextEncoder.encodeInto==='function'?function(a,b){return cachedTextEncoder.encodeInto(a,b)}:function(a,b){const c=cachedTextEncoder.encode(a);b.set(c);return {read:a.length,written:c.length}};function passStringToWasm0(a,b,c){if(c===undefined){const h=cachedTextEncoder.encode(a);const i=b(h.length,1)>>>0;getUint8Memory0().subarray(i,i+ h.length).set(h);WASM_VECTOR_LEN=h.length;return i};let d=a.length;let e=b(d,1)>>>0;const f=getUint8Memory0();let g=0;for(;g<d;g++){const h=a.charCodeAt(g);if(h>0x7F)break;f[e+ g]=h};if(g!==d){if(g!==0){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,1)>>>0;const h=getUint8Memory0().subarray(e+ g,e+ d);const i=encodeString(a,h);g+=i.written};WASM_VECTOR_LEN=g;return e}function isLikeNone(a){return a===undefined||a===null}let cachedInt32Memory0=null;function getInt32Memory0(){if(cachedInt32Memory0===null||cachedInt32Memory0.byteLength===0){cachedInt32Memory0=new Int32Array(wasm.memory.buffer)};return cachedInt32Memory0}let heap_next=heap.length;function dropObject(a){if(a<132)return;heap[a]=heap_next;heap_next=a}function takeObject(a){const b=getObject(a);dropObject(a);return b}function addHeapObject(a){if(heap_next===heap.length)heap.push(heap.length+ 1);const b=heap_next;heap_next=heap[b];heap[b]=a;return b}const cachedTextDecoder=typeof TextDecoder!=='undefined'?new TextDecoder('utf-8',{ignoreBOM:true,fatal:true}):{decode:()=>{throw Error('TextDecoder not available')}};if(typeof TextDecoder!=='undefined'){cachedTextDecoder.decode()};function getStringFromWasm0(a,b){a=a>>>0;return cachedTextDecoder.decode(getUint8Memory0().subarray(a,a+ b))}function debugString(a){const b=typeof a;if(b=='number'||b=='boolean'||a==null){return `${a}`};if(b=='string'){return `"${a}"`};if(b=='symbol'){const e=a.description;if(e==null){return 'Symbol'}else{return `Symbol(${e})`}};if(b=='function'){const e=a.name;if(typeof e=='string'&&e.length>0){return `Function(${e})`}else{return 'Function'}};if(Array.isArray(a)){const e=a.length;let f='[';if(e>0){f+=debugString(a[0])};for(let g=1;g<e;g++){f+=', '+ debugString(a[g])};f+=']';return f};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>1){d=c[1]}else{return toString.call(a)};if(d=='Object'){try{return 'Object('+ JSON.stringify(a)+ ')'}catch(e){return 'Object'}};if(a instanceof Error){return `${a.name}: ${a.message}\n${a.stack}`};return d}function makeClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;try{return d(e.a,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(e.a,e.b);e.a=0}}};f.original=e;return f}function __wbg_adapter_26(a,b){wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h089fd8880e927554(a,b)}function makeMutClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;const h=e.a;e.a=0;try{return d(h,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(h,e.b)}else{e.a=h}}};f.original=e;return f}function __wbg_adapter_29(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__hcc7032eb88946665(a,b,addHeapObject(c))}function __wbg_adapter_32(a,b){wasm.wasm_bindgen__convert__closures__invoke0_mut__h64a144e66f6c28a5(a,b)}function __wbg_adapter_35(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__h18af2cd794bb0426(a,b,addHeapObject(c))}function getCachedStringFromWasm0(a,b){if(a===0){return getObject(b)}else{return getStringFromWasm0(a,b)}}function handleError(a,b){try{return a.apply(this,b)}catch(c){wasm.__wbindgen_exn_store(addHeapObject(c))}}async function __wbg_load(a,b){if(typeof Response==='function'&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming==='function'){try{return await WebAssembly.instantiateStreaming(a,b)}catch(d){if(a.headers.get('Content-Type')!='application/wasm'){console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",d)}else{throw d}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}}function __wbg_get_imports(){const a={};a.wbg={};a.wbg.__wbindgen_is_undefined=function(b){const c=getObject(b)===undefined;return c};a.wbg.__wbindgen_string_get=function(b,c){const f=getObject(c);const g=typeof f==='string'?f:undefined;var d=isLikeNone(g)?0:passStringToWasm0(g,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbindgen_object_drop_ref=function(b){takeObject(b)};a.wbg.__wbindgen_object_clone_ref=function(b){const c=getObject(b);return addHeapObject(c)};a.wbg.__wbindgen_string_new=function(b,c){const d=getStringFromWasm0(b,c);return addHeapObject(d)};a.wbg.__wbindgen_cb_drop=function(b){const c=takeObject(b).original;if(c.cnt--==1){c.a=0;return true};const d=false;return d};a.wbg.__wbindgen_number_new=function(b){const c=b;return addHeapObject(c)};a.wbg.__wbindgen_boolean_get=function(b){const c=getObject(b);const d=typeof c==='boolean'?(c?1:0):2;return d};a.wbg.__wbindgen_jsval_eq=function(b,c){const d=getObject(b)===getObject(c);return d};a.wbg.__wbg_nodeId_bbf0efafa303e805=function(b,c){const d=getObject(c).$$$nodeId;getInt32Memory0()[b/4+ 1]=isLikeNone(d)?0:d;getInt32Memory0()[b/4+ 0]=!isLikeNone(d)};a.wbg.__wbg_setnodeId_433ef8ed15bd1612=function(b,c){getObject(b).$$$nodeId=c>>>0};a.wbg.__wbg_createTextNode_a7d5f5b956acda97=function(b,c){const d=getObject(b).createTextNode(c);return addHeapObject(d)};a.wbg.__wbg_setclassName_f86a95d6ffe042e6=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).className=e},arguments)};a.wbg.__wbg_queueMicrotask_b580a35152f7cc7c=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_newwitheventinitdict_4444ad4e8ce3d9dd=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new CustomEvent(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_documentElement_1cdfe4c8cdb2f569=function(b){const c=getObject(b).documentElement;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_body_64abc9aba1891e91=function(b){const c=getObject(b).body;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_createComment_529b047c02bbe600=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createComment(e);return addHeapObject(f)};a.wbg.__wbg_createElement_fdd5c113cb84539e=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createElement(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_createTextNode_7ff0c034b2855f66=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createTextNode(e);return addHeapObject(f)};a.wbg.__wbg_getElementById_65b9547a428b5eb4=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).getElementById(e);return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_querySelector_c72dce5ac4b6bc3e=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelector(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_querySelectorAll_3c5fa13bff8fc108=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_tagName_3c7c71862fb4d0ec=function(b,c){const d=getObject(c).tagName;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setid_26cf25dc305dcc43=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).id=e};a.wbg.__wbg_innerHTML_f7f7b01874bbd3af=function(b,c){const d=getObject(c).innerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setinnerHTML_ce0d6527ce4086f2=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).innerHTML=e};a.wbg.__wbg_outerHTML_b5a8d952b5615778=function(b,c){const d=getObject(c).outerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_closest_8ebda487e74cc848=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).closest(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_getAttribute_bff489553dd803cc=function(b,c,d,e){var f=getCachedStringFromWasm0(d,e);const i=getObject(c).getAttribute(f);var g=isLikeNone(i)?0:passStringToWasm0(i,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var h=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=h;getInt32Memory0()[b/4+ 0]=g};a.wbg.__wbg_insertAdjacentHTML_5069477067719725=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).insertAdjacentHTML(g,h)},arguments)};a.wbg.__wbg_querySelectorAll_3e2bd695ce88c618=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_setAttribute_e7b72a5e7cfcb5a3=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).setAttribute(g,h)},arguments)};a.wbg.__wbg_append_df44ca631c3c1657=function(){return handleError(function(b,c){getObject(b).append(getObject(c))},arguments)};a.wbg.__wbg_target_52ddf6955f636bf5=function(b){const c=getObject(b).target;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_preventDefault_7f821f72e7c6b5d4=function(b){getObject(b).preventDefault()};a.wbg.__wbg_addEventListener_9bf60ea8a362e5e4=function(){return handleError(function(b,c,d,e){var f=getCachedStringFromWasm0(c,d);getObject(b).addEventListener(f,getObject(e))},arguments)};a.wbg.__wbg_dispatchEvent_40c3472e9e4dcf5e=function(){return handleError(function(b,c){const d=getObject(b).dispatchEvent(getObject(c));return d},arguments)};a.wbg.__wbg_new_7a20246daa6eec7e=function(){return handleError(function(){const b=new Headers();return addHeapObject(b)},arguments)};a.wbg.__wbg_pushState_e159043fce8f87bc=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).pushState(getObject(c),h,i)},arguments)};a.wbg.__wbg_replaceState_b51dd62c7235b1ac=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).replaceState(getObject(c),h,i)},arguments)};a.wbg.__wbg_rel_a5353976cdef8cba=function(b,c){const d=getObject(c).rel;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_origin_65f3c9cf7b510b7f=function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_pathname_928bf8d6f9c4be6b=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_hash_7b11ae9299213ee6=function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_href_f21dc804d4da134a=function(b,c){const d=getObject(c).href;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_altKey_bf16cace6fb79198=function(b){const c=getObject(b).altKey;return c};a.wbg.__wbg_ctrlKey_977280484bcead08=function(b){const c=getObject(b).ctrlKey;return c};a.wbg.__wbg_shiftKey_55894418ec38c771=function(b){const c=getObject(b).shiftKey;return c};a.wbg.__wbg_metaKey_16606958d932a374=function(b){const c=getObject(b).metaKey;return c};a.wbg.__wbg_sethref_90b000c8b01f96b1=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).href=e},arguments)};a.wbg.__wbg_origin_595edc88be6e66b8=function(){return handleError(function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_pathname_1ab7e82aaa4511ff=function(){return handleError(function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_hash_be2940ca236b5efc=function(){return handleError(function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_sethash_73fa47879a93b645=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).hash=e},arguments)};a.wbg.__wbg_language_4fad184e312b8f92=function(b,c){const f=getObject(c).language;var d=isLikeNone(f)?0:passStringToWasm0(f,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbg_languages_8bbf651a1c5254f7=function(b){const c=getObject(b).languages;return addHeapObject(c)};a.wbg.__wbg_parentNode_92a7017b3a4fad43=function(b){const c=getObject(b).parentNode;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_nextSibling_bafccd3347d24543=function(b){const c=getObject(b).nextSibling;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_settextContent_3ebccdd9354e1601=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).textContent=e};a.wbg.__wbg_appendChild_d30e6b83791d04c0=function(){return handleError(function(b,c){const d=getObject(b).appendChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_insertBefore_726c1640c419e940=function(){return handleError(function(b,c,d){const e=getObject(b).insertBefore(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_removeChild_942eb9c02243d84d=function(){return handleError(function(b,c){const d=getObject(b).removeChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_replaceChild_4ef0c29ad2413951=function(){return handleError(function(b,c,d){const e=getObject(b).replaceChild(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_length_f845c1c304d9837a=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_get_14aeb06cb5731d58=function(b,c){const d=getObject(b)[c>>>0];return isLikeNone(d)?0:addHeapObject(d)};a.wbg.__wbg_url_d64448346abf0f74=function(b,c){const d=getObject(c).url;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_newwithstr_8aa8479760b1e560=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Request(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_newwithstrandinit_f581dff0d19a8b03=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new Request(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_instanceof_Response_4c3b1446206114d1=function(b){let c;try{c=getObject(b) instanceof Response}catch(e){c=false};const d=c;return d};a.wbg.__wbg_status_d6d47ad2837621eb=function(b){const c=getObject(b).status;return c};a.wbg.__wbg_text_668782292b0bc561=function(){return handleError(function(b){const c=getObject(b).text();return addHeapObject(c)},arguments)};a.wbg.__wbg_pathname_aeafa820be91c325=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_search_f6e95882a48d3f69=function(b,c){const d=getObject(c).search;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setsearch_4f7d084e0d811add=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).search=e};a.wbg.__wbg_new_9e08fd37c1c53142=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new URL(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_new_bc66a7e94d71957f=function(){return handleError(function(){const b=new URLSearchParams();return addHeapObject(b)},arguments)};a.wbg.__wbg_instanceof_Window_3e5cd1f48c152d01=function(b){let c;try{c=getObject(b) instanceof Window}catch(e){c=false};const d=c;return d};a.wbg.__wbg_document_d609202d16c38224=function(b){const c=getObject(b).document;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_location_176c34e89c2c9d80=function(b){const c=getObject(b).location;return addHeapObject(c)};a.wbg.__wbg_history_80998b7456bf367e=function(){return handleError(function(b){const c=getObject(b).history;return addHeapObject(c)},arguments)};a.wbg.__wbg_navigator_96ba491902f8f083=function(b){const c=getObject(b).navigator;return addHeapObject(c)};a.wbg.__wbg_scrollTo_eb21c4452d7b3cd6=function(b,c,d){getObject(b).scrollTo(c,d)};a.wbg.__wbg_get_644791d4d61a5f69=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b)[e];return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_fetch_6c415b3a07763878=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_instanceof_WorkerGlobalScope_af28ee97555db40a=function(b){let c;try{c=getObject(b) instanceof WorkerGlobalScope}catch(e){c=false};const d=c;return d};a.wbg.__wbg_fetch_693453ca3f88c055=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_log_a4530b4fe289336f=function(b){console.log(getObject(b))};a.wbg.__wbg_queueMicrotask_adae4bc085237231=function(b){const c=getObject(b).queueMicrotask;return addHeapObject(c)};a.wbg.__wbindgen_is_function=function(b){const c=typeof getObject(b)==='function';return c};a.wbg.__wbg_queueMicrotask_4d890031a6a5a50c=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_self_f0e34d89f33b99fd=function(){return handleError(function(){const b=self.self;return addHeapObject(b)},arguments)};a.wbg.__wbg_window_d3b084224f4774d7=function(){return handleError(function(){const b=window.window;return addHeapObject(b)},arguments)};a.wbg.__wbg_globalThis_9caa27ff917c6860=function(){return handleError(function(){const b=globalThis.globalThis;return addHeapObject(b)},arguments)};a.wbg.__wbg_global_35dfdd59a4da3e74=function(){return handleError(function(){const b=global.global;return addHeapObject(b)},arguments)};a.wbg.__wbg_length_1009b1af0c481d7b=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_new_9fb8d994e1c0aaac=function(){const b=new Object();return addHeapObject(b)};a.wbg.__wbg_decodeURIComponent_22e23486459e3e19=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=decodeURIComponent(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_get_f01601b5a68d10e3=function(b,c){const d=getObject(b)[c>>>0];return addHeapObject(d)};a.wbg.__wbg_instanceof_Error_31ca8d97f188bfbc=function(b){let c;try{c=getObject(b) instanceof Error}catch(e){c=false};const d=c;return d};a.wbg.__wbg_message_55b9ea8030688597=function(b){const c=getObject(b).message;return addHeapObject(c)};a.wbg.__wbg_name_e5eede664187fed6=function(b){const c=getObject(b).name;return addHeapObject(c)};a.wbg.__wbg_toString_a44236e90224e279=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_newnoargs_c62ea9419c21fbac=function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Function(d);return addHeapObject(e)};a.wbg.__wbg_call_90c26b09837aba1c=function(){return handleError(function(b,c){const d=getObject(b).call(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_getTimezoneOffset_e742a5098e2c04d3=function(b){const c=getObject(b).getTimezoneOffset();return c};a.wbg.__wbg_new_d77fabdc03b9edd6=function(b){const c=new Date(getObject(b));return addHeapObject(c)};a.wbg.__wbg_is_ff7acd231c75c0e4=function(b,c){const d=Object.is(getObject(b),getObject(c));return d};a.wbg.__wbg_toString_6577cc00288ad588=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_get_7b48513de5dc5ea4=function(){return handleError(function(b,c){const d=Reflect.get(getObject(b),getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_set_759f75cd92b612d2=function(){return handleError(function(b,c,d){const e=Reflect.set(getObject(b),getObject(c),getObject(d));return e},arguments)};a.wbg.__wbg_resolve_6e1c6553a82f85b7=function(b){const c=Promise.resolve(getObject(b));return addHeapObject(c)};a.wbg.__wbg_then_3ab08cd4fbb91ae9=function(b,c){const d=getObject(b).then(getObject(c));return addHeapObject(d)};a.wbg.__wbg_then_8371cc12cfedc5a2=function(b,c,d){const e=getObject(b).then(getObject(c),getObject(d));return addHeapObject(e)};a.wbg.__wbindgen_debug_string=function(b,c){const d=debugString(getObject(c));const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbindgen_throw=function(b,c){throw new Error(getStringFromWasm0(b,c))};a.wbg.__wbindgen_closure_wrapper1128=function(b,c,d){const e=makeClosure(b,c,304,__wbg_adapter_26);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1135=function(b,c,d){const e=makeMutClosure(b,c,304,__wbg_adapter_29);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1944=function(b,c,d){const e=makeMutClosure(b,c,427,__wbg_adapter_32);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper3092=function(b,c,d){const e=makeMutClosure(b,c,451,__wbg_adapter_35);return addHeapObject(e)};return a}function __wbg_init_memory(a,b){}function __wbg_finalize_init(a,b){wasm=a.exports;__wbg_init.__wbindgen_wasm_module=b;cachedInt32Memory0=null;cachedUint8Memory0=null;wasm.__wbindgen_start();return wasm}function initSync(a){if(wasm!==undefined)return wasm;const b=__wbg_get_imports();__wbg_init_memory(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const c=new WebAssembly.Instance(a,b);return __wbg_finalize_init(c,a)}async function __wbg_init(a){if(wasm!==undefined)return wasm;if(typeof a==='undefined'){a=new URL('perseus_engine_bg.wasm',import.meta.url)};const b=__wbg_get_imports();if(typeof a==='string'||typeof Request==='function'&&a instanceof Request||typeof URL==='function'&&a instanceof URL){a=fetch(a)};__wbg_init_memory(b);const {instance:c,module:d}=await __wbg_load(await a,b);return __wbg_finalize_init(c,d)}export default __wbg_init;;