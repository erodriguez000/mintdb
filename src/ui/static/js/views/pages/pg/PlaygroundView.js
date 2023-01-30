import Component from "../../../lib/component.js";
import store from "../../../store/index.js";
import SQLPanel from "./sql/SQLPanel.js";
import WebSocketPanel from "./ws/WebSocketPanel.js";

export default class PlaygroundView extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("content")
        })
    }
    async render() {
        this.element.innerHTML = /*html*/`
            <div class="pg-ws-input-panel" id="ws-panel"></div>
            <div class="pg-sql-input-panel" id="sql-panel"></div>
        `;
        
        const sqlPanel = new SQLPanel();
        await sqlPanel.render();

        const wsPanel = new WebSocketPanel();
        await wsPanel.render();
    }
}