var addon = require('./addon.node');
console.log(addon.hello('test'));
addon.add_slow(1, 2, function(data) {
    console.log("result: ", data);
});
console.log('wait add_slow...');

var obj = new addon.make_wrap(1234);
console.log('obj.native.n:', addon.get_wrap(obj));