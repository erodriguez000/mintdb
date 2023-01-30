import Component from "../../../../lib/component.js";
import store from "../../../../store/index.js";
import mintDB from "../../../../lib/mint.js";

export default class WebSocketInput extends Component {
    constructor() {
        super({
            store,
            element: document.getElementById("connect-container"),
            subscriptions: ["socketConnected"]
        });
    }

    async render() {
        this.element.innerHTML = /*html*/`
            <button class="connect-btn" id="connect-btn">${store.state.socketConnected ? "close" : "connect"}</button>
            <div class="add-sub-container ${store.state.socketConnected ? "" : "closed"}">
                <input type="text" id="add-sub-input" placeholder="add a subscription">
                <button class="add-sub-btn" id="add-sub-btn">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <line x1="12" y1="5" x2="12" y2="19"></line>
                        <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                </button>
            </div>
        `;
        
        document.querySelector("#connect-btn").addEventListener("click", async () => {
            if(store.state.socketConnected) {
                mintDB.subscriptions = [];
                store.dispatch("toggleSocket", { socketConnected: false });
                store.dispatch("setSubscriptions", []);
                mintDB.closeWS();
                
            } else {
                store.dispatch("toggleSocket", { socketConnected: true });
                await mintDB.listenOn(store.state.subscriptions, (msg) => {
                    store.dispatch("setWebsocketResponse", msg.data);
                });
            }
        });

        document.querySelector("#add-sub-btn").addEventListener("click", () => {
            const inputEl = document.querySelector("#add-sub-input");
            mintDB.addSubscription(inputEl.value);
            const payload = mintDB.subscriptions;
            store.dispatch("setSubscriptions", payload);
            inputEl.value = "";
        });
    }
}