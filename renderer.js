const { ipcRenderer } = require('electron');

let app = new Vue({
    el: '#app',
    data: {
        process: process,
        entities: [],
        components: [],
        resources: [],
        rawComponents: null,
        selectedEntity: null,
    },
    methods: {
        selectEntity: function(entity) {
            this.selectedEntity = entity;
        }
    }
});
exports.app = app;

ipcRenderer.on('message', (event, data) => {
    app.entities = data.entities;
    app.components = data.components;
    app.resources = data.resources;
});
