!function(e){function t(t){for(var n,o,i=t[0],a=t[1],c=0,u=[];c<i.length;c++)o=i[c],Object.prototype.hasOwnProperty.call(r,o)&&r[o]&&u.push(r[o][0]),r[o]=0;for(n in a)Object.prototype.hasOwnProperty.call(a,n)&&(e[n]=a[n]);for(f&&f(t);u.length;)u.shift()()}var n={},r={0:0};var o={};var i={3:function(){return{"./index_bg.js":{__wbindgen_object_drop_ref:function(e){return n[1].exports.l(e)},__wbg_log_3e243e25bb2293ad:function(e,t){return n[1].exports.f(e,t)},__wbg_canvas_56042b08dcda9b73:function(e){return n[1].exports.a(e)},__wbg_getImageData_0fedc3bf39be0d04:function(e,t,r,o,i){return n[1].exports.c(e,t,r,o,i)},__wbg_putImageData_14966f8782ccf3de:function(e,t,r,o){return n[1].exports.h(e,t,r,o)},__wbg_width_56afa08c5d6a4ccc:function(e){return n[1].exports.i(e)},__wbg_height_8f8f429d977ce11a:function(e){return n[1].exports.d(e)},__wbg_width_c4082e5dae323d9b:function(e){return n[1].exports.j(e)},__wbg_height_d31dc3acc4ef1e60:function(e){return n[1].exports.e(e)},__wbg_data_b15144b16dbbb0ce:function(e,t){return n[1].exports.b(e,t)},__wbg_newwithu8clampedarrayandsh_d46fa191b076edfe:function(e,t,r,o){return n[1].exports.g(e,t,r,o)},__wbindgen_debug_string:function(e,t){return n[1].exports.k(e,t)},__wbindgen_throw:function(e,t){return n[1].exports.m(e,t)}}}}};function a(t){if(n[t])return n[t].exports;var r=n[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,a),r.l=!0,r.exports}a.e=function(e){var t=[],n=r[e];if(0!==n)if(n)t.push(n[2]);else{var c=new Promise((function(t,o){n=r[e]=[t,o]}));t.push(n[2]=c);var u,s=document.createElement("script");s.charset="utf-8",s.timeout=120,a.nc&&s.setAttribute("nonce",a.nc),s.src=function(e){return a.p+""+e+".index.js"}(e);var f=new Error;u=function(t){s.onerror=s.onload=null,clearTimeout(d);var n=r[e];if(0!==n){if(n){var o=t&&("load"===t.type?"missing":t.type),i=t&&t.target&&t.target.src;f.message="Loading chunk "+e+" failed.\n("+o+": "+i+")",f.name="ChunkLoadError",f.type=o,f.request=i,n[1](f)}r[e]=void 0}};var d=setTimeout((function(){u({type:"timeout",target:s})}),12e4);s.onerror=s.onload=u,document.head.appendChild(s)}return({2:[3]}[e]||[]).forEach((function(e){var n=o[e];if(n)t.push(n);else{var r,c=i[e](),u=fetch(a.p+""+{3:"20795e51fda0a1c4c0ce"}[e]+".module.wasm");if(c instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(u),c]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(u,c);else{r=u.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,c)}))}t.push(o[e]=r.then((function(t){return a.w[e]=(t.instance||t).exports})))}})),Promise.all(t)},a.m=e,a.c=n,a.d=function(e,t,n){a.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},a.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},a.t=function(e,t){if(1&t&&(e=a(e)),8&t)return e;if(4&t&&"object"==typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(a.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)a.d(n,r,function(t){return e[t]}.bind(null,r));return n},a.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return a.d(t,"a",t),t},a.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},a.p="",a.oe=function(e){throw console.error(e),e},a.w={};var c=window.webpackJsonp=window.webpackJsonp||[],u=c.push.bind(c);c.push=t,c=c.slice();for(var s=0;s<c.length;s++)t(c[s]);var f=u;a(a.s=0)}([function(e,t,n){"use strict";n.r(t);var r=n.p+"d299381c74762ce528338b861337b457.jpg";const o=Promise.all([n.e(1),n.e(2)]).then(n.bind(null,7));let i;o.then(e=>{i=e});const a=document.getElementById("canvas"),c=a.getContext("2d"),u=document.getElementById("webcamVideo");let s=!1;const f=document.getElementById("testImage");f.src=r;const d=new Promise(e=>f.addEventListener("load",e)),l=async e=>{if(s=!1,await o,"image"===e)await d,c.drawImage(f,0,0,a.width,a.height),i.apply_filter(c);else if("webcam"===e){if(!navigator.mediaDevices.getUserMedia)return;const e=await navigator.mediaDevices.getUserMedia({video:!0});u.srcObject=e;const t=()=>{s&&(c.drawImage(u,0,0,a.width,a.height),i.apply_filter(c),requestAnimationFrame(t))};s=!0,requestAnimationFrame(t)}},p=document.getElementById("selectInput");l("image"),p.addEventListener("change",e=>l(e.target.value))}]);