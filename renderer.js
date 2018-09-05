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

    // Sort components before updating the local data to ensure that components always appear
    // in the same order regardless of the order they are sent in.
    var sortedComponents = data.components;
    sortedComponents.sort(compareNamed);
    app.components = sortedComponents;

    // Sort resources before updating the local data to ensure that resources always appear
    // in the same order regardless of the order they are sent in.
    var sortedResources = data.resources;
    sortedResources.sort(compareNamed);
    app.resources = sortedResources;
});

function compareNamed(left, right) {
    if (left.name < right.name) { return -1; }
    if (left.name > right.name) { return 1; }
    return 0;
}
