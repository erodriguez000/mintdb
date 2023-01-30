import Component from "../../../../lib/component.js";
import mintDB from "../../../../lib/mint.js";
import store from "../../../../store/index.js";
import KVContainer from "./KVContainer.js";

export default class DocumentDetail extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("doc-detail"),
            subscriptions: []
        })
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="document-details-header">
                <div class="kv-add-group">
                    <input class="kv-input" id="key" placeholder="key">
                    <input class="kv-input" id="value" placeholder="value">
                    <button class="add-table-btn" id="add-kv-btn">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <line x1="12" y1="5" x2="12" y2="19"></line>
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                        </svg>
                    </button>
                </div>
            </div>
            <div class="kv-input-container" id="kv-input-container"></div>
        `;
        const kvContainer = new KVContainer();
        kvContainer.render();

        document.querySelector("#add-kv-btn").addEventListener("click", async () => {
            const keyEl = document.querySelector("#key");
            const valueEl = document.querySelector("#value");
            const payload = { [keyEl.value]: valueEl.value };
            
            const activeDocument = await mintDB.merge(store.state.activeTable, store.state.activeDocument.id, payload);
            const documentList = await mintDB.getAll(store.state.activeTable);
            
            store.dispatch('setActiveDocument', { activeDocument });
            store.dispatch('setDocuments', documentList);
            
            keyEl.value = "";
            valueEl.value = "";
        });
    }
}