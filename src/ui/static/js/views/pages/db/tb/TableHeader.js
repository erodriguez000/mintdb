import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";

export default class TableHeader extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("tb-header"),
            subscriptions: ["activeTable"]
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="table-header-text-container">
                <h1 id="table-header-text">${store.state.activeTable ?? ""}</h1>   
            </div>
        `;
    }
}