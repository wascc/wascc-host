module.exports=function(t){var e={};function r(n){if(e[n])return e[n].exports;var o=e[n]={i:n,l:!1,exports:{}};return t[n].call(o.exports,o,o.exports,r),o.l=!0,o.exports}return r.m=t,r.c=e,r.d=function(t,e,n){r.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:n})},r.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},r.t=function(t,e){if(1&e&&(t=r(t)),8&e)return t;if(4&e&&"object"==typeof t&&t&&t.__esModule)return t;var n=Object.create(null);if(r.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var o in t)r.d(n,o,function(e){return t[e]}.bind(null,o));return n},r.n=function(t){var e=t&&t.__esModule?function(){return t.default}:function(){return t};return r.d(e,"a",e),e},r.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},r.p="/",r(r.s="QfWi")}({HteQ:function(t,e){t.exports=require("preact")},MV5A:function(t,e,r){},QfWi:function(t,e,r){"use strict";r.r(e),function(t){r.d(e,"default",(function(){return o}));r("MV5A");var n=r("HteQ");var o=function(e){var r,n;function o(t){var r;return(r=e.call(this,t)||this).counterRequest=function(){fetch("http://localhost:8080/counter1").then((function(t){return t.json()})).then((function(t){return r.setState({counter:t.counter,tweaked:t.tweaked,payloadRequested:!0,payload:t})})).catch((function(t){return r.setState({payload:{error:"error making request"}})}))},r.state={payload:null,error:""},r}return n=e,(r=o).prototype=Object.create(n.prototype),r.prototype.constructor=r,r.__proto__=n,o.prototype.render=function(){return t("div",{className:"main"},t("img",{src:"https://miro.medium.com/max/400/1*-wSVG5Qyg80Fu2bt6Tvs2w.png",style:{height:"350px",marginTop:"-25px",marginBottom:"-50px"},onClick:this.counterRequest}),t("h1",null,"waSCC Counter Demo"),t("div",{className:"request"},t("a",{onClick:this.counterRequest,"data-title":"Count!"}),this.state.payload?t("div",{className:"payload"},JSON.stringify(this.state.payload).replace(",",",\n  ").replace("{","{\n  ").replace("}","\n}")):t("div",{className:"payload",style:{color:"white"}},"secret",t("br",null))),t("br",null),t("h2",null,"No boilerplate!"),t("img",{src:"https://i.imgur.com/w2QWiQL.png"}))},o}(n.Component)}.call(this,r("HteQ").h)}});
//# sourceMappingURL=ssr-bundle.js.map