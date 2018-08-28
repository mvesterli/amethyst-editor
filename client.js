const ipc = require('node-ipc');

console.log("Hello from the server process~~!");

ipc.config.id   = 'hello';
ipc.config.retry= 1500;

ipc.serveNet(
    8001,
    'udp4',
    function(){
        ipc.server.on(
            'message',
            function(data){
                ipc.log('got Data');
                ipc.log('got a message from '.debug, data.from.variable ,' : '.debug, data.message.variable);
            }
        );
        ipc.server.emit(
            {
                address : '127.0.0.1', //any hostname will work
                port    : ipc.config.networkPort
            },
            'message',
            {
                from    : ipc.config.id,
                message : 'Hello'
            }
        );
    }
);

ipc.server.start();
