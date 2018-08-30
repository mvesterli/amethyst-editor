const { ipcRenderer } = require('electron');

exports.app = new Vue({
    el: '#app',
    data: {
        process: process,
        entities: [],
        transforms: {},
    }
});

ipcRenderer.on('message', (event, data) => {
    exports.app.entities = data.entities;

    let transforms = {};
    for (let [entityId, transform] of data.transforms) {
        transforms['' + entityId] = transform;
    }
    exports.app.transforms = transforms;
});
