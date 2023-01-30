import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";

export default class WebSocketResponse extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("ws-response"),
            subscriptions: ["socketConnected", "wsResponse"]
        });
    }
    render() {
        this.element.innerHTML = /*html*/`
            <p style="color: ${store.state.socketConnected ? "aquamarine" : "indianred"}">
                WebSocket 
                ${store.state.socketConnected ? "Connected" : "Disconnected"}
            </p>
            <p>${store.state.wsResponse}</p>
        `;
    }
}