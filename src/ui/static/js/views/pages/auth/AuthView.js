import Component from "../../../lib/component.js";
import store from "../../../store/index.js";
import AuthTable from "./tb/AuthTable.js";

export default class AuthView extends Component {
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
                <table id="auth-table"></table>
            </div>
        `;

        const authTable = new AuthTable();
        await authTable.render();
    }
}