
var path = require('path')
var ffi = require('ffi');

var lib = ffi.Library(path.join(__dirname, '../../target/debug/metropolis'), {
  fibonacci: ['int', ['int']]
});

var num = lib.fibonacci(20);
console.log(num)
