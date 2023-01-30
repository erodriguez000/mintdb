import Component from "../../../lib/component.js";
import store from "../../../store/index.js";

export default class InfoView extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("content"),
            subscriptions: []
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="user-table-container">
                <h1>Info View</h1>
            </div>
        `;
    }
}