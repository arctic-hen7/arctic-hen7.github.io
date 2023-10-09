let wasm;const heap=new Array(128).fill(undefined);heap.push(undefined,null,true,false);function getObject(a){return heap[a]}let WASM_VECTOR_LEN=0;let cachedUint8Memory0=null;function getUint8Memory0(){if(cachedUint8Memory0===null||cachedUint8Memory0.byteLength===0){cachedUint8Memory0=new Uint8Array(wasm.memory.buffer)};return cachedUint8Memory0}const cachedTextEncoder=typeof TextEncoder!=='undefined'?new TextEncoder('utf-8'):{encode:()=>{throw Error('TextEncoder not available')}};const encodeString=typeof cachedTextEncoder.encodeInto==='function'?function(a,b){return cachedTextEncoder.encodeInto(a,b)}:function(a,b){const c=cachedTextEncoder.encode(a);b.set(c);return {read:a.length,written:c.length}};function passStringToWasm0(a,b,c){if(c===undefined){const h=cachedTextEncoder.encode(a);const i=b(h.length,1)>>>0;getUint8Memory0().subarray(i,i+ h.length).set(h);WASM_VECTOR_LEN=h.length;return i};let d=a.length;let e=b(d,1)>>>0;const f=getUint8Memory0();let g=0;for(;g<d;g++){const h=a.charCodeAt(g);if(h>0x7F)break;f[e+ g]=h};if(g!==d){if(g!==0){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,1)>>>0;const h=getUint8Memory0().subarray(e+ g,e+ d);const i=encodeString(a,h);g+=i.written};WASM_VECTOR_LEN=g;return e}function isLikeNone(a){return a===undefined||a===null}let cachedInt32Memory0=null;function getInt32Memory0(){if(cachedInt32Memory0===null||cachedInt32Memory0.byteLength===0){cachedInt32Memory0=new Int32Array(wasm.memory.buffer)};return cachedInt32Memory0}let heap_next=heap.length;function dropObject(a){if(a<132)return;heap[a]=heap_next;heap_next=a}function takeObject(a){const b=getObject(a);dropObject(a);return b}function addHeapObject(a){if(heap_next===heap.length)heap.push(heap.length+ 1);const b=heap_next;heap_next=heap[b];heap[b]=a;return b}const cachedTextDecoder=typeof TextDecoder!=='undefined'?new TextDecoder('utf-8',{ignoreBOM:true,fatal:true}):{decode:()=>{throw Error('TextDecoder not available')}};if(typeof TextDecoder!=='undefined'){cachedTextDecoder.decode()};function getStringFromWasm0(a,b){a=a>>>0;return cachedTextDecoder.decode(getUint8Memory0().subarray(a,a+ b))}function debugString(a){const b=typeof a;if(b=='number'||b=='boolean'||a==null){return `${a}`};if(b=='string'){return `"${a}"`};if(b=='symbol'){const e=a.description;if(e==null){return 'Symbol'}else{return `Symbol(${e})`}};if(b=='function'){const e=a.name;if(typeof e=='string'&&e.length>0){return `Function(${e})`}else{return 'Function'}};if(Array.isArray(a)){const e=a.length;let f='[';if(e>0){f+=debugString(a[0])};for(let g=1;g<e;g++){f+=', '+ debugString(a[g])};f+=']';return f};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>1){d=c[1]}else{return toString.call(a)};if(d=='Object'){try{return 'Object('+ JSON.stringify(a)+ ')'}catch(e){return 'Object'}};if(a instanceof Error){return `${a.name}: ${a.message}\n${a.stack}`};return d}function makeMutClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;const h=e.a;e.a=0;try{return d(h,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(h,e.b)}else{e.a=h}}};f.original=e;return f}function __wbg_adapter_24(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__h5095fcdaf8092370(a,b,addHeapObject(c))}function makeClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;try{return d(e.a,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(e.a,e.b);e.a=0}}};f.original=e;return f}function __wbg_adapter_27(a,b){wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h30ff94b245a0d0eb(a,b)}function __wbg_adapter_30(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__hd83b7884b8df7437(a,b,addHeapObject(c))}function __wbg_adapter_33(a,b){wasm.wasm_bindgen__convert__closures__invoke0_mut__hb4e43363b5febd42(a,b)}function __wbg_adapter_36(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__h0b386e962207feeb(a,b,addHeapObject(c))}function getCachedStringFromWasm0(a,b){if(a===0){return getObject(b)}else{return getStringFromWasm0(a,b)}}function handleError(a,b){try{return a.apply(this,b)}catch(c){wasm.__wbindgen_exn_store(addHeapObject(c))}}async function __wbg_load(a,b){if(typeof Response==='function'&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming==='function'){try{return await WebAssembly.instantiateStreaming(a,b)}catch(d){if(a.headers.get('Content-Type')!='application/wasm'){console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",d)}else{throw d}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}}function __wbg_get_imports(){const a={};a.wbg={};a.wbg.__wbindgen_is_undefined=function(b){const c=getObject(b)===undefined;return c};a.wbg.__wbindgen_string_get=function(b,c){const f=getObject(c);const g=typeof f==='string'?f:undefined;var d=isLikeNone(g)?0:passStringToWasm0(g,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbindgen_object_drop_ref=function(b){takeObject(b)};a.wbg.__wbindgen_object_clone_ref=function(b){const c=getObject(b);return addHeapObject(c)};a.wbg.__wbindgen_string_new=function(b,c){const d=getStringFromWasm0(b,c);return addHeapObject(d)};a.wbg.__wbindgen_cb_drop=function(b){const c=takeObject(b).original;if(c.cnt--==1){c.a=0;return true};const d=false;return d};a.wbg.__wbindgen_number_new=function(b){const c=b;return addHeapObject(c)};a.wbg.__wbindgen_boolean_get=function(b){const c=getObject(b);const d=typeof c==='boolean'?(c?1:0):2;return d};a.wbg.__wbindgen_jsval_eq=function(b,c){const d=getObject(b)===getObject(c);return d};a.wbg.__wbg_nodeId_bbf0efafa303e805=function(b,c){const d=getObject(c).$$$nodeId;getInt32Memory0()[b/4+ 1]=isLikeNone(d)?0:d;getInt32Memory0()[b/4+ 0]=!isLikeNone(d)};a.wbg.__wbg_setnodeId_433ef8ed15bd1612=function(b,c){getObject(b).$$$nodeId=c>>>0};a.wbg.__wbg_createTextNode_a7d5f5b956acda97=function(b,c){const d=getObject(b).createTextNode(c);return addHeapObject(d)};a.wbg.__wbg_setclassName_f86a95d6ffe042e6=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).className=e},arguments)};a.wbg.__wbg_queueMicrotask_b580a35152f7cc7c=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_newwitheventinitdict_4e9208842a1bd356=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new CustomEvent(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_documentElement_0df873a7503a9af9=function(b){const c=getObject(b).documentElement;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_body_674aec4c1c0910cd=function(b){const c=getObject(b).body;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_createComment_6b5ea2660a7c961a=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createComment(e);return addHeapObject(f)};a.wbg.__wbg_createElement_4891554b28d3388b=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createElement(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_createTextNode_2fd22cd7e543f938=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createTextNode(e);return addHeapObject(f)};a.wbg.__wbg_getElementById_cc0e0d931b0d9a28=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).getElementById(e);return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_querySelector_52ded52c20e23921=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelector(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_querySelectorAll_c03e8664a5a0f0c5=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_tagName_5d48e5a3ca410385=function(b,c){const d=getObject(c).tagName;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setid_1984ee27e5075311=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).id=e};a.wbg.__wbg_innerHTML_7957d4fb76221e5a=function(b,c){const d=getObject(c).innerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setinnerHTML_b089587252408b67=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).innerHTML=e};a.wbg.__wbg_outerHTML_f7749ceff37b5832=function(b,c){const d=getObject(c).outerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_closest_48a6f505535d402f=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).closest(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_getAttribute_3d8fcc9eaea35a17=function(b,c,d,e){var f=getCachedStringFromWasm0(d,e);const i=getObject(c).getAttribute(f);var g=isLikeNone(i)?0:passStringToWasm0(i,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var h=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=h;getInt32Memory0()[b/4+ 0]=g};a.wbg.__wbg_insertAdjacentHTML_04bc2b21165e1256=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).insertAdjacentHTML(g,h)},arguments)};a.wbg.__wbg_querySelectorAll_521f18edab19a2d0=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_setAttribute_e7e80b478b7b8b2f=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).setAttribute(g,h)},arguments)};a.wbg.__wbg_append_184b4e731e022bf0=function(){return handleError(function(b,c){getObject(b).append(getObject(c))},arguments)};a.wbg.__wbg_target_f171e89c61e2bccf=function(b){const c=getObject(b).target;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_preventDefault_24104f3f0a54546a=function(b){getObject(b).preventDefault()};a.wbg.__wbg_addEventListener_5651108fc3ffeb6e=function(){return handleError(function(b,c,d,e){var f=getCachedStringFromWasm0(c,d);getObject(b).addEventListener(f,getObject(e))},arguments)};a.wbg.__wbg_dispatchEvent_a622a6455be582eb=function(){return handleError(function(b,c){const d=getObject(b).dispatchEvent(getObject(c));return d},arguments)};a.wbg.__wbg_new_1eead62f64ca15ce=function(){return handleError(function(){const b=new Headers();return addHeapObject(b)},arguments)};a.wbg.__wbg_pushState_1145414a47c0b629=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).pushState(getObject(c),h,i)},arguments)};a.wbg.__wbg_replaceState_2e530b05e604adc4=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).replaceState(getObject(c),h,i)},arguments)};a.wbg.__wbg_rel_7a5daa1634871543=function(b,c){const d=getObject(c).rel;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_origin_8d3cc36142304e88=function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_pathname_28ec40e7c9ba7ea6=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_hash_1d8a5443664f6e12=function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_href_47b90f0ddf3ddcd7=function(b,c){const d=getObject(c).href;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_altKey_612289acf855835c=function(b){const c=getObject(b).altKey;return c};a.wbg.__wbg_ctrlKey_582686fb2263dd3c=function(b){const c=getObject(b).ctrlKey;return c};a.wbg.__wbg_shiftKey_48e8701355d8e2d4=function(b){const c=getObject(b).shiftKey;return c};a.wbg.__wbg_metaKey_43193b7cc99f8914=function(b){const c=getObject(b).metaKey;return c};a.wbg.__wbg_sethref_e5626365d7354fea=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).href=e},arguments)};a.wbg.__wbg_origin_50aa482fa6784a0a=function(){return handleError(function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_pathname_c8fd5c498079312d=function(){return handleError(function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_hash_a1a795b89dda8e3d=function(){return handleError(function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_sethash_b6135fe95fa0eebe=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).hash=e},arguments)};a.wbg.__wbg_language_4f943dbdc59c3951=function(b,c){const f=getObject(c).language;var d=isLikeNone(f)?0:passStringToWasm0(f,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbg_languages_4ab80469955a57f7=function(b){const c=getObject(b).languages;return addHeapObject(c)};a.wbg.__wbg_parentNode_9e53f8b17eb98c9d=function(b){const c=getObject(b).parentNode;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_nextSibling_304d9aac7c2774ae=function(b){const c=getObject(b).nextSibling;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_settextContent_28d80502cf08bde7=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).textContent=e};a.wbg.__wbg_appendChild_51339d4cde00ee22=function(){return handleError(function(b,c){const d=getObject(b).appendChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_insertBefore_ffa01d4b747c95fc=function(){return handleError(function(b,c,d){const e=getObject(b).insertBefore(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_removeChild_973429f368206138=function(){return handleError(function(b,c){const d=getObject(b).removeChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_replaceChild_3ec13b15218637aa=function(){return handleError(function(b,c,d){const e=getObject(b).replaceChild(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_length_7aeee1534dbcb390=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_get_c77649dd3862b63a=function(b,c){const d=getObject(b)[c>>>0];return isLikeNone(d)?0:addHeapObject(d)};a.wbg.__wbg_url_fda63503ced387ff=function(b,c){const d=getObject(c).url;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_newwithstr_3d9bc779603a93c7=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Request(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_newwithstrandinit_cad5cd6038c7ff5d=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new Request(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_instanceof_Response_fc4327dbfcdf5ced=function(b){let c;try{c=getObject(b) instanceof Response}catch{c=false};const d=c;return d};a.wbg.__wbg_status_ac85a3142a84caa2=function(b){const c=getObject(b).status;return c};a.wbg.__wbg_text_a667ac1770538491=function(){return handleError(function(b){const c=getObject(b).text();return addHeapObject(c)},arguments)};a.wbg.__wbg_pathname_57290e07c6bc0683=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_search_2ff3bb9114e0ca34=function(b,c){const d=getObject(c).search;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setsearch_16b87f04ea0e6b80=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).search=e};a.wbg.__wbg_new_a76f6bcb38f791ea=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new URL(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_new_2a98b9c4a51bdc04=function(){return handleError(function(){const b=new URLSearchParams();return addHeapObject(b)},arguments)};a.wbg.__wbg_instanceof_Window_9029196b662bc42a=function(b){let c;try{c=getObject(b) instanceof Window}catch{c=false};const d=c;return d};a.wbg.__wbg_document_f7ace2b956f30a4f=function(b){const c=getObject(b).document;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_location_56243dba507f472d=function(b){const c=getObject(b).location;return addHeapObject(c)};a.wbg.__wbg_history_3c2280e6b2a9316e=function(){return handleError(function(b){const c=getObject(b).history;return addHeapObject(c)},arguments)};a.wbg.__wbg_navigator_7c9103698acde322=function(b){const c=getObject(b).navigator;return addHeapObject(c)};a.wbg.__wbg_scrollTo_3fa406312438ebdf=function(b,c,d){getObject(b).scrollTo(c,d)};a.wbg.__wbg_get_cb7c1c2da725c920=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b)[e];return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_fetch_336b6f0cb426b46e=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_instanceof_WorkerGlobalScope_d9d741da0fb130ce=function(b){let c;try{c=getObject(b) instanceof WorkerGlobalScope}catch{c=false};const d=c;return d};a.wbg.__wbg_fetch_8eaf01857a5bb21f=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_log_1d3ae0273d8f4f8a=function(b){console.log(getObject(b))};a.wbg.__wbg_self_1ff1d729e9aae938=function(){return handleError(function(){const b=self.self;return addHeapObject(b)},arguments)};a.wbg.__wbg_window_5f4faef6c12b79ec=function(){return handleError(function(){const b=window.window;return addHeapObject(b)},arguments)};a.wbg.__wbg_globalThis_1d39714405582d3c=function(){return handleError(function(){const b=globalThis.globalThis;return addHeapObject(b)},arguments)};a.wbg.__wbg_global_651f05c6a0944d1c=function(){return handleError(function(){const b=global.global;return addHeapObject(b)},arguments)};a.wbg.__wbg_length_fff51ee6522a1a18=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_new_b51585de1b234aff=function(){const b=new Object();return addHeapObject(b)};a.wbg.__wbg_decodeURIComponent_3822776b09f2f835=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=decodeURIComponent(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_get_44be0491f933a435=function(b,c){const d=getObject(b)[c>>>0];return addHeapObject(d)};a.wbg.__wbg_instanceof_Error_ab19e20608ea43c7=function(b){let c;try{c=getObject(b) instanceof Error}catch{c=false};const d=c;return d};a.wbg.__wbg_message_48bacc5ea57d74ee=function(b){const c=getObject(b).message;return addHeapObject(c)};a.wbg.__wbg_name_8f734cbbd6194153=function(b){const c=getObject(b).name;return addHeapObject(c)};a.wbg.__wbg_toString_1c056108b87ba68b=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_newnoargs_581967eacc0e2604=function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Function(d);return addHeapObject(e)};a.wbg.__wbg_call_cb65541d95d71282=function(){return handleError(function(b,c){const d=getObject(b).call(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_getTimezoneOffset_8aee3445f323973e=function(b){const c=getObject(b).getTimezoneOffset();return c};a.wbg.__wbg_new_cd59bfc8881f487b=function(b){const c=new Date(getObject(b));return addHeapObject(c)};a.wbg.__wbg_is_205d914af04a8faa=function(b,c){const d=Object.is(getObject(b),getObject(c));return d};a.wbg.__wbg_toString_a8e343996af880e9=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_get_97b561fb56f034b5=function(){return handleError(function(b,c){const d=Reflect.get(getObject(b),getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_set_092e06b0f9d71865=function(){return handleError(function(b,c,d){const e=Reflect.set(getObject(b),getObject(c),getObject(d));return e},arguments)};a.wbg.__wbg_resolve_53698b95aaf7fcf8=function(b){const c=Promise.resolve(getObject(b));return addHeapObject(c)};a.wbg.__wbg_then_f7e06ee3c11698eb=function(b,c){const d=getObject(b).then(getObject(c));return addHeapObject(d)};a.wbg.__wbg_then_b2267541e2a73865=function(b,c,d){const e=getObject(b).then(getObject(c),getObject(d));return addHeapObject(e)};a.wbg.__wbindgen_debug_string=function(b,c){const d=debugString(getObject(c));const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbindgen_throw=function(b,c){throw new Error(getStringFromWasm0(b,c))};a.wbg.__wbindgen_closure_wrapper847=function(b,c,d){const e=makeMutClosure(b,c,256,__wbg_adapter_24);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1153=function(b,c,d){const e=makeClosure(b,c,296,__wbg_adapter_27);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1160=function(b,c,d){const e=makeMutClosure(b,c,296,__wbg_adapter_30);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper2035=function(b,c,d){const e=makeMutClosure(b,c,436,__wbg_adapter_33);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper3193=function(b,c,d){const e=makeMutClosure(b,c,461,__wbg_adapter_36);return addHeapObject(e)};return a}function __wbg_init_memory(a,b){}function __wbg_finalize_init(a,b){wasm=a.exports;__wbg_init.__wbindgen_wasm_module=b;cachedInt32Memory0=null;cachedUint8Memory0=null;wasm.__wbindgen_start();return wasm}function initSync(a){if(wasm!==undefined)return wasm;const b=__wbg_get_imports();__wbg_init_memory(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const c=new WebAssembly.Instance(a,b);return __wbg_finalize_init(c,a)}async function __wbg_init(a){if(wasm!==undefined)return wasm;if(typeof a==='undefined'){a=new URL('perseus_engine_bg.wasm',import.meta.url)};const b=__wbg_get_imports();if(typeof a==='string'||typeof Request==='function'&&a instanceof Request||typeof URL==='function'&&a instanceof URL){a=fetch(a)};__wbg_init_memory(b);const {instance:c,module:d}=await __wbg_load(await a,b);return __wbg_finalize_init(c,d)}export default __wbg_init;;