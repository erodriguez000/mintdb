import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";

export default class SQLResponse extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("sql-response"),
            subscriptions: ["sqlResponse"]
        });
    }
    render() {
        this.element.innerHTML = /*html*/`
            <p>SQL Response</p>
            <p>${store.state.sqlResponse}</p>
        `;
    }
}