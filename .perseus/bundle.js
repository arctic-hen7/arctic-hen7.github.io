let wasm;const heap=new Array(128).fill(undefined);heap.push(undefined,null,true,false);function getObject(a){return heap[a]}let WASM_VECTOR_LEN=0;let cachedUint8Memory0=null;function getUint8Memory0(){if(cachedUint8Memory0===null||cachedUint8Memory0.byteLength===0){cachedUint8Memory0=new Uint8Array(wasm.memory.buffer)};return cachedUint8Memory0}const cachedTextEncoder=typeof TextEncoder!=='undefined'?new TextEncoder('utf-8'):{encode:()=>{throw Error('TextEncoder not available')}};const encodeString=typeof cachedTextEncoder.encodeInto==='function'?function(a,b){return cachedTextEncoder.encodeInto(a,b)}:function(a,b){const c=cachedTextEncoder.encode(a);b.set(c);return {read:a.length,written:c.length}};function passStringToWasm0(a,b,c){if(c===undefined){const h=cachedTextEncoder.encode(a);const i=b(h.length,1)>>>0;getUint8Memory0().subarray(i,i+ h.length).set(h);WASM_VECTOR_LEN=h.length;return i};let d=a.length;let e=b(d,1)>>>0;const f=getUint8Memory0();let g=0;for(;g<d;g++){const h=a.charCodeAt(g);if(h>0x7F)break;f[e+ g]=h};if(g!==d){if(g!==0){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,1)>>>0;const h=getUint8Memory0().subarray(e+ g,e+ d);const i=encodeString(a,h);g+=i.written};WASM_VECTOR_LEN=g;return e}function isLikeNone(a){return a===undefined||a===null}let cachedInt32Memory0=null;function getInt32Memory0(){if(cachedInt32Memory0===null||cachedInt32Memory0.byteLength===0){cachedInt32Memory0=new Int32Array(wasm.memory.buffer)};return cachedInt32Memory0}let heap_next=heap.length;function dropObject(a){if(a<132)return;heap[a]=heap_next;heap_next=a}function takeObject(a){const b=getObject(a);dropObject(a);return b}function addHeapObject(a){if(heap_next===heap.length)heap.push(heap.length+ 1);const b=heap_next;heap_next=heap[b];heap[b]=a;return b}const cachedTextDecoder=typeof TextDecoder!=='undefined'?new TextDecoder('utf-8',{ignoreBOM:true,fatal:true}):{decode:()=>{throw Error('TextDecoder not available')}};if(typeof TextDecoder!=='undefined'){cachedTextDecoder.decode()};function getStringFromWasm0(a,b){a=a>>>0;return cachedTextDecoder.decode(getUint8Memory0().subarray(a,a+ b))}function debugString(a){const b=typeof a;if(b=='number'||b=='boolean'||a==null){return `${a}`};if(b=='string'){return `"${a}"`};if(b=='symbol'){const e=a.description;if(e==null){return 'Symbol'}else{return `Symbol(${e})`}};if(b=='function'){const e=a.name;if(typeof e=='string'&&e.length>0){return `Function(${e})`}else{return 'Function'}};if(Array.isArray(a)){const e=a.length;let f='[';if(e>0){f+=debugString(a[0])};for(let g=1;g<e;g++){f+=', '+ debugString(a[g])};f+=']';return f};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>1){d=c[1]}else{return toString.call(a)};if(d=='Object'){try{return 'Object('+ JSON.stringify(a)+ ')'}catch(e){return 'Object'}};if(a instanceof Error){return `${a.name}: ${a.message}\n${a.stack}`};return d}function makeClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;try{return d(e.a,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(e.a,e.b);e.a=0}}};f.original=e;return f}function __wbg_adapter_26(a,b){wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfd793592f4608895(a,b)}function makeMutClosure(a,b,c,d){const e={a:a,b:b,cnt:1,dtor:c};const f=(...g)=>{e.cnt++;const h=e.a;e.a=0;try{return d(h,e.b,...g)}finally{if(--e.cnt===0){wasm.__wbindgen_export_2.get(e.dtor)(h,e.b)}else{e.a=h}}};f.original=e;return f}function __wbg_adapter_29(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__h819005e6763a12ee(a,b,addHeapObject(c))}function __wbg_adapter_32(a,b){wasm.wasm_bindgen__convert__closures__invoke0_mut__hc306b663a9b7a1fe(a,b)}function __wbg_adapter_35(a,b,c){wasm.wasm_bindgen__convert__closures__invoke1_mut__h3f355a2354438f2c(a,b,addHeapObject(c))}function getCachedStringFromWasm0(a,b){if(a===0){return getObject(b)}else{return getStringFromWasm0(a,b)}}function handleError(a,b){try{return a.apply(this,b)}catch(c){wasm.__wbindgen_exn_store(addHeapObject(c))}}async function __wbg_load(a,b){if(typeof Response==='function'&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming==='function'){try{return await WebAssembly.instantiateStreaming(a,b)}catch(d){if(a.headers.get('Content-Type')!='application/wasm'){console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",d)}else{throw d}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}}function __wbg_get_imports(){const a={};a.wbg={};a.wbg.__wbindgen_is_undefined=function(b){const c=getObject(b)===undefined;return c};a.wbg.__wbindgen_string_get=function(b,c){const f=getObject(c);const g=typeof f==='string'?f:undefined;var d=isLikeNone(g)?0:passStringToWasm0(g,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbindgen_object_drop_ref=function(b){takeObject(b)};a.wbg.__wbindgen_object_clone_ref=function(b){const c=getObject(b);return addHeapObject(c)};a.wbg.__wbindgen_string_new=function(b,c){const d=getStringFromWasm0(b,c);return addHeapObject(d)};a.wbg.__wbindgen_cb_drop=function(b){const c=takeObject(b).original;if(c.cnt--==1){c.a=0;return true};const d=false;return d};a.wbg.__wbindgen_number_new=function(b){const c=b;return addHeapObject(c)};a.wbg.__wbindgen_boolean_get=function(b){const c=getObject(b);const d=typeof c==='boolean'?(c?1:0):2;return d};a.wbg.__wbindgen_jsval_eq=function(b,c){const d=getObject(b)===getObject(c);return d};a.wbg.__wbg_nodeId_bbf0efafa303e805=function(b,c){const d=getObject(c).$$$nodeId;getInt32Memory0()[b/4+ 1]=isLikeNone(d)?0:d;getInt32Memory0()[b/4+ 0]=!isLikeNone(d)};a.wbg.__wbg_setnodeId_433ef8ed15bd1612=function(b,c){getObject(b).$$$nodeId=c>>>0};a.wbg.__wbg_createTextNode_a7d5f5b956acda97=function(b,c){const d=getObject(b).createTextNode(c);return addHeapObject(d)};a.wbg.__wbg_setclassName_f86a95d6ffe042e6=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).className=e},arguments)};a.wbg.__wbg_queueMicrotask_b580a35152f7cc7c=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_newwitheventinitdict_e2325c1670b25276=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new CustomEvent(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_documentElement_aab620794c1eb119=function(b){const c=getObject(b).documentElement;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_body_11da0c1aa9610cb3=function(b){const c=getObject(b).body;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_createComment_3b18d83b12cfbf48=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createComment(e);return addHeapObject(f)};a.wbg.__wbg_createElement_9ce3fdea8322ff34=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createElement(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_createTextNode_01a7250c5ca46b04=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).createTextNode(e);return addHeapObject(f)};a.wbg.__wbg_getElementById_328f8c4a5bb51ba8=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).getElementById(e);return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_querySelector_391afe271b8236d5=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelector(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_querySelectorAll_8f123e2ef4dc4a17=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_tagName_54808c4e28eacd30=function(b,c){const d=getObject(c).tagName;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setid_fe7d3f00faee9503=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).id=e};a.wbg.__wbg_innerHTML_6492d9db4a3fc77d=function(b,c){const d=getObject(c).innerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setinnerHTML_b88bf159b62c2334=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).innerHTML=e};a.wbg.__wbg_outerHTML_72dcf3aa34725f10=function(b,c){const d=getObject(c).outerHTML;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_closest_92a7847496001d92=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).closest(e);return isLikeNone(f)?0:addHeapObject(f)},arguments)};a.wbg.__wbg_getAttribute_a5ee18ad06ff0c94=function(b,c,d,e){var f=getCachedStringFromWasm0(d,e);const i=getObject(c).getAttribute(f);var g=isLikeNone(i)?0:passStringToWasm0(i,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var h=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=h;getInt32Memory0()[b/4+ 0]=g};a.wbg.__wbg_insertAdjacentHTML_04a1d091136d7cb6=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).insertAdjacentHTML(g,h)},arguments)};a.wbg.__wbg_querySelectorAll_56ac196deca3c4b1=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b).querySelectorAll(e);return addHeapObject(f)},arguments)};a.wbg.__wbg_setAttribute_aebcae2169f2f869=function(){return handleError(function(b,c,d,e,f){var g=getCachedStringFromWasm0(c,d);var h=getCachedStringFromWasm0(e,f);getObject(b).setAttribute(g,h)},arguments)};a.wbg.__wbg_append_66a5969d7f5d27f8=function(){return handleError(function(b,c){getObject(b).append(getObject(c))},arguments)};a.wbg.__wbg_target_6efb4504c149139f=function(b){const c=getObject(b).target;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_preventDefault_9299867e06da6909=function(b){getObject(b).preventDefault()};a.wbg.__wbg_addEventListener_0f2891b0794e07fa=function(){return handleError(function(b,c,d,e){var f=getCachedStringFromWasm0(c,d);getObject(b).addEventListener(f,getObject(e))},arguments)};a.wbg.__wbg_dispatchEvent_2434b822eb17a7b5=function(){return handleError(function(b,c){const d=getObject(b).dispatchEvent(getObject(c));return d},arguments)};a.wbg.__wbg_new_19676474aa414d62=function(){return handleError(function(){const b=new Headers();return addHeapObject(b)},arguments)};a.wbg.__wbg_pushState_8eaca41f86b13910=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).pushState(getObject(c),h,i)},arguments)};a.wbg.__wbg_replaceState_dba824356fa31972=function(){return handleError(function(b,c,d,e,f,g){var h=getCachedStringFromWasm0(d,e);var i=getCachedStringFromWasm0(f,g);getObject(b).replaceState(getObject(c),h,i)},arguments)};a.wbg.__wbg_rel_c0c246566e27b34a=function(b,c){const d=getObject(c).rel;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_origin_51586b7dd5d291b7=function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_pathname_2d9626e0ba95c39b=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_hash_a0ca2b42dd642ee2=function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_href_2777cc28ba3aac82=function(b,c){const d=getObject(c).href;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_altKey_f384daa388a44745=function(b){const c=getObject(b).altKey;return c};a.wbg.__wbg_ctrlKey_ac674c31f44bd157=function(b){const c=getObject(b).ctrlKey;return c};a.wbg.__wbg_shiftKey_a741da931809868b=function(b){const c=getObject(b).shiftKey;return c};a.wbg.__wbg_metaKey_d37dd650c2a748a7=function(b){const c=getObject(b).metaKey;return c};a.wbg.__wbg_sethref_2c377515f8ddd13a=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).href=e},arguments)};a.wbg.__wbg_origin_57ece1d4025136f7=function(){return handleError(function(b,c){const d=getObject(c).origin;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_pathname_cd5a90c8f3ab524a=function(){return handleError(function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_hash_ced9ee31706e591d=function(){return handleError(function(b,c){const d=getObject(c).hash;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e},arguments)};a.wbg.__wbg_sethash_765223ae5d42e0f5=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).hash=e},arguments)};a.wbg.__wbg_language_e2a23bf9f4b99268=function(b,c){const f=getObject(c).language;var d=isLikeNone(f)?0:passStringToWasm0(f,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var e=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=e;getInt32Memory0()[b/4+ 0]=d};a.wbg.__wbg_languages_b7fc8db4b1a47077=function(b){const c=getObject(b).languages;return addHeapObject(c)};a.wbg.__wbg_parentNode_e1c214fc3f362af0=function(b){const c=getObject(b).parentNode;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_nextSibling_d029031876ed1b1b=function(b){const c=getObject(b).nextSibling;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_settextContent_57c7c19d2b0e7614=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).textContent=e};a.wbg.__wbg_appendChild_2e6a6c9d1f0d443d=function(){return handleError(function(b,c){const d=getObject(b).appendChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_insertBefore_bdaeec8969497673=function(){return handleError(function(b,c,d){const e=getObject(b).insertBefore(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_removeChild_a63022ebbfa6ebf5=function(){return handleError(function(b,c){const d=getObject(b).removeChild(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_replaceChild_708a63be18c12f97=function(){return handleError(function(b,c,d){const e=getObject(b).replaceChild(getObject(c),getObject(d));return addHeapObject(e)},arguments)};a.wbg.__wbg_length_eae3bc233f10d60d=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_get_4eb9e7aa6c8c3dcc=function(b,c){const d=getObject(b)[c>>>0];return isLikeNone(d)?0:addHeapObject(d)};a.wbg.__wbg_url_2ec4d9660febcaa6=function(b,c){const d=getObject(c).url;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_newwithstr_5aa2429fd929c6e3=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Request(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_newwithstrandinit_29038da14d09e330=function(){return handleError(function(b,c,d){var e=getCachedStringFromWasm0(b,c);const f=new Request(e,getObject(d));return addHeapObject(f)},arguments)};a.wbg.__wbg_instanceof_Response_944e2745b5db71f5=function(b){let c;try{c=getObject(b) instanceof Response}catch(e){c=false};const d=c;return d};a.wbg.__wbg_status_7841bb47be2a8f16=function(b){const c=getObject(b).status;return c};a.wbg.__wbg_text_39a6fb98be736e16=function(){return handleError(function(b){const c=getObject(b).text();return addHeapObject(c)},arguments)};a.wbg.__wbg_pathname_a83d8f2ebefa6791=function(b,c){const d=getObject(c).pathname;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_search_8c5f74fa2d11377e=function(b,c){const d=getObject(c).search;const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbg_setsearch_a168105ad9dbdb8b=function(b,c,d){var e=getCachedStringFromWasm0(c,d);getObject(b).search=e};a.wbg.__wbg_new_d7cd05d9de7d4000=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new URL(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_new_c182c030c6c56588=function(){return handleError(function(){const b=new URLSearchParams();return addHeapObject(b)},arguments)};a.wbg.__wbg_instanceof_Window_cde2416cf5126a72=function(b){let c;try{c=getObject(b) instanceof Window}catch(e){c=false};const d=c;return d};a.wbg.__wbg_document_183cf1eecfdbffee=function(b){const c=getObject(b).document;return isLikeNone(c)?0:addHeapObject(c)};a.wbg.__wbg_location_61ca61017633c753=function(b){const c=getObject(b).location;return addHeapObject(c)};a.wbg.__wbg_history_56dc869560201113=function(){return handleError(function(b){const c=getObject(b).history;return addHeapObject(c)},arguments)};a.wbg.__wbg_navigator_7078da62d92ff5ad=function(b){const c=getObject(b).navigator;return addHeapObject(c)};a.wbg.__wbg_scrollTo_89258e9b1d4aee7d=function(b,c,d){getObject(b).scrollTo(c,d)};a.wbg.__wbg_get_70ad4d417836c1e8=function(b,c,d){var e=getCachedStringFromWasm0(c,d);const f=getObject(b)[e];return isLikeNone(f)?0:addHeapObject(f)};a.wbg.__wbg_fetch_8cebc656dc6b11b1=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_instanceof_WorkerGlobalScope_c8009eec4236245d=function(b){let c;try{c=getObject(b) instanceof WorkerGlobalScope}catch(e){c=false};const d=c;return d};a.wbg.__wbg_fetch_701fcd2bde06379a=function(b,c){const d=getObject(b).fetch(getObject(c));return addHeapObject(d)};a.wbg.__wbg_log_7811587c4c6d2844=function(b){console.log(getObject(b))};a.wbg.__wbg_queueMicrotask_2be8b97a81fe4d00=function(b){const c=getObject(b).queueMicrotask;return addHeapObject(c)};a.wbg.__wbindgen_is_function=function(b){const c=typeof getObject(b)==='function';return c};a.wbg.__wbg_queueMicrotask_e5949c35d772a669=function(b){queueMicrotask(getObject(b))};a.wbg.__wbg_self_3fad056edded10bd=function(){return handleError(function(){const b=self.self;return addHeapObject(b)},arguments)};a.wbg.__wbg_window_a4f46c98a61d4089=function(){return handleError(function(){const b=window.window;return addHeapObject(b)},arguments)};a.wbg.__wbg_globalThis_17eff828815f7d84=function(){return handleError(function(){const b=globalThis.globalThis;return addHeapObject(b)},arguments)};a.wbg.__wbg_global_46f939f6541643c5=function(){return handleError(function(){const b=global.global;return addHeapObject(b)},arguments)};a.wbg.__wbg_length_cace2e0b3ddc0502=function(b){const c=getObject(b).length;return c};a.wbg.__wbg_new_c728d68b8b34487e=function(){const b=new Object();return addHeapObject(b)};a.wbg.__wbg_decodeURIComponent_1ddf5a2890a9a11f=function(){return handleError(function(b,c){var d=getCachedStringFromWasm0(b,c);const e=decodeURIComponent(d);return addHeapObject(e)},arguments)};a.wbg.__wbg_get_4a9aa5157afeb382=function(b,c){const d=getObject(b)[c>>>0];return addHeapObject(d)};a.wbg.__wbg_instanceof_Error_9f5881c3c4149389=function(b){let c;try{c=getObject(b) instanceof Error}catch(e){c=false};const d=c;return d};a.wbg.__wbg_message_35f9b952e1b922e2=function(b){const c=getObject(b).message;return addHeapObject(c)};a.wbg.__wbg_name_e1152a59269f79e5=function(b){const c=getObject(b).name;return addHeapObject(c)};a.wbg.__wbg_toString_d0cefe4046ecb265=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_newnoargs_ccdcae30fd002262=function(b,c){var d=getCachedStringFromWasm0(b,c);const e=new Function(d);return addHeapObject(e)};a.wbg.__wbg_call_669127b9d730c650=function(){return handleError(function(b,c){const d=getObject(b).call(getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_getTimezoneOffset_5cb2b8973dd9f251=function(b){const c=getObject(b).getTimezoneOffset();return c};a.wbg.__wbg_new_a49511604c14761d=function(b){const c=new Date(getObject(b));return addHeapObject(c)};a.wbg.__wbg_is_c74aa9bb973d6109=function(b,c){const d=Object.is(getObject(b),getObject(c));return d};a.wbg.__wbg_toString_2c5d5b612e8bdd61=function(b){const c=getObject(b).toString();return addHeapObject(c)};a.wbg.__wbg_get_2aff440840bb6202=function(){return handleError(function(b,c){const d=Reflect.get(getObject(b),getObject(c));return addHeapObject(d)},arguments)};a.wbg.__wbg_set_40f7786a25a9cc7e=function(){return handleError(function(b,c,d){const e=Reflect.set(getObject(b),getObject(c),getObject(d));return e},arguments)};a.wbg.__wbg_resolve_a3252b2860f0a09e=function(b){const c=Promise.resolve(getObject(b));return addHeapObject(c)};a.wbg.__wbg_then_89e1c559530b85cf=function(b,c){const d=getObject(b).then(getObject(c));return addHeapObject(d)};a.wbg.__wbg_then_1bbc9edafd859b06=function(b,c,d){const e=getObject(b).then(getObject(c),getObject(d));return addHeapObject(e)};a.wbg.__wbindgen_debug_string=function(b,c){const d=debugString(getObject(c));const e=passStringToWasm0(d,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const f=WASM_VECTOR_LEN;getInt32Memory0()[b/4+ 1]=f;getInt32Memory0()[b/4+ 0]=e};a.wbg.__wbindgen_throw=function(b,c){throw new Error(getStringFromWasm0(b,c))};a.wbg.__wbindgen_closure_wrapper1143=function(b,c,d){const e=makeClosure(b,c,292,__wbg_adapter_26);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1151=function(b,c,d){const e=makeMutClosure(b,c,292,__wbg_adapter_29);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper1997=function(b,c,d){const e=makeMutClosure(b,c,427,__wbg_adapter_32);return addHeapObject(e)};a.wbg.__wbindgen_closure_wrapper3153=function(b,c,d){const e=makeMutClosure(b,c,452,__wbg_adapter_35);return addHeapObject(e)};return a}function __wbg_init_memory(a,b){}function __wbg_finalize_init(a,b){wasm=a.exports;__wbg_init.__wbindgen_wasm_module=b;cachedInt32Memory0=null;cachedUint8Memory0=null;wasm.__wbindgen_start();return wasm}function initSync(a){if(wasm!==undefined)return wasm;const b=__wbg_get_imports();__wbg_init_memory(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const c=new WebAssembly.Instance(a,b);return __wbg_finalize_init(c,a)}async function __wbg_init(a){if(wasm!==undefined)return wasm;if(typeof a==='undefined'){a=new URL('perseus_engine_bg.wasm',import.meta.url)};const b=__wbg_get_imports();if(typeof a==='string'||typeof Request==='function'&&a instanceof Request||typeof URL==='function'&&a instanceof URL){a=fetch(a)};__wbg_init_memory(b);const {instance:c,module:d}=await __wbg_load(await a,b);return __wbg_finalize_init(c,d)}export default __wbg_init;;