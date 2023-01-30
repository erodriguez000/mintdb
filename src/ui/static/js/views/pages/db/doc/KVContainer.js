import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";

export default class KVContainer extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("kv-input-container"),
            subscriptions: ["activeDocument"]
        })
    }
    render() {
        this.element.innerHTML = /*html*/`
        ${Object.entries(store.state.activeDocument).map(([key, value]) => {
            return /*html*/`
                        <div class="kv-input-group">
                            <input 
                              class="kv-input" 
                              id="key-${key}"
                              value="${key}"
                            >
                            <input 
                                class="kv-input" 
                                id="value-${key}"
                                value="${value}"    
                            >
                            <button class="add-table-btn" id="${key}">
                                <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <line x1="18" y1="6" x2="6" y2="18"></line>
                                    <line x1="6" y1="6" x2="18" y2="18"></line>
                                </svg>
                            </button>
                        </div>
                    `}).join('')}
            `;
            const kvPairs = document.querySelectorAll(".kv-input-group");
            kvPairs.forEach(kv => kv
                .querySelectorAll("button")
                .forEach(btn => btn.addEventListener("click", () => {
                    const payload = btn.id;
                    console.log(payload);
                    store.dispatch("deleteKeyValue", btn.id);
            })));
    }
}