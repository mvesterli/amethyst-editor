const { ipcRenderer } = require('electron');

let app = new Vue({
    el: '#app',
    data: {
        process: process,
        entities: [],
        components: [],
        rawComponents: null,
        selectedEntity: null,
    },
    methods: {
        selectEntity: function(entity) {
            console.log('selected entity', entity);
            this.selectedEntity = entity;
        }
    }
});
exports.app = app;

ipcRenderer.on('message', (event, data) => {
    app.entities = data.entities;
    app.rawComponents = data.components;

    // Convert the list of [key, value] pairs into maps that we can use to
    // quickly lookup which components are associated with which entities.
    //
    // TODO: Could we have Rust send the data directly in the format we want?
    let components = [];
    for (let [type, flatData] of Object.entries(data.components)) {
        let lookup = {};
        for (let [entity, componentData] of flatData) {
            lookup[entity] = componentData;
        }
        components.push({
            type: type,
            data: lookup,
        });
    }
    app.components = components;
});
