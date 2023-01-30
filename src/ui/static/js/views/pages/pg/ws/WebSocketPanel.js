import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";
import WebSocketInput from "./WebSocketInput.js";
import WebSocketResponse from "./WebSocketResponse.js";
import WSSubscriptionList from "./WSSubscriptionList.js";

export default class WebSocketPanel extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("ws-panel"),
            subscriptions: []
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <div class="ws-input-view">
                <div class="connect-container" id="connect-container"></div>
                <div class="subscriptions-container">
                    <div class="sub-group" id="sub-list"></div>
                </div>
            </div>
            <div class="ws-response" id="ws-response"></div>
        `;
        
        const wsInput = new WebSocketInput();
        wsInput.render();
        
        const wsSubsriptionList = new WSSubscriptionList();
        wsSubsriptionList.render();
        
        const wsResponse = new WebSocketResponse();
        wsResponse.render();
        
    }
}