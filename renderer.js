const { ipcRenderer } = require('electron');

exports.app = new Vue({
    el: '#app',
    data: {
        process: process,
        entities: [],
    }
});

ipcRenderer.on('message', (event, data) => {
    console.log('Recieved super cool event', event, data);
    exports.app.entities = data;
});
